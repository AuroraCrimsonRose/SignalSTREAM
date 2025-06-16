use crate::config::StationConfig;
use crate::station::manager::StationManager;

use std::fs;
use std::path::Path;

pub fn load_all_stations<P: AsRef<Path>>(
    dir: P,
) -> Result<Vec<StationManager>, Box<dyn std::error::Error>> {
    let mut managers = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let config = StationConfig::from_file(entry.path())?;
            managers.push(StationManager::new(config));
        }
    }
    Ok(managers)
}
