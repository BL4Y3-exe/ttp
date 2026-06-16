use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::App;
use crate::storage::models::SavedTestResult;
use crate::theme;

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let palette = theme::default::palette();

    frame.render_widget(
        Paragraph::new("ttp\nhistory")
            .style(
                Style::default()
                    .fg(palette.accent)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(history_text(&app.recent_results))
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

fn history_text(results: &[SavedTestResult]) -> String {
    if results.is_empty() {
        return "No results yet.\nComplete a test first.".to_owned();
    }

    let mut lines = vec![format!(
        "{:<8} {:>6} {:>10} {:>9}   {}",
        "Mode", "WPM", "Accuracy", "Mistakes", "Date"
    )];

    for result in results {
        lines.push(format!(
            "{:<8} {:>6.0} {:>9.0}% {:>9}   {}",
            result.mode_label(),
            result.wpm,
            result.accuracy,
            result.mistakes,
            result.created_at.format("%Y-%m-%d %H:%M")
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::history_text;

    #[test]
    fn empty_history_formatting_does_not_panic() {
        let text = history_text(&[]);

        assert!(text.contains("No results yet."));
        assert!(text.contains("Complete a test first."));
    }
}
