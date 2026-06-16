use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use crate::core::test_session::TypingSession;

pub fn render(frame: &mut Frame<'_>, area: Rect, session: &TypingSession, active: bool) {
    let text = if active {
        active_text(session)
    } else {
        disabled_text(&session.target_text)
    };

    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn active_text(session: &TypingSession) -> Text<'static> {
    let target_chars: Vec<char> = session.target_text.chars().collect();
    let typed_chars: Vec<char> = session.typed_input.chars().collect();
    let mut spans = Vec::with_capacity(target_chars.len().max(typed_chars.len()));

    for (index, expected) in target_chars.iter().copied().enumerate() {
        if let Some(typed) = typed_chars.get(index).copied() {
            if typed == expected {
                spans.push(Span::styled(
                    typed.to_string(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    typed.to_string(),
                    Style::default().fg(Color::Red),
                ));
            }
        } else if index == session.current_index {
            spans.push(Span::styled(
                expected.to_string(),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(
                expected.to_string(),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }

    if typed_chars.len() > target_chars.len() {
        for typed in typed_chars.iter().skip(target_chars.len()) {
            spans.push(Span::styled(
                typed.to_string(),
                Style::default().fg(Color::Red),
            ));
        }
    }

    Text::from(Line::from(spans))
}

fn disabled_text(target_text: &str) -> Text<'static> {
    let blurred = target_text
        .chars()
        .enumerate()
        .map(|(index, ch)| {
            if ch.is_whitespace() {
                ch
            } else {
                match index % 3 {
                    0 => '░',
                    1 => '▒',
                    _ => '▓',
                }
            }
        })
        .collect::<String>();

    Text::from(Line::from(Span::styled(
        blurred,
        Style::default().fg(Color::DarkGray),
    )))
}
