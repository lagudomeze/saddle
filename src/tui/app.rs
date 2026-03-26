use crate::SaddleResult;
use crate::tui::Theme;
use exn::ResultExt;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, List, ListItem},
    layout::{Layout, Constraint, Direction, Alignment},
    Terminal,
    style::Style,
    text::{Text, Span},
};
use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, EnableMouseCapture, DisableMouseCapture},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const MAX_OUTPUT_LINES: usize = 1000;

#[derive(Debug, Clone)]
pub struct TuiApp {
    title: String,
    output_lines: Vec<String>,
    input_buffer: String,
    scroll_offset: usize,
    history_index: Option<usize>,
    command_history: Vec<String>,
    theme: Theme,
    should_quit: bool,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            title: "Saddle CLI".to_string(),
            output_lines: vec![
                format!("Welcome to Saddle CLI v{}", VERSION),
                "Type 'help' for available commands.".to_string(),
                "".to_string(),
            ],
            input_buffer: String::new(),
            scroll_offset: 0,
            history_index: None,
            command_history: Vec::new(),
            theme: Theme::default(),
            should_quit: false,
        }
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn run(&mut self) -> SaddleResult<()> {
        enable_raw_mode().or_raise(|| crate::SaddleError::Tui("Failed to enable raw mode".into()))?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .or_raise(|| crate::SaddleError::Tui("Failed to enter alternate screen".into()))?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)
            .or_raise(|| crate::SaddleError::Tui("Failed to create terminal".into()))?;

        terminal.clear()
            .or_raise(|| crate::SaddleError::Tui("Failed to clear terminal".into()))?;

        self.draw(&mut terminal)?;

        while !self.should_quit {
            if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
                if let Event::Key(key) = event::read()
                    .or_raise(|| crate::SaddleError::Tui("Failed to read event".into()))?
                {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key_event(key.code);
                    }
                }
                self.draw(&mut terminal)?;
            }
        }

        disable_raw_mode().or_raise(|| crate::SaddleError::Tui("Failed to disable raw mode".into()))?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)
            .or_raise(|| crate::SaddleError::Tui("Failed to leave alternate screen".into()))?;
        terminal.show_cursor()
            .or_raise(|| crate::SaddleError::Tui("Failed to show cursor".into()))?;

        Ok(())
    }

    fn handle_key_event(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
                self.history_index = None;
            },
            KeyCode::Backspace => {
                self.input_buffer.pop();
            },
            KeyCode::Enter => {
                if !self.input_buffer.trim().is_empty() {
                    let input = self.input_buffer.trim().to_string();
                    self.command_history.push(input.clone());
                    self.execute_command(&input);
                    self.input_buffer.clear();
                }
            },
            KeyCode::Up => {
                if let Some(idx) = self.history_index {
                    if idx > 0 {
                        self.history_index = Some(idx - 1);
                        self.input_buffer = self.command_history[idx - 1].clone();
                    }
                } else if !self.command_history.is_empty() {
                    self.history_index = Some(self.command_history.len() - 1);
                    self.input_buffer = self.command_history.last().unwrap().clone();
                }
            },
            KeyCode::Down => {
                if let Some(idx) = self.history_index {
                    if idx < self.command_history.len() - 1 {
                        self.history_index = Some(idx + 1);
                        self.input_buffer = self.command_history[idx + 1].clone();
                    } else {
                        self.history_index = None;
                        self.input_buffer.clear();
                    }
                }
            },
            KeyCode::PageUp => {
                let height = 20;
                self.scroll_offset = self.scroll_offset.saturating_sub(height);
            },
            KeyCode::PageDown => {
                let height = 20;
                self.scroll_offset = (self.scroll_offset + height).min(self.output_lines.len().saturating_sub(1));
            },
            KeyCode::Home => {
                if event::read().map(|e| matches!(e, Event::Key(k) if k.modifiers.contains(crossterm::event::KeyModifiers::CONTROL))).unwrap_or(false) {
                    self.scroll_offset = 0;
                }
            },
            KeyCode::End => {
                if event::read().map(|e| matches!(e, Event::Key(k) if k.modifiers.contains(crossterm::event::KeyModifiers::CONTROL))).unwrap_or(false) {
                    self.scroll_offset = self.output_lines.len().saturating_sub(1);
                }
            },
            KeyCode::Esc => {
                if self.input_buffer.is_empty() {
                    self.should_quit = true;
                }
            },
            _ => {},
        }
    }

    fn execute_command(&mut self, input: &str) {
        self.output_lines.push(format!("❯ {}", input));
        let response = self.process_input(input);
        if let Some(response_text) = response {
            for line in response_text.lines() {
                self.output_lines.push(line.to_string());
            }
        } else {
            self.output_lines.clear();
        }
        self.output_lines.push(String::new());

        if self.output_lines.len() > MAX_OUTPUT_LINES {
            self.output_lines = self.output_lines.split_off(self.output_lines.len() - MAX_OUTPUT_LINES);
        }
        self.scroll_offset = self.output_lines.len().saturating_sub(1);
    }

    fn process_input(&self, input: &str) -> Option<String> {
        match input.trim().to_lowercase().as_str() {
            "help" => Some(self.help_text()),
            "status" => Some("Use 'saddle status' command for detailed status.".to_string()),
            "list" => Some("Use 'saddle status --verbose' for full feature list.".to_string()),
            "run" => Some("Starting application...".to_string()),
            "clear" => None,
            "theme" | "themes" => Some("Available themes: default (nord), dracula, monokai".to_string()),
            _ => Some(format!("Unknown command: {}\nType 'help' for available commands.", input.trim())),
        }
    }

    fn help_text(&self) -> String {
        r#"Available commands:
  help        - Show this help message
  status      - Show project status
  list        - List all features
  run         - Run the application
  clear       - Clear the output
  theme       - Show available themes
  quit/exit   - Exit the application

Keyboard shortcuts:
  ↑/↓         - Navigate command history
  PageUp/Down - Scroll output
  Esc         - Exit"#.to_string()
    }

    fn draw(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> SaddleResult<()> {
        terminal.draw(|f| {
            let area = f.area();
            
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(area);

            self.draw_header(f, vertical_chunks[0]);
            self.draw_output(f, vertical_chunks[1]);
            self.draw_input(f, vertical_chunks[2]);
            self.draw_status_bar(f, vertical_chunks[3]);
        }).or_raise(|| crate::SaddleError::Tui("Failed to draw terminal".into()))?;
        
        Ok(())
    }

    fn draw_header(&self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let header_text = Text::from(Span::raw(format!(
            " {} v{} | Press 'help' for commands ",
            self.title, VERSION
        )));
        let header = Paragraph::new(header_text)
            .style(self.theme.title)
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.border));
        f.render_widget(header, area);
    }

    fn draw_output(&self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let visible_lines: Vec<ListItem> = self.output_lines
            .iter()
            .skip(self.scroll_offset)
            .take(area.height as usize)
            .map(|line| {
                let (style, content) = self.style_output_line(line);
                ListItem::new(Text::from(Span::styled(content, style)))
            })
            .collect();

        let output_block = Block::default()
            .title(" Output ")
            .borders(Borders::ALL)
            .border_style(self.theme.border);

        let list = List::new(visible_lines)
            .block(output_block)
            .style(self.theme.output);
        f.render_widget(list, area);
    }

    fn style_output_line(&self, line: &str) -> (Style, String) {
        if line.starts_with("❯ ") {
            (self.theme.highlight, line.to_string())
        } else if line.starts_with("Error") || line.starts_with("error") {
            (self.theme.status_error, line.to_string())
        } else if line.starts_with("✓") || line.starts_with("✔") {
            (self.theme.status_ok, line.to_string())
        } else if line.starts_with("⚠") || line.starts_with("!") {
            (self.theme.status_pending, line.to_string())
        } else if line.is_empty() {
            (self.theme.dim, " ".to_string())
        } else {
            (self.theme.output_text, line.to_string())
        }
    }

    fn draw_input(&self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let input_text = if self.input_buffer.is_empty() {
            Text::from(Span::raw(""))
        } else {
            Text::from(Span::styled(self.input_buffer.as_str(), self.theme.input_text))
        };

        let input_block = Block::default()
            .title(" Command Input ")
            .borders(Borders::ALL)
            .border_style(self.theme.border_selected)
            .style(self.theme.input);

        let input = Paragraph::new(input_text)
            .block(input_block)
            .alignment(Alignment::Left);
        f.render_widget(input, area);

        if area.width > self.input_buffer.len() as u16 + 2 {
            f.set_cursor(
                area.x + self.input_buffer.len() as u16 + 1,
                area.y + 1,
            );
        }
    }

    fn draw_status_bar(&self, f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let history_count = self.command_history.len();
        let status_text = Text::from(Span::raw(format!(
            " History: {} | Lines: {} | Press Esc to quit ",
            history_count,
            self.output_lines.len()
        )));
        let status = Paragraph::new(status_text)
            .style(self.theme.dim)
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(self.theme.border));
        f.render_widget(status, area);
    }
}
