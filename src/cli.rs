use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "saddle")]
#[command(version = "0.1.0")]
#[command(about = "Harness-native CLI with local memory and plugin system")]
pub struct Cli {
    #[arg(short, long, default_value = "info")]
    pub verbose: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser, Debug)]
pub enum Commands {
    Run,
    Init,
    Status,
}
