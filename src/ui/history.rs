use chrono::Local;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Paragraph, Wrap};
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
        Paragraph::new("ttp\nprofile")
            .style(
                Style::default()
                    .fg(palette.accent)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(profile_text(&app.all_results, &app.recent_results))
            .style(Style::default().fg(palette.text))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: false }),
        chunks[1],
    );

    frame.render_widget(
        Paragraph::new(format!("mode: {}", app.input_mode_label()))
            .style(Style::default().fg(palette.muted))
            .alignment(Alignment::Center),
        chunks[2],
    );
}

fn profile_text(all_results: &[SavedTestResult], recent_results: &[SavedTestResult]) -> String {
    profile_lines(all_results, recent_results).join("\n")
}

fn profile_lines(
    all_results: &[SavedTestResult],
    recent_results: &[SavedTestResult],
) -> Vec<String> {
    let today_results = today_results(all_results);
    let today_stats = SummaryStats::from_results(&today_results);
    let overall_stats = SummaryStats::from_results(all_results);
    let mut lines = Vec::new();

    lines.push("Today's statistics".to_owned());
    lines.extend(summary_lines(
        &today_stats,
        &[
            SummaryField::TestsCompleted,
            SummaryField::HighestWpm,
            SummaryField::AverageWpm,
        ],
    ));
    lines.push(String::new());

    lines.push("Personal bests".to_owned());
    lines.push("Time modes".to_owned());
    lines.extend(personal_best_lines(all_results, "time", TIME_MODES));
    lines.push(String::new());
    lines.push("Word count modes".to_owned());
    lines.extend(personal_best_lines(all_results, "words", WORD_MODES));
    lines.push(String::new());

    lines.push("Overall statistics".to_owned());
    lines.extend(summary_lines(
        &overall_stats,
        &[
            SummaryField::TestsCompleted,
            SummaryField::HighestWpm,
            SummaryField::AverageWpm,
            SummaryField::HighestAccuracy,
            SummaryField::AverageAccuracy,
        ],
    ));
    lines.push(String::new());

    lines.push("History".to_owned());
    lines.extend(history_lines(recent_results));

    lines
}

#[derive(Debug, Clone, Copy)]
enum SummaryField {
    TestsCompleted,
    HighestWpm,
    AverageWpm,
    HighestAccuracy,
    AverageAccuracy,
}

fn summary_lines(stats: &SummaryStats, fields: &[SummaryField]) -> Vec<String> {
    fields
        .iter()
        .map(|field| match field {
            SummaryField::TestsCompleted => {
                format!("Tests completed: {}", stats.tests_completed)
            }
            SummaryField::HighestWpm => {
                format!("Highest WPM: {}", format_optional_number(stats.highest_wpm))
            }
            SummaryField::AverageWpm => {
                format!("Average WPM: {}", format_optional_number(stats.average_wpm))
            }
            SummaryField::HighestAccuracy => {
                format!(
                    "Highest accuracy: {}",
                    format_optional_percent(stats.highest_accuracy)
                )
            }
            SummaryField::AverageAccuracy => {
                format!(
                    "Average accuracy: {}",
                    format_optional_percent(stats.average_accuracy)
                )
            }
        })
        .collect()
}

fn today_results(results: &[SavedTestResult]) -> Vec<SavedTestResult> {
    let today = Local::now().date_naive();

    results
        .iter()
        .filter(|result| result.created_at.date_naive() == today)
        .cloned()
        .collect()
}

fn personal_best_lines(
    results: &[SavedTestResult],
    mode_type: &str,
    mode_values: &[u16],
) -> Vec<String> {
    mode_values
        .iter()
        .map(|mode_value| {
            let label = match mode_type {
                "time" => format!("{mode_value}s"),
                "words" => format!("{mode_value}w"),
                _ => format!("{mode_value}{mode_type}"),
            };

            match personal_best_for_mode(results, mode_type, *mode_value) {
                Some(result) => format!(
                    "{:<4} {:>5.0} WPM   {:>3.0}% acc   {}",
                    label,
                    result.wpm,
                    result.accuracy,
                    result.created_at.format("%Y-%m-%d")
                ),
                None => format!("{:<4}     - WPM     - acc   -", label),
            }
        })
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

fn history_lines(results: &[SavedTestResult]) -> Vec<String> {
    if results.is_empty() {
        return vec!["No results yet. Complete a test first.".to_owned()];
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

    lines
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

    use super::{
        compare_personal_best, history_lines, personal_best_for_mode, profile_text, today_results,
        SummaryStats,
    };
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
    fn empty_profile_formatting_does_not_panic() {
        let text = profile_text(&[], &[]);

        assert!(text.contains("Today's statistics"));
        assert!(text.contains("Tests completed: 0"));
        assert!(text.contains("Highest WPM: -"));
        assert!(text.contains("No results yet."));
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
    fn history_keeps_existing_recent_format() {
        let results = vec![saved_result("words", 25, 80.0, 99.0, 0)];
        let text = history_lines(&results).join("\n");

        assert!(text.contains("Mode"));
        assert!(text.contains("25w"));
        assert!(text.contains("80"));
        assert!(text.contains("99%"));
    }
}
