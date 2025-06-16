use std::fs::{File, read_to_string};
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use tokio::sync::broadcast::Sender;
use tracing::{info, error};

/// Playlist playback engine for a station
pub struct PlaylistPlayer {
    pub playlist: Vec<PathBuf>,
    pub loop_enabled: bool,
    pub tx: Sender<Vec<u8>>,
    pub now_playing: Arc<RwLock<Option<String>>>,
}

impl PlaylistPlayer {
    /// Load a playlist from a `.m3u` file or a folder of `.mp3`s
    pub fn load(
        path: &str,
        tx: Sender<Vec<u8>>,
        now_playing: Arc<RwLock<Option<String>>>,
    ) -> Self {
        let mut playlist = Vec::new();
        let source = PathBuf::from(path);

        if source.is_dir() {
            // Load all .mp3s in the folder
            if let Ok(entries) = source.read_dir() {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|ext| ext == "mp3").unwrap_or(false) {
                        playlist.push(path);
                    }
                }
            }
        } else if source.is_file() && source.extension().map(|e| e == "m3u").unwrap_or(false) {
            // Load from M3U playlist
            if let Ok(lines) = read_to_string(&source) {
                for line in lines.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    playlist.push(PathBuf::from(trimmed));
                }
            }
        }

        info!("Playlist loaded: {} tracks from {}", playlist.len(), path);

        PlaylistPlayer {
            playlist,
            loop_enabled: true,
            tx,
            now_playing,
        }
    }

    /// Begin streaming playlist files in a loop
    pub async fn play(&self) {
        if self.playlist.is_empty() {
            error!("Playlist is empty — nothing to stream.");
            return;
        }

        loop {
            for path in &self.playlist {
                if let Err(e) = self.play_file(path).await {
                    error!("Error playing {}: {}", path.display(), e);
                }
            }

            if !self.loop_enabled {
                break;
            }
        }
    }

    async fn play_file(&self, path: &PathBuf) -> std::io::Result<()> {
        // Update now-playing metadata
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        {
            let mut np = self.now_playing.write().unwrap();
            *np = Some(name.clone());
        }

        info!("Now playing: {}", name);

        let mut file = File::open(path)?;
        let mut buffer = [0u8; 4096];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }

            let _ = self.tx.send(buffer[..n].to_vec());
            tokio::time::sleep(tokio::time::Duration::from_millis(25)).await;
        }

        Ok(())
    }
}
