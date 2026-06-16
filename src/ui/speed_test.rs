use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::{App, InputMode};
use crate::theme;
use crate::ui::components::typing_area;

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    let palette = theme::default::palette();

    let header = Paragraph::new(format!(
        "ttp\nmode: {}    language: english",
        app.current_mode.label()
    ))
    .style(
        Style::default()
            .fg(palette.accent)
            .add_modifier(Modifier::BOLD),
    )
    .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    if let Some(session) = app.session.as_ref() {
        typing_area::render(
            frame,
            chunks[1],
            session,
            app.input_mode == InputMode::Typing,
        );
    } else {
        let empty = Paragraph::new("speed-test")
            .style(Style::default().fg(palette.text))
            .alignment(Alignment::Center);
        frame.render_widget(empty, chunks[1]);
    }

    let footer_text = if app.input_mode == InputMode::Normal {
        "press s to start typing".to_owned()
    } else if let Some(session) = app.session.as_ref() {
        format_status(session)
    } else {
        String::new()
    };

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(palette.muted))
        .alignment(Alignment::Center);
    frame.render_widget(footer, chunks[2]);
}

fn format_status(session: &crate::core::test_session::TypingSession) -> String {
    match session.mode {
        crate::core::test_session::TestMode::Time(seconds) => {
            let remaining = (f64::from(seconds) - session.elapsed_seconds()).max(0.0);
            format!("{remaining:.0}s")
        }
        crate::core::test_session::TestMode::Words(_) => {
            let total = session.target_text.chars().count();
            format!("{} / {}", session.current_index.min(total), total)
        }
    }
}
