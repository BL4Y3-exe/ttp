use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;
use crate::theme;

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let palette = theme::default::palette();

    frame.render_widget(
        Paragraph::new("ttp")
            .style(
                Style::default()
                    .fg(palette.accent)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
        chunks[0],
    );
    let result_text = app.last_result.as_ref().map_or_else(
        || "result\n\ntest result will appear here".to_owned(),
        |result| {
            format!(
                "result\n\nWPM: {:.0}\nAccuracy: {:.0}%\nMistakes: {}\nMode: {}\nTime: {:.2}s",
                result.wpm,
                result.accuracy,
                result.mistakes,
                result.mode.label(),
                result.elapsed_seconds
            )
        },
    );

    frame.render_widget(
        Paragraph::new(result_text)
            .style(Style::default().fg(palette.text))
            .alignment(Alignment::Center),
        chunks[1],
    );
    frame.render_widget(
        Paragraph::new(format!("mode: {}", app.input_mode_label()))
            .style(Style::default().fg(palette.muted))
            .alignment(Alignment::Center),
        chunks[2],
    );
}
