use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::theme;
use crate::ui::components::shell::centered_rect;

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let palette = theme::default::palette();
    let card_height = if area.height >= 12 { 11 } else { area.height };
    let card = centered_rect(area, 46, card_height);
    let card = Rect {
        y: area
            .y
            .saturating_add(area.height.saturating_sub(card.height) / 3),
        ..card
    };
    let block = Block::default()
        .title(" Result ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette.muted));
    let inner = block.inner(card);
    frame.render_widget(block, card);

    let Some(result) = app.last_result.as_ref() else {
        frame.render_widget(
            Paragraph::new("test result will appear here")
                .style(Style::default().fg(palette.muted))
                .alignment(Alignment::Center),
            inner,
        );
        return;
    };

    if inner.width < 32 || inner.height < 7 {
        render_compact_result(frame, inner, result, palette);
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Min(2),
        ])
        .split(inner);
    let metrics = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(rows[0]);

    for (area, label, value) in [
        (metrics[0], "WPM", format!("{:.0}", result.wpm)),
        (metrics[1], "Accuracy", format!("{:.0}%", result.accuracy)),
        (metrics[2], "Mistakes", result.mistakes.to_string()),
    ] {
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(Span::styled(label, Style::default().fg(palette.muted))),
                Line::from(Span::styled(
                    value,
                    Style::default()
                        .fg(palette.text)
                        .add_modifier(Modifier::BOLD),
                )),
            ])
            .alignment(Alignment::Center),
            area,
        );
    }

    let secondary = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[2]);
    for (area, label, value) in [
        (secondary[0], "Mode", result.mode.label()),
        (
            secondary[1],
            "Time",
            format!("{:.2}s", result.elapsed_seconds),
        ),
    ] {
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(Span::styled(label, Style::default().fg(palette.muted))),
                Line::from(Span::styled(value, Style::default().fg(palette.text))),
            ])
            .alignment(Alignment::Center),
            area,
        );
    }
}

fn render_compact_result(
    frame: &mut Frame<'_>,
    area: Rect,
    result: &crate::core::test_session::TestResult,
    palette: theme::default::Palette,
) {
    let lines = [
        format!("WPM: {:.0}", result.wpm),
        format!("Accuracy: {:.0}%", result.accuracy),
        format!("Mistakes: {}", result.mistakes),
        format!("{}  {:.2}s", result.mode.label(), result.elapsed_seconds),
    ];
    frame.render_widget(
        Paragraph::new(lines.join("\n"))
            .style(
                Style::default()
                    .fg(palette.text)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
        area,
    );
}
