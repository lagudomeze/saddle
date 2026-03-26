use crate::SaddleResult;
use exn::ResultExt;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use super::Settings;

pub fn init_logging(settings: &Settings) -> SaddleResult<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&settings.logging.level));

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer);

    if let Some(log_file) = &settings.logging.file {
        let log_dir = std::path::Path::new(log_file)
            .parent()
            .map(|p| p.to_path_buf());

        if let Some(dir) = log_dir {
            std::fs::create_dir_all(&dir)
                .or_raise(|| crate::SaddleError::Logging(format!("Failed to create log directory: {:?}", dir)))?;
        }

        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)
            .or_raise(|| crate::SaddleError::Logging(format!("Failed to open log file: {}", log_file)))?;

        let file_layer = fmt::layer()
            .with_target(true)
            .with_ansi(false)
            .with_writer(file);

        registry.with(file_layer).init();
    } else {
        registry.init();
    }

    Ok(())
}
