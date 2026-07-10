use chrono::Local;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use ratatui::Frame;

use crate::app::App;
use crate::storage::models::SavedTestResult;
use crate::theme;

const TIME_MODES: &[u16] = &[15, 30, 60, 120];
const WORD_MODES: &[u16] = &[10, 25, 50, 100];

#[derive(Debug, Clone, Copy, PartialEq)]
struct SummaryStats {
    tests_completed: usize,
    highest_wpm: Option<f64>,
    average_wpm: Option<f64>,
    highest_accuracy: Option<f64>,
    average_accuracy: Option<f64>,
}

impl SummaryStats {
    fn from_results(results: &[SavedTestResult]) -> Self {
        if results.is_empty() {
            return Self {
                tests_completed: 0,
                highest_wpm: None,
                average_wpm: None,
                highest_accuracy: None,
                average_accuracy: None,
            };
        }

        let tests_completed = results.len();
        let highest_wpm = results.iter().map(|result| result.wpm).reduce(f64::max);
        let highest_accuracy = results
            .iter()
            .map(|result| result.accuracy)
            .reduce(f64::max);
        let average_wpm =
            Some(results.iter().map(|result| result.wpm).sum::<f64>() / tests_completed as f64);
        let average_accuracy = Some(
            results.iter().map(|result| result.accuracy).sum::<f64>() / tests_completed as f64,
        );

        Self {
            tests_completed,
            highest_wpm,
            average_wpm,
            highest_accuracy,
            average_accuracy,
        }
    }
}

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let dashboard = dashboard_layout(area, app.recent_results.len());
    let palette = theme::default::palette();
    let today_stats = SummaryStats::from_results(&today_results(&app.all_results));
    let overall_stats = SummaryStats::from_results(&app.all_results);

    if let Some(today) = visible_rect(dashboard.today, area, app.stats_scroll_offset) {
        render_stats_panel(
            frame,
            today,
            " Today's Statistics ",
            &[
                ("tests completed", today_stats.tests_completed.to_string()),
                (
                    "highest WPM",
                    format_optional_number(today_stats.highest_wpm),
                ),
                (
                    "average WPM",
                    format_optional_number(today_stats.average_wpm),
                ),
            ],
        );
    }

    if let Some(personal_bests) =
        visible_rect(dashboard.personal_bests, area, app.stats_scroll_offset)
    {
        render_personal_bests(frame, personal_bests, app, palette);
    }

    if let Some(overall) = visible_rect(dashboard.overall, area, app.stats_scroll_offset) {
        render_stats_panel(
            frame,
            overall,
            " Overall Statistics ",
            &[
                ("tests completed", overall_stats.tests_completed.to_string()),
                (
                    "highest WPM",
                    format_optional_number(overall_stats.highest_wpm),
                ),
                (
                    "average WPM",
                    format_optional_number(overall_stats.average_wpm),
                ),
                (
                    "highest accuracy",
                    format_optional_percent(overall_stats.highest_accuracy),
                ),
                (
                    "average accuracy",
                    format_optional_percent(overall_stats.average_accuracy),
                ),
            ],
        );
    }

    if let Some(history) = visible_rect(dashboard.history, area, app.stats_scroll_offset) {
        let history_scroll = app
            .stats_scroll_offset
            .saturating_sub(usize::from(dashboard.history.y.saturating_sub(area.y)) + 3);
        render_history_table(frame, history, app, palette, history_scroll);
    }
}

pub fn scroll_max_for_area(app: &App, terminal_area: Rect) -> usize {
    let main = crate::ui::components::shell::layout(terminal_area).main;
    let dashboard = dashboard_layout(main, app.recent_results.len());
    usize::from(dashboard.content_height).saturating_sub(usize::from(main.height))
}

#[derive(Debug, Clone, Copy)]
struct DashboardLayout {
    today: Rect,
    personal_bests: Rect,
    overall: Rect,
    history: Rect,
    content_height: u16,
}

