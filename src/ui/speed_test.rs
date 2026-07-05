use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::app::{App, InputMode};
use crate::core::test_session::TestMode;
use crate::theme;
use crate::ui::components::shell::centered_rect;
use crate::ui::components::typing_area;

const MODE_PANEL_MIN_WIDTH: u16 = 38;
const MODE_PANEL_HORIZONTAL_PADDING: u16 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TypingScreenLayout {
    text_area: Rect,
    status_area: Option<Rect>,
}

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let palette = theme::default::palette();
    let (metadata_area, typing_region) = speed_test_layout(area);
    render_mode_panel(frame, metadata_area, app);

    let typing_layout = typing_screen_layout(typing_region);

    if let Some(session) = app.session.as_ref() {
        if app.input_mode == InputMode::Typing {
            if let Some(status_area) = typing_layout.status_area {
                let status = Paragraph::new(format_status(session))
                    .style(Style::default().fg(palette.muted))
                    .alignment(Alignment::Left);
                frame.render_widget(status, status_area);
            }
        }

        typing_area::render(
            frame,
            typing_layout.text_area,
            session,
            app.input_mode == InputMode::Typing,
        );
    } else {
        let empty = Paragraph::new("speed-test")
            .style(Style::default().fg(palette.text))
            .alignment(Alignment::Left);
        frame.render_widget(empty, typing_layout.text_area);
    }
}

fn speed_test_layout(area: Rect) -> (Option<Rect>, Rect) {
    if area.height < 8 {
        return (None, area);
    }

    let metadata = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(3)])
        .split(area)[0];

    if area.height < 12 {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(3)])
            .split(area);

        return (Some(metadata), chunks[1]);
    }

    (Some(metadata), area)
}

fn render_mode_panel(frame: &mut Frame<'_>, area: Option<Rect>, app: &App) {
    let Some(area) = area else {
        return;
    };

    if area.width < 24 || area.height < 3 {
        return;
    }

    let label = mode_panel_label(app);
    let panel = centered_rect(area, mode_panel_width(&label, area.width), 3);
    let palette = theme::default::palette();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette.muted));
    let inner = padded_mode_panel_inner(block.inner(panel));

    frame.render_widget(block, panel);
    frame.render_widget(
        Paragraph::new(label)
            .style(
                Style::default()
                    .fg(palette.text)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
        inner,
    );
}

fn mode_panel_label(app: &App) -> String {
    format!(
        "mode: {}  |  language: {}",
        app.current_mode.label(),
        app.current_language_mode.label()
    )
}

fn mode_panel_width(label: &str, max_width: u16) -> u16 {
    let desired_width = u16::try_from(label.chars().count())
        .unwrap_or(u16::MAX)
        .saturating_add(MODE_PANEL_HORIZONTAL_PADDING.saturating_mul(2))
        .saturating_add(2)
        .max(MODE_PANEL_MIN_WIDTH);

    desired_width.min(max_width)
}

fn padded_mode_panel_inner(area: Rect) -> Rect {
    let padding = MODE_PANEL_HORIZONTAL_PADDING.min(area.width / 2);

    Rect {
        x: area.x.saturating_add(padding),
        width: area.width.saturating_sub(padding.saturating_mul(2)),
        ..area
    }
}

fn typing_screen_layout(area: Rect) -> TypingScreenLayout {
    if area.width == 0 || area.height == 0 {
        return TypingScreenLayout {
            text_area: area,
            status_area: None,
        };
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

    let text_area = Rect {
        x: area.x.saturating_add(left_margin),
        y: area.y.saturating_add(y_offset),
        width: text_width,
        height: text_height,
    };

    let status_area = status_area_above(text_area, area.y);

    TypingScreenLayout {
        text_area,
        status_area,
    }
}

fn status_area_above(text_area: Rect, min_y: u16) -> Option<Rect> {
    if text_area.y <= min_y {
        return None;
    }

    let y = if text_area.y > min_y.saturating_add(1) {
        text_area.y.saturating_sub(2)
    } else {
        text_area.y.saturating_sub(1)
    };

    Some(Rect {
        x: text_area.x,
        y,
        width: text_area.width,
        height: 1,
    })
}

fn format_status(session: &crate::core::test_session::TypingSession) -> String {
    match session.mode {
        TestMode::Time(seconds) => {
            let remaining = (f64::from(seconds) - session.elapsed_seconds()).max(0.0);
            format!("{remaining:.0}s")
        }
        TestMode::Words(total_words) => {
            let completed_words = session.completed_words().min(usize::from(total_words));
            format!("{completed_words}/{total_words}")
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::{mode_panel_width, padded_mode_panel_inner, typing_screen_layout};

    #[test]
    fn uses_nine_eighty_two_nine_horizontal_split() {
        let layout = typing_screen_layout(Rect::new(0, 0, 100, 24));

        assert_eq!(layout.text_area.x, 9);
        assert_eq!(layout.text_area.width, 82);
    }

    #[test]
    fn centers_three_line_typing_area_vertically() {
        let layout = typing_screen_layout(Rect::new(0, 0, 100, 25));

        assert_eq!(layout.text_area.y, 11);
        assert_eq!(layout.text_area.height, 3);
    }

    #[test]
    fn clamps_for_small_terminals() {
        let layout = typing_screen_layout(Rect::new(0, 0, 2, 2));

        assert_eq!(layout.text_area.width, 2);
        assert_eq!(layout.text_area.height, 2);
    }

    #[test]
    fn status_area_aligns_with_text_area_above_text() {
        let layout = typing_screen_layout(Rect::new(0, 0, 100, 24));
        let status_area = layout.status_area.expect("status area");

        assert_eq!(status_area.x, layout.text_area.x);
        assert_eq!(status_area.width, layout.text_area.width);
        assert!(status_area.y < layout.text_area.y);
    }

    #[test]
    fn speed_test_text_region_uses_full_main_area_when_tall_enough() {
        let area = Rect::new(0, 0, 100, 30);
        let (_metadata, typing_region) = super::speed_test_layout(area);

        assert_eq!(typing_region, area);
    }

    #[test]
    fn mode_panel_width_grows_for_long_language_labels() {
        let short = mode_panel_width("mode: 15s  |  language: english", 100);
        let long = mode_panel_width("mode: 15s  |  language: azerbaijani-hard", 100);

        assert!(long > short);
        assert!(short >= super::MODE_PANEL_MIN_WIDTH);
    }

    #[test]
    fn mode_panel_width_clamps_to_available_terminal_width() {
        let width = mode_panel_width("mode: 15s  |  language: azerbaijani-hard", 20);

        assert_eq!(width, 20);
    }

    #[test]
    fn mode_panel_inner_keeps_horizontal_padding_when_possible() {
        let inner = padded_mode_panel_inner(Rect::new(10, 0, 20, 1));

        assert_eq!(inner.x, 12);
        assert_eq!(inner.width, 16);
    }
}
