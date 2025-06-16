pub mod playlist;

use crate::config::StationConfig;
use playlist::PlaylistPlayer;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast::{self, Sender};
use tracing::{error, info};

#[derive(Clone)]
pub struct StreamRouter {
    pub config: StationConfig,
    pub tx: Sender<Vec<u8>>,
    pub now_playing: Arc<RwLock<Option<String>>>, // 👈 new
}

impl StreamRouter {
    pub fn new(config: StationConfig) -> Self {
        let (tx, _) = broadcast::channel(32);
        StreamRouter {
            config,
            tx,
            now_playing: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn start(&self) {
        let player = PlaylistPlayer::load(
            self.config
                .playlist
                .get(0)
                .map(|s| s.as_str())
                .unwrap_or(""),
            self.tx.clone(),
            self.now_playing.clone(), // 👈 this is the missing 3rd arg
        );
        player.play().await;
    }
}
