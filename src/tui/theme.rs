use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub title: Style,
    pub border: Style,
    pub border_selected: Style,
    pub input: Style,
    pub input_text: Style,
    pub output: Style,
    pub output_text: Style,
    pub status_ok: Style,
    pub status_error: Style,
    pub status_pending: Style,
    pub status_info: Style,
    pub highlight: Style,
    pub dim: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self::nord()
    }
}

impl Theme {
    pub fn nord() -> Self {
        Self {
            title: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            border: Style::default().fg(Color::Blue),
            border_selected: Style::default().fg(Color::LightCyan),
            input: Style::default().bg(Color::DarkGray),
            input_text: Style::default().fg(Color::White),
            output: Style::default().bg(Color::Black),
            output_text: Style::default().fg(Color::White),
            status_ok: Style::default().fg(Color::Green),
            status_error: Style::default().fg(Color::Red),
            status_pending: Style::default().fg(Color::Yellow),
            status_info: Style::default().fg(Color::Blue),
            highlight: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            dim: Style::default().fg(Color::DarkGray),
        }
    }

    pub fn dracula() -> Self {
        Self {
            title: Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            border: Style::default().fg(Color::Magenta),
            border_selected: Style::default().fg(Color::LightMagenta),
            input: Style::default().bg(Color::Rgb(40, 42, 54)),
            input_text: Style::default().fg(Color::White),
            output: Style::default().bg(Color::Rgb(40, 42, 54)),
            output_text: Style::default().fg(Color::White),
            status_ok: Style::default().fg(Color::Green),
            status_error: Style::default().fg(Color::Red),
            status_pending: Style::default().fg(Color::Yellow),
            status_info: Style::default().fg(Color::Cyan),
            highlight: Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            dim: Style::default().fg(Color::DarkGray),
        }
    }

    pub fn monokai() -> Self {
        Self {
            title: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            border: Style::default().fg(Color::Yellow),
            border_selected: Style::default().fg(Color::LightYellow),
            input: Style::default().bg(Color::Rgb(39, 40, 34)),
            input_text: Style::default().fg(Color::White),
            output: Style::default().bg(Color::Rgb(39, 40, 34)),
            output_text: Style::default().fg(Color::White),
            status_ok: Style::default().fg(Color::Green),
            status_error: Style::default().fg(Color::Red),
            status_pending: Style::default().fg(Color::Yellow),
            status_info: Style::default().fg(Color::Cyan),
            highlight: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            dim: Style::default().fg(Color::DarkGray),
        }
    }
}
