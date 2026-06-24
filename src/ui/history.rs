use chrono::Local;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
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
    let dashboard = dashboard_layout(area);
    let palette = theme::default::palette();
    let today_stats = SummaryStats::from_results(&today_results(&app.all_results));
    let overall_stats = SummaryStats::from_results(&app.all_results);

    render_stats_panel(
        frame,
        dashboard.today,
        " Today's statistics ",
        &[
            ("tests completed", today_stats.tests_completed.to_string()),
            (
                "highest wpm",
                format_optional_number(today_stats.highest_wpm),
            ),
            (
                "average wpm",
                format_optional_number(today_stats.average_wpm),
            ),
        ],
    );

    render_personal_bests(frame, dashboard.personal_bests, app, palette);

    render_stats_panel(
        frame,
        dashboard.overall,
        " Overall statistics ",
        &[
            ("tests completed", overall_stats.tests_completed.to_string()),
            (
                "highest wpm",
                format_optional_number(overall_stats.highest_wpm),
            ),
            (
                "average wpm",
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

    render_history_table(frame, dashboard.history, app, palette);
}

pub fn scroll_max_for_height(app: &App, terminal_height: u16) -> usize {
    let main_height = terminal_height.saturating_sub(6);
    let history = dashboard_layout(Rect::new(0, 0, 80, main_height)).history;
    let inner_height = Block::default().borders(Borders::ALL).inner(history).height;
    let visible_rows = usize::from(inner_height.saturating_sub(1));
    app.recent_results.len().saturating_sub(visible_rows)
}

#[derive(Debug, Clone, Copy)]
struct DashboardLayout {
    today: Rect,
    personal_bests: Rect,
    overall: Rect,
    history: Rect,
}

fn dashboard_layout(area: Rect) -> DashboardLayout {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(1),
            Constraint::Length(10),
            Constraint::Length(1),
            Constraint::Length(6),
            Constraint::Length(1),
            Constraint::Min(5),
        ])
        .split(area);

    DashboardLayout {
        today: chunks[0],
        personal_bests: chunks[2],
        overall: chunks[4],
        history: chunks[6],
    }
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
    let outer = panel_block(" Personal bests ", palette);
    let inner = outer.inner(area);
    frame.render_widget(outer, area);
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner);

    render_best_group(
        frame,
        columns[0],
        " Time modes ",
        "time",
        TIME_MODES,
        &app.all_results,
        palette,
    );
    render_best_group(
        frame,
        columns[1],
        " Word modes ",
        "words",
        WORD_MODES,
        &app.all_results,
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
            format!("{mode}s")
        } else {
            format!("{mode}w")
        };
        let (wpm, accuracy, date) = personal_best_for_mode(results, mode_type, *mode).map_or_else(
            || ("-".to_owned(), "-".to_owned(), "-".to_owned()),
            |result| {
                (
                    format!("{:.0} wpm", result.wpm),
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

    let visible_rows = usize::from(inner.height.saturating_sub(1));
    let rows = app
        .recent_results
        .iter()
        .skip(app.stats_scroll_offset)
        .take(visible_rows)
        .map(|result| {
            Row::new(vec![
                Cell::from(result.mode_label()),
                Cell::from(format!("{:.0}", result.wpm)),
                Cell::from(format!("{:.0}%", result.accuracy)),
                Cell::from(result.mistakes.to_string()),
                Cell::from(result.created_at.format("%Y-%m-%d %H:%M").to_string()),
            ])
        });
    let header = Row::new(["mode", "wpm", "accuracy", "mistakes", "date"]).style(
        Style::default()
            .fg(palette.accent)
            .add_modifier(Modifier::BOLD),
    );
    let table = Table::new(
        rows,
        [
            Constraint::Length(9),
            Constraint::Length(7),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Min(16),
        ],
    )
    .header(header)
    .column_spacing(1)
    .style(Style::default().fg(palette.text));
    frame.render_widget(table, inner);
}

fn panel_block<'a>(title: &'a str, palette: theme::default::Palette) -> Block<'a> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
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

    use super::{compare_personal_best, personal_best_for_mode, today_results, SummaryStats};
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
}
