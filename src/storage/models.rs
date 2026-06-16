use chrono::{DateTime, Local};

use crate::core::test_session::TestResult;

#[derive(Debug, Clone)]
pub struct SavedTestResult {
    #[allow(dead_code)]
    pub id: Option<i64>,
    pub mode_type: String,
    pub mode_value: u16,
    pub wpm: f64,
    pub accuracy: f64,
    pub mistakes: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
    pub total_typed_chars: usize,
    pub elapsed_seconds: f64,
    pub created_at: DateTime<Local>,
}

impl SavedTestResult {
    pub fn from_test_result(result: &TestResult) -> Self {
        Self {
            id: None,
            mode_type: result.mode.mode_type().to_owned(),
            mode_value: result.mode.mode_value(),
            wpm: result.wpm,
            accuracy: result.accuracy,
            mistakes: result.mistakes,
            correct_chars: result.correct_chars,
            incorrect_chars: result.incorrect_chars,
            total_typed_chars: result.total_typed_chars,
            elapsed_seconds: result.elapsed_seconds,
            created_at: Local::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SavedTestResult;
    use crate::core::test_session::{TestMode, TestResult};

    #[test]
    fn saved_test_result_can_be_created_from_test_result() {
        let result = TestResult {
            mode: TestMode::Time(30),
            wpm: 80.0,
            accuracy: 95.0,
            mistakes: 3,
            correct_chars: 200,
            incorrect_chars: 3,
            total_typed_chars: 203,
            elapsed_seconds: 30.0,
        };

        let saved = SavedTestResult::from_test_result(&result);

        assert_eq!(saved.id, None);
        assert_eq!(saved.mode_type, "time");
        assert_eq!(saved.mode_value, 30);
        assert_eq!(saved.wpm, 80.0);
        assert_eq!(saved.accuracy, 95.0);
        assert_eq!(saved.mistakes, 3);
        assert_eq!(saved.correct_chars, 200);
        assert_eq!(saved.incorrect_chars, 3);
        assert_eq!(saved.total_typed_chars, 203);
        assert_eq!(saved.elapsed_seconds, 30.0);
    }
}
