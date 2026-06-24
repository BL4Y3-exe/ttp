use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::app::{App, InputMode, Page};
use crate::theme;

#[derive(Debug, Clone, Copy)]
pub struct ScreenLayout {
    pub header: Rect,
    pub main: Rect,
    pub footer: Rect,
}

pub fn layout(area: Rect) -> ScreenLayout {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(if area.width >= 8 { 2 } else { 0 }),
            Constraint::Min(1),
            Constraint::Length(if area.width >= 8 { 2 } else { 0 }),
        ])
        .split(area);

    ScreenLayout {
        header: Rect {
            x: horizontal[1].x,
            y: vertical[1].y,
            width: horizontal[1].width,
            height: vertical[1].height,
        },
        main: Rect {
            x: horizontal[1].x,
            y: vertical[3].y,
            width: horizontal[1].width,
            height: vertical[3].height,
        },
        footer: Rect {
            x: horizontal[1].x,
            y: vertical[4].y,
            width: horizontal[1].width,
            height: vertical[4].height,
        },
    }
}

pub fn render_header(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let palette = theme::default::palette();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette.muted));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(23)])
        .split(inner);

    frame.render_widget(
        Paragraph::new("ttp")
            .style(
                Style::default()
                    .fg(palette.text)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Left),
        columns[0],
    );

    let speed_test_active = app.page != Page::History;
    let active_style = Style::default()
        .fg(palette.accent)
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(palette.muted);
    let navigation = Line::from(vec![
        Span::styled(
            "speed-test",
            if speed_test_active {
                active_style
            } else {
                inactive_style
            },
        ),
        Span::styled("  |  ", Style::default().fg(palette.text)),
        Span::styled(
            "profile",
            if speed_test_active {
                inactive_style
            } else {
                active_style
            },
        ),
    ]);

    frame.render_widget(
        Paragraph::new(navigation).alignment(Alignment::Right),
        columns[1],
    );
}

pub fn render_footer(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let hint = match (app.page, app.input_mode) {
        (Page::SpeedTest, InputMode::Normal) => "press s to start typing",
        (Page::Result, InputMode::Normal) => "press s to start a new test",
        (Page::History, InputMode::Normal) => "j/k: scroll",
        _ => "",
    };

    frame.render_widget(
        Paragraph::new(hint)
            .style(Style::default().fg(theme::default::palette().muted))
            .alignment(Alignment::Center),
        area,
    );
}

pub fn centered_rect(area: Rect, desired_width: u16, desired_height: u16) -> Rect {
    let width = desired_width.min(area.width);
    let height = desired_height.min(area.height);

    Rect {
        x: area.x.saturating_add(area.width.saturating_sub(width) / 2),
        y: area
            .y
            .saturating_add(area.height.saturating_sub(height) / 2),
        width,
        height,
    }
}
