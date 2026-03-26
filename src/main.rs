use anyhow::Result;
use clap::Parser;
use saddle::{ConfigLoader, init_logging, Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let settings = match ConfigLoader::load() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Warning: Config error (using defaults): {}", e);
            saddle::Settings::default()
        }
    };
    
    init_logging(&settings)?;

    tracing::info!("Saddle started: {} v{}", settings.app.name, settings.app.version);

    cli.run()?;
    
    Ok(())
}
