mod config;
mod logger;
mod route;
mod server;
mod station;

use crate::station::load_all_stations;
use server::http::serve_all_routes;

#[tokio::main]
async fn main() {
    let _log_guards = logger::init_logging();
    tracing::info!("SignalSTREAM starting up...");

    let stations = load_all_stations("stations").expect("Failed to load stations");

    if stations.is_empty() {
        tracing::error!("No valid station configs found in /stations/");
        return;
    }

    // Start all station managers
    for manager in &stations {
        manager.start();
        tracing::info!("Started station: {}", manager.config.id);
    }

    // Start unified HTTP server for streams + status
    serve_all_routes(stations).await;
}
