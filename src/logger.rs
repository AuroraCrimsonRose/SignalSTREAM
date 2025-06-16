use std::fs;
use tracing::info;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

/// Holds logger guard handles so log files stay open
pub struct LoggerGuards {
    pub system_guard: WorkerGuard,
    pub error_guard: WorkerGuard,
}

/// Initializes the global logger
pub fn init_logging() -> LoggerGuards {
    let _ = fs::create_dir_all("logs");

    let system_file = tracing_appender::rolling::never("logs", "system.log");
    let (system_writer, system_guard) = tracing_appender::non_blocking(system_file);

    let error_file = tracing_appender::rolling::never("logs", "error.log");
    let (error_writer, error_guard) = tracing_appender::non_blocking(error_file);

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(system_writer)
                .with_ansi(true)
                .with_filter(
                    EnvFilter::from_default_env()
                        .add_directive("signalstream=info".parse().unwrap()),
                ),
        )
        .with(
            fmt::layer()
                .with_writer(error_writer)
                .with_ansi(false)
                .with_filter(EnvFilter::new("error")),
        )
        .init();

    info!("Logger initialized.");

    LoggerGuards {
        system_guard,
        error_guard,
    }
}

/// Per-station logger struct — keeps log file open and scoped
pub struct StationLogger {
    pub id: String,
    pub guard: WorkerGuard,
}

impl StationLogger {
    pub fn new(id: &str) -> Self {
        let path = format!("station-{}.log", id);
        let file = tracing_appender::rolling::never("logs", &path);
        let (writer, guard) = tracing_appender::non_blocking(file);

        let station_layer = tracing_subscriber::fmt::layer()
            .with_writer(writer)
            .with_ansi(false)
            .with_target(false)
            .with_filter(EnvFilter::new("info"));

        // Use Dispatch to avoid overwriting global subscriber
        let dispatch = tracing::Dispatch::new(tracing_subscriber::registry().with(station_layer));

        tracing::dispatcher::with_default(&dispatch, || {
            tracing::info!("Station logger [{}] initialized", id);
        });

        StationLogger {
            id: id.to_string(),
            guard,
        }
    }
}
