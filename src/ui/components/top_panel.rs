use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::theme;

pub fn render_command_line(frame: &mut Frame<'_>, command_input: &str) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let palette = theme::default::palette();
    let command_line = Paragraph::new(fit_line(&format!(":{command_input}"), area.width))
        .style(Style::default().fg(palette.accent));

    frame.render_widget(command_line, chunks[1]);
}

pub fn render_command_error(frame: &mut Frame<'_>, error: &str) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let message =
        Paragraph::new(fit_line(error, area.width)).style(Style::default().fg(Color::Red));

    frame.render_widget(message, chunks[1]);
}

fn fit_line(text: &str, width: u16) -> String {
    let max_width = usize::from(width);

    if max_width == 0 {
        return String::new();
    }

    text.chars().take(max_width).collect()
}
