use crate::SaddleResult;
use crate::harness::{FeatureManager, ProgressTracker};
use crate::tui::TuiApp;
use clap::Parser;
use exn::ResultExt;

#[derive(Parser, Debug)]
#[command(name = "saddle")]
#[command(version = "0.1.0")]
#[command(about = "Harness-native CLI with local memory and plugin system")]
pub struct Cli {
    #[arg(short, long, default_value = "info")]
    pub verbose: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Run the main application (TUI mode)
    Run(RunCmd),
    /// Initialize the project
    Init(InitCmd),
    /// Show project status
    Status(StatusCmd),
}

#[derive(Parser, Debug, Default)]
pub struct RunCmd {
    #[arg(short, long, default_value = "harness/features.json")]
    pub features: Option<String>,
}

#[derive(Parser, Debug, Default)]
pub struct InitCmd {
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Parser, Debug, Default)]
pub struct StatusCmd {
    #[arg(short, long)]
    pub verbose: bool,
}

impl Cli {
    pub fn run(&self) -> SaddleResult<()> {
        match &self.command {
            Commands::Run(cmd) => cmd.run(),
            Commands::Init(cmd) => cmd.run(),
            Commands::Status(cmd) => cmd.run(),
        }
    }
}

impl RunCmd {
    pub fn run(&self) -> SaddleResult<()> {
        let mut app = TuiApp::new();
        app.run()
    }
}

impl InitCmd {
    pub fn run(&self) -> SaddleResult<()> {
        let fm = FeatureManager::new()
            .or_raise(|| crate::SaddleError::Feature("Failed to create FeatureManager".into()))?;

        if !self.force && fm.load().is_ok() {
            tracing::warn!("Project already initialized. Use --force to reinitialize.");
            return Ok(());
        }

        let pt = ProgressTracker::new();
        pt.update("# 进度报告\n\n## 初始化阶段\n\n### 已完成\n- [x] 项目初始化完成\n")
            .or_raise(|| crate::SaddleError::Progress("Failed to update progress".into()))?;

        tracing::info!("Project initialized successfully.");
        Ok(())
    }
}

impl StatusCmd {
    pub fn run(&self) -> SaddleResult<()> {
        let fm = FeatureManager::new()
            .or_raise(|| crate::SaddleError::Feature("Failed to create FeatureManager".into()))?;
        let features = fm.load()
            .or_raise(|| crate::SaddleError::Feature("Failed to load features".into()))?;

        let completed: Vec<_> = features
            .iter()
            .filter(|f| f.status == "completed")
            .collect();
        let pending: Vec<_> = features.iter().filter(|f| f.status == "pending").collect();

        tracing::info!("Project Status");
        tracing::info!("==============");
        tracing::info!("Total features: {}", features.len());
        tracing::info!(
            "Completed: {} ({:.1}%)",
            completed.len(),
            (completed.len() as f64 / features.len() as f64) * 100.0
        );
        tracing::info!(
            "Pending: {} ({:.1}%)",
            pending.len(),
            (pending.len() as f64 / features.len() as f64) * 100.0
        );

        if self.verbose {
            tracing::info!("\nCompleted features:");
            for f in &completed {
                tracing::info!("  [✓] {} - {}", f.id, f.title);
            }
            tracing::info!("\nPending features:");
            for f in &pending {
                tracing::info!("  [ ] {} - {}", f.id, f.title);
            }
        }

        Ok(())
    }
}
