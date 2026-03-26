use clap::Parser;
use crate::harness::{FeatureManager, ProgressTracker};
use crate::tui::TuiApp;

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
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Some(Commands::Run(cmd)) => cmd.run(),
            Some(Commands::Init(cmd)) => cmd.run(),
            Some(Commands::Status(cmd)) => cmd.run(),
            None => {
                println!("Use --help for usage information");
                Ok(())
            }
        }
    }
}

impl RunCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut app = TuiApp::new();
        app.run()?;
        Ok(())
    }
}

impl InitCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        let fm = FeatureManager::new()?;
        
        if !self.force && fm.load().is_ok() {
            println!("Project already initialized. Use --force to reinitialize.");
            return Ok(());
        }
        
        let pt = ProgressTracker::new();
        pt.update("# 进度报告\n\n## 初始化阶段\n\n### 已完成\n- [x] 项目初始化完成\n")?;
        
        println!("Project initialized successfully.");
        Ok(())
    }
}

impl StatusCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        let fm = FeatureManager::new()?;
        let features = fm.load()?;
        
        let completed: Vec<_> = features.iter()
            .filter(|f| f.status == "completed")
            .collect();
        let pending: Vec<_> = features.iter()
            .filter(|f| f.status == "pending")
            .collect();
        
        println!("Project Status");
        println!("==============");
        println!("Total features: {}", features.len());
        println!("Completed: {} ({:.1}%)", completed.len(), 
            (completed.len() as f64 / features.len() as f64) * 100.0);
        println!("Pending: {} ({:.1}%)", pending.len(),
            (pending.len() as f64 / features.len() as f64) * 100.0);
        
        if self.verbose {
            println!("\nCompleted features:");
            for f in &completed {
                println!("  [✓] {} - {}", f.id, f.title);
            }
            println!("\nPending features:");
            for f in &pending {
                println!("  [ ] {} - {}", f.id, f.title);
            }
        }
        
        Ok(())
    }
}