fn dashboard_layout(area: Rect, history_rows: usize) -> DashboardLayout {
    const TODAY_HEIGHT: u16 = 6;
    const PERSONAL_BESTS_HEIGHT: u16 = 8;
    const STACKED_PERSONAL_BESTS_HEIGHT: u16 = 14;
    const OVERALL_HEIGHT: u16 = 6;
    const SECTION_GAP: u16 = 1;
    const MIN_HISTORY_HEIGHT: u16 = 8;

    let personal_bests_height = if area.width < 76 {
        STACKED_PERSONAL_BESTS_HEIGHT
    } else {
        PERSONAL_BESTS_HEIGHT
    };
    let history_height = u16::try_from(history_rows.saturating_add(3))
        .unwrap_or(u16::MAX)
        .max(MIN_HISTORY_HEIGHT);
    let mut y = area.y;
    let today = left_aligned_panel(area, y, TODAY_HEIGHT, 58);
    y = y.saturating_add(TODAY_HEIGHT + SECTION_GAP);
    let personal_bests = Rect::new(area.x, y, area.width, personal_bests_height);
    y = y.saturating_add(personal_bests_height + SECTION_GAP);
    let overall = left_aligned_panel(area, y, OVERALL_HEIGHT, 94);
    y = y.saturating_add(OVERALL_HEIGHT + SECTION_GAP);
    let history = Rect::new(area.x, y, area.width, history_height);
    let content_height = y.saturating_add(history_height).saturating_sub(area.y);

    DashboardLayout {
        today,
        personal_bests,
        overall,
        history,
        content_height,
    }
}

fn left_aligned_panel(area: Rect, y: u16, height: u16, preferred_width: u16) -> Rect {
    Rect::new(area.x, y, area.width.min(preferred_width), height)
}

fn visible_rect(rect: Rect, viewport: Rect, scroll_offset: usize) -> Option<Rect> {
    let screen_y = i32::from(rect.y) - i32::try_from(scroll_offset).unwrap_or(i32::MAX);
    let viewport_top = i32::from(viewport.y);
    let viewport_bottom = i32::from(viewport.bottom());
    let top = screen_y.max(viewport_top);
    let bottom = (screen_y + i32::from(rect.height)).min(viewport_bottom);

    if bottom <= top || rect.width == 0 || viewport.width == 0 {
        return None;
    }

    Some(Rect::new(
        rect.x.max(viewport.x),
        u16::try_from(top).unwrap_or(viewport.y),
        rect.width.min(viewport.width),
        u16::try_from(bottom - top).unwrap_or(0),
    ))
}

fn render_stats_panel(frame: &mut Frame<'_>, area: Rect, title: &str, stats: &[(&str, String)]) {
    let palette = theme::default::palette();
    let block = panel_block(title, palette);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let constraints = vec![Constraint::Ratio(1, stats.len().max(1) as u32); stats.len()];
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(inner);

    for ((label, value), column) in stats.iter().zip(columns.iter()) {
        let label_area = Rect {
            y: column.y.saturating_add(1),
            height: column.height.saturating_sub(1).min(1),
            ..*column
        };
        frame.render_widget(
            Paragraph::new((*label).to_owned())
                .style(Style::default().fg(palette.muted))
                .alignment(Alignment::Center),
            label_area,
        );
        let value_area = Rect {
            y: column.y.saturating_add(2),
            height: column.height.saturating_sub(2),
            ..*column
        };
        frame.render_widget(
            Paragraph::new(value.clone())
                .style(
                    Style::default()
                        .fg(palette.text)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center),
            value_area,
        );
    }
}

fn render_personal_bests(
    frame: &mut Frame<'_>,
    area: Rect,
    app: &App,
    palette: theme::default::Palette,
) {
    let outer = panel_block(" Personal Bests ", palette);
    let inner = outer.inner(area);
    frame.render_widget(outer, area);
    let stack_groups = inner.width < 72;
    let groups = if stack_groups {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner)
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner)
    };

    render_best_group(
        frame,
        groups[0],
        " Time Modes ",
        "time",
        TIME_MODES,
        &english_basic_results(&app.all_results),
        palette,
    );
    render_best_group(
        frame,
        groups[1],
        " Word Modes ",
        "words",
        WORD_MODES,
        &english_basic_results(&app.all_results),
        palette,
    );
}

