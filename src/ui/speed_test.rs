use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::theme;

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    let palette = theme::default::palette();

    let header = Paragraph::new("ttp")
        .style(
            Style::default()
                .fg(palette.accent)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    let body = Paragraph::new("speed-test\n\npress s to start typing")
        .style(Style::default().fg(palette.text))
        .alignment(Alignment::Center)
        .block(Block::default());
    frame.render_widget(body, chunks[1]);

    let footer = Paragraph::new(format!("mode: {}", app.input_mode_label()))
        .style(Style::default().fg(palette.muted))
        .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[2]);
}
