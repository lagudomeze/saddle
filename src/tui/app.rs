use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    Terminal,
    style::{Style, Color},
};
use std::io;

#[derive(Debug)]
pub struct TuiError {
    message: String,
}

impl TuiError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for TuiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TuiError {}

#[derive(Debug, Clone)]
pub struct TuiApp {
    title: String,
    output_lines: Vec<String>,
    input_buffer: String,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            title: "Saddle CLI".to_string(),
            output_lines: vec!["Welcome to Saddle CLI".to_string(), "Type 'help' for available commands.".to_string()],
            input_buffer: String::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        
        terminal.clear()?;
        self.draw(&mut terminal)?;
        
        loop {
            if let Ok(input) = self.read_line() {
                if input.trim().is_empty() {
                    continue;
                }
                
                let response = self.process_input(&input);
                self.output_lines.push(format!("> {}", input));
                if !response.is_empty() {
                    self.output_lines.push(response);
                }
                
                if input.trim().to_lowercase() == "quit" || input.trim().to_lowercase() == "exit" {
                    break;
                }
                
                self.draw(&mut terminal)?;
            }
        }
        
        Ok(())
    }

    fn read_line(&self) -> Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    fn process_input(&self, input: &str) -> String {
        match input.trim().to_lowercase().as_str() {
            "help" => {
                "Available commands:\n  status    - Show project status\n  list      - List all features\n  run       - Run the application\n  quit/exit - Exit the application".to_string()
            },
            "status" => {
                "Use 'saddle status' command for detailed status.".to_string()
            },
            "list" => {
                "Use 'saddle status --verbose' for full feature list.".to_string()
            },
            "run" => {
                "Starting application...".to_string()
            },
            _ => format!("Unknown command: {}\nType 'help' for available commands.", input.trim()),
        }
    }

    fn draw(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        terminal.draw(|f| {
            let area = f.area();
            
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(area);

            let block = Block::default()
                .title(self.title.as_str())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            
            let output_text = self.output_lines.join("\n");
            let output_paragraph = Paragraph::new(output_text.as_str())
                .block(block);
            
            f.render_widget(output_paragraph, chunks[0]);

            let input_block = Block::default()
                .title(" Command Input ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green));
            
            let input_paragraph = Paragraph::new(self.input_buffer.as_str())
                .block(input_block)
                .style(Style::default().fg(Color::Yellow));
            
            f.render_widget(input_paragraph, chunks[1]);

            let status_bar = Paragraph::new(" Press Ctrl+C to exit ")
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(status_bar, chunks[2]);
        })?;
        
        Ok(())
    }
}