fn render_best_group(
    frame: &mut Frame<'_>,
    area: Rect,
    title: &str,
    mode_type: &str,
    modes: &[u16],
    results: &[SavedTestResult],
    palette: theme::default::Palette,
) {
    let block = panel_block(title, palette);
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let constraints = vec![Constraint::Ratio(1, modes.len() as u32); modes.len()];
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(inner);

    for (mode, column) in modes.iter().zip(columns.iter()) {
        let label = if mode_type == "time" {
            format!("{mode} seconds")
        } else {
            format!("{mode} words")
        };
        let (wpm, accuracy, date) = personal_best_for_mode(results, mode_type, *mode).map_or_else(
            || ("--".to_owned(), "--".to_owned(), "--".to_owned()),
            |result| {
                (
                    format!("{:.0} WPM", result.wpm),
                    format!("{:.0}%", result.accuracy),
                    result.created_at.format("%d %b %Y").to_string(),
                )
            },
        );
        frame.render_widget(
            Paragraph::new(vec![
                ratatui::text::Line::from(label),
                ratatui::text::Line::from(wpm),
                ratatui::text::Line::from(accuracy),
                ratatui::text::Line::from(date),
            ])
            .style(Style::default().fg(palette.text))
            .alignment(Alignment::Center),
            *column,
        );
    }
}

fn render_history_table(
    frame: &mut Frame<'_>,
    area: Rect,
    app: &App,
    palette: theme::default::Palette,
    row_offset: usize,
) {
    let block = panel_block(" History ", palette);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.recent_results.is_empty() {
        frame.render_widget(
            Paragraph::new("No results yet. Complete a test first.")
                .style(Style::default().fg(palette.muted))
                .alignment(Alignment::Center),
            inner,
        );
        return;
    }

    let Some(visible_rows) = visible_history_rows(inner) else {
        return;
    };
    let rows = app
        .recent_results
        .iter()
        .skip(row_offset)
        .take(visible_rows)
        .map(|result| {
            Row::new(vec![
                Cell::from(history_mode_label(result)),
                Cell::from(format!("{:.0}", result.wpm)),
                Cell::from(format!("{:.0}%", result.accuracy)),
                Cell::from(result.mistakes.to_string()),
                Cell::from(result.created_at.format("%Y-%m-%d %H:%M").to_string()),
            ])
        });
    let header = Row::new(["Mode", "WPM", "Accuracy", "Mistakes", "Date"]).style(
        Style::default()
            .fg(palette.accent)
            .add_modifier(Modifier::BOLD),
    );
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(18),
            Constraint::Percentage(14),
            Constraint::Percentage(18),
            Constraint::Percentage(18),
            Constraint::Percentage(32),
        ],
    )
    .header(header)
    .column_spacing(1)
    .style(Style::default().fg(palette.text));
    frame.render_widget(table, inner);
}

fn visible_history_rows(inner: Rect) -> Option<usize> {
    if inner.width == 0 || inner.height < 2 {
        return None;
    }

    Some(usize::from(inner.height - 1))
}

fn history_mode_label(result: &SavedTestResult) -> String {
    match result.mode_type.as_str() {
        "time" => format!("time {}", result.mode_value),
        "words" => format!("words {}", result.mode_value),
        _ => result.mode_label(),
    }
}

fn panel_block<'a>(title: &'a str, palette: theme::default::Palette) -> Block<'a> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette.muted))
}

fn today_results(results: &[SavedTestResult]) -> Vec<SavedTestResult> {
    let today = Local::now().date_naive();

    results
        .iter()
        .filter(|result| result.created_at.date_naive() == today)
        .cloned()
        .collect()
}

fn english_basic_results(results: &[SavedTestResult]) -> Vec<SavedTestResult> {
    results
        .iter()
        .filter(|result| result.language_mode == "english")
        .cloned()
        .collect()
}

fn personal_best_for_mode<'a>(
    results: &'a [SavedTestResult],
    mode_type: &str,
    mode_value: u16,
) -> Option<&'a SavedTestResult> {
    results
        .iter()
        .filter(|result| result.mode_type == mode_type && result.mode_value == mode_value)
        .max_by(|left, right| compare_personal_best(left, right))
}

fn compare_personal_best(left: &SavedTestResult, right: &SavedTestResult) -> std::cmp::Ordering {
    left.wpm
        .total_cmp(&right.wpm)
        .then_with(|| left.accuracy.total_cmp(&right.accuracy))
        .then_with(|| left.created_at.cmp(&right.created_at))
}

fn format_optional_number(value: Option<f64>) -> String {
    value.map_or_else(|| "-".to_owned(), |value| format!("{value:.0}"))
}

