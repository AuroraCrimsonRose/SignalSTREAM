use std::{fs, path::PathBuf, process::Command, sync::Arc, thread};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use tokio::task;
use tracing::{error, info};

use crate::config::StationConfig;
use crate::route::StreamRouter;

pub struct StationManager {
    pub config: StationConfig,
    pub tx: broadcast::Sender<Vec<u8>>,
    pub router: Arc<StreamRouter>,
}

impl StationManager {
    pub fn new(config: StationConfig) -> Self {
        // 1) Create a broadcast channel buffered at 1024 frames
        let (tx, _) = broadcast::channel(1024);

        // 2) Construct your router from only the config
        let router = Arc::new(StreamRouter::new(config.clone()));

        StationManager { config, tx, router }
    }

    pub fn start(&self) {
        let router_clone = Arc::clone(&self.router);
        task::spawn_blocking(move || {
            router_clone.start();
        });

        // —— B) Spawn FFmpeg to package DASH ——
        let station_id = &self.config.id;
        let out_dir = PathBuf::from("public").join("dash").join(station_id);
        if let Err(e) = fs::create_dir_all(&out_dir) {
            error!("Failed to create DASH dir {:?}: {}", out_dir, e);
            return;
        }

        // Input = your live MP3 endpoint
        let input_url = format!("http://localhost:9090/{}.mp3", station_id);
        let mpd_path = out_dir.join("manifest.mpd");

        let ffmpeg_args = [
            "-i",
            &format!("http://127.0.0.1:9090/{}.mp3", self.config.id),
            "-map",
            "0:a",
            "-c:a",
            "aac",
            "-f",
            "dash",
            "-seg_duration",
            "2",
            "-use_timeline",
            "1",
            "-use_template",
            "1",
            &format!("{}/manifest.mpd", out_dir.display()),
        ];

        match Command::new("ffmpeg").args(&ffmpeg_args).spawn() {
            Ok(child) => info!(
                "Spawned DASH packager for station '{}', PID={}",
                station_id,
                child.id()
            ),
            Err(e) => error!(
                "Could not spawn FFmpeg DASH packager for '{}': {}",
                station_id, e
            ),
        }
    }

    /// Loads all station configs from the given directory and returns a Vec of StationManager.
    pub fn load_all<P: AsRef<std::path::Path>>(
        dir: P,
    ) -> Result<Vec<StationManager>, Box<dyn std::error::Error>> {
        let mut managers = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let config = StationConfig::from_file(entry.path())?;
                managers.push(StationManager::new(config));
            }
        }
        Ok(managers)
    }
}

pub fn start_streaming(playlist: Vec<String>, tx: Sender<Vec<u8>>) {
    tokio::spawn(async move {
        loop {
            for track in &playlist {
                let mut file = match File::open(track).await {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Failed to open {}: {}", track, e);
                        continue;
                    }
                };
                let mut buf = [0u8; 4096];
                loop {
                    let n = match file.read(&mut buf).await {
                        Ok(0) => break, // EOF
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Read error: {}", e);
                            break;
                        }
                    };
                    // Ignore send errors (no listeners)
                    let _ = tx.send(buf[..n].to_vec());
                }
            }
        }
    });
}
