use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
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

    let typing_area = typing_text_area(area);

    if let Some(session) = app.session.as_ref() {
        typing_area::render(
            frame,
            typing_area,
            session,
            app.input_mode == InputMode::Typing,
        );
    } else {
        let empty = Paragraph::new("speed-test")
            .style(Style::default().fg(palette.text))
            .alignment(Alignment::Left);
        frame.render_widget(empty, typing_area);
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

fn typing_text_area(area: Rect) -> Rect {
    if area.width == 0 || area.height == 0 {
        return area;
    }

    let left_margin = ((u32::from(area.width) * 9) / 100) as u16;
    let right_margin = ((u32::from(area.width) * 9) / 100) as u16;
    let used_margins = left_margin.saturating_add(right_margin);
    let text_width = area.width.saturating_sub(used_margins).max(1);

    let text_height = area.height.min(3);
    let y_offset = if area.height <= text_height {
        0
    } else {
        (area.height / 2).saturating_sub(1)
    };

    Rect {
        x: area.x.saturating_add(left_margin),
        y: area.y.saturating_add(y_offset),
        width: text_width,
        height: text_height,
    }
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

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::typing_text_area;

    #[test]
    fn uses_nine_eighty_two_nine_horizontal_split() {
        let area = typing_text_area(Rect::new(0, 0, 100, 24));

        assert_eq!(area.x, 9);
        assert_eq!(area.width, 82);
    }

    #[test]
    fn centers_three_line_typing_area_vertically() {
        let area = typing_text_area(Rect::new(0, 0, 100, 25));

        assert_eq!(area.y, 11);
        assert_eq!(area.height, 3);
    }

    #[test]
    fn clamps_for_small_terminals() {
        let area = typing_text_area(Rect::new(0, 0, 2, 2));

        assert_eq!(area.width, 2);
        assert_eq!(area.height, 2);
    }
}
