use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

/// Station configuration parsed from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationConfig {
    pub id: String,
    pub mount: String,
    pub playlist: Vec<String>, // <-- change to Vec<String>
    pub fallback: String,
    pub enable_live: bool,
    pub crossfade: f32,
    // add more fields as needed
}

impl StationConfig {
    /// Loads a StationConfig from a file path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let config: StationConfig = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
