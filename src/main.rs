use clap::Parser;
use exn::ResultExt;
use saddle::{Cli, ConfigLoader, SaddleResult, SaddleError, init_logging};

fn main() -> SaddleResult<()> {
    let cli = Cli::parse();

    let settings = ConfigLoader::load().unwrap_or_else(|e| {
        tracing::warn!("Config error (using defaults): {}", e);
        saddle::Settings::default()
    });

    init_logging(&settings)
        .or_raise(|| SaddleError::Init("Failed to initialize logging".into()))?;

    tracing::info!(
        "Saddle started: {} v{}",
        settings.app.name,
        settings.app.version
    );

    cli.run()
}