fn format_optional_percent(value: Option<f64>) -> String {
    value.map_or_else(|| "-".to_owned(), |value| format!("{value:.0}%"))
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Local};
    use ratatui::layout::Rect;

    use super::{
        compare_personal_best, dashboard_layout, english_basic_results, history_mode_label,
        personal_best_for_mode, scroll_max_for_area, today_results, SummaryStats,
    };
    use crate::app::App;
    use crate::storage::models::SavedTestResult;

    fn saved_result(
        mode_type: &str,
        mode_value: u16,
        wpm: f64,
        accuracy: f64,
        days_offset: i64,
    ) -> SavedTestResult {
        SavedTestResult {
            id: None,
            mode_type: mode_type.to_owned(),
            mode_value,
            wpm,
            accuracy,
            mistakes: 1,
            correct_chars: 100,
            incorrect_chars: 1,
            total_typed_chars: 101,
            elapsed_seconds: 30.0,
            language_mode: "english".to_owned(),
            created_at: Local::now() + Duration::days(days_offset),
        }
    }

    #[test]
    fn today_results_only_include_local_today() {
        let results = vec![
            saved_result("time", 30, 90.0, 98.0, 0),
            saved_result("time", 30, 70.0, 95.0, -1),
        ];

        let today = today_results(&results);

        assert_eq!(today.len(), 1);
        assert_eq!(today[0].wpm, 90.0);
    }

    #[test]
    fn summary_stats_calculates_totals_and_averages() {
        let results = vec![
            saved_result("time", 30, 90.0, 98.0, 0),
            saved_result("words", 25, 70.0, 96.0, 0),
        ];

        let stats = SummaryStats::from_results(&results);

        assert_eq!(stats.tests_completed, 2);
        assert_eq!(stats.highest_wpm, Some(90.0));
        assert_eq!(stats.average_wpm, Some(80.0));
        assert_eq!(stats.highest_accuracy, Some(98.0));
        assert_eq!(stats.average_accuracy, Some(97.0));
    }

    #[test]
    fn personal_best_prefers_wpm_then_accuracy_then_newer_date() {
        let older = saved_result("time", 30, 90.0, 98.0, -1);
        let newer = saved_result("time", 30, 90.0, 98.0, 0);
        let higher_accuracy = saved_result("time", 30, 90.0, 99.0, -2);
        let higher_wpm = saved_result("time", 30, 91.0, 90.0, -3);

        assert!(compare_personal_best(&newer, &older).is_gt());
        assert!(compare_personal_best(&higher_accuracy, &newer).is_gt());
        assert!(compare_personal_best(&higher_wpm, &higher_accuracy).is_gt());
    }

    #[test]
    fn personal_best_is_grouped_by_exact_mode() {
        let results = vec![
            saved_result("time", 30, 90.0, 98.0, 0),
            saved_result("time", 60, 95.0, 97.0, 0),
            saved_result("words", 30, 120.0, 99.0, 0),
        ];

        let best = personal_best_for_mode(&results, "time", 30).expect("best");

        assert_eq!(best.wpm, 90.0);
        assert_eq!(best.mode_type, "time");
    }

    #[test]
    fn personal_best_source_only_includes_english_basic_results() {
        let mut english = saved_result("time", 30, 90.0, 98.0, 0);
        english.language_mode = "english".to_owned();
        let mut russian = saved_result("time", 30, 140.0, 99.0, 0);
        russian.language_mode = "russian".to_owned();
        let mut english_hard = saved_result("time", 30, 130.0, 99.0, 0);
        english_hard.language_mode = "english-hard".to_owned();

        let results = english_basic_results(&[english, russian, english_hard]);
        let best = personal_best_for_mode(&results, "time", 30).expect("best");

        assert_eq!(best.wpm, 90.0);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn dashboard_panels_remain_ordered_at_compact_sizes() {
        for area in [
            Rect::new(0, 0, 48, 18),
            Rect::new(0, 0, 48, 22),
            Rect::new(0, 0, 100, 30),
        ] {
            let layout = dashboard_layout(area, 15);

            assert!(layout.today.bottom() <= layout.personal_bests.y);
            assert!(layout.personal_bests.bottom() <= layout.overall.y);
            assert!(layout.overall.bottom() <= layout.history.y);
        }
    }

    #[test]
    fn dashboard_scrolls_even_when_history_is_empty() {
        let mut app = App::default();
        app.open_history();

        assert!(scroll_max_for_area(&app, Rect::new(0, 0, 100, 24)) > 0);
    }

    #[test]
    fn history_mode_labels_are_explicit_for_profile_display() {
        assert_eq!(
            history_mode_label(&saved_result("time", 15, 80.0, 99.0, 0)),
            "time 15"
        );
        assert_eq!(
            history_mode_label(&saved_result("words", 10, 80.0, 99.0, 0)),
            "words 10"
        );
    }
}
