use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::theme;

pub fn render_command_line(frame: &mut Frame<'_>, command_input: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let palette = theme::default::palette();
    let command_line =
        Paragraph::new(format!(":{command_input}")).style(Style::default().fg(palette.accent));

    frame.render_widget(command_line, chunks[1]);
}
