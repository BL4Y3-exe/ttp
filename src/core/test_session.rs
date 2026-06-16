use std::time::Instant;

use crate::core::scoring::{calculate_accuracy, calculate_wpm};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestMode {
    #[allow(dead_code)]
    Words(u16),
    Time(u16),
}

impl Default for TestMode {
    fn default() -> Self {
        Self::Time(30)
    }
}

impl TestMode {
    pub fn label(&self) -> String {
        match self {
            Self::Words(words) => format!("{words}w"),
            Self::Time(seconds) => format!("{seconds}s"),
        }
    }

    #[allow(dead_code)]
    pub fn mode_type(&self) -> &'static str {
        match self {
            Self::Words(_) => "words",
            Self::Time(_) => "time",
        }
    }

    #[allow(dead_code)]
    pub fn mode_value(&self) -> u16 {
        match self {
            Self::Words(value) | Self::Time(value) => *value,
        }
    }

    #[allow(dead_code)]
    pub fn is_supported(&self) -> bool {
        matches!(
            self,
            Self::Words(10 | 25 | 50 | 100) | Self::Time(15 | 30 | 60 | 120)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStatus {
    Waiting,
    Running,
    Finished,
    Aborted,
}

#[derive(Debug)]
pub struct TypingSession {
    pub mode: TestMode,
    pub target_text: String,
    pub typed_input: String,
    pub current_index: usize,
    pub status: SessionStatus,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
    pub mistakes: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TestResult {
    pub mode: TestMode,
    pub wpm: f64,
    pub accuracy: f64,
    pub mistakes: usize,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
    pub total_typed_chars: usize,
    pub elapsed_seconds: f64,
}

impl TypingSession {
    pub fn new(mode: TestMode, target_text: String) -> Self {
        Self {
            mode,
            target_text,
            typed_input: String::new(),
            current_index: 0,
            status: SessionStatus::Waiting,
            started_at: None,
            finished_at: None,
            mistakes: 0,
            correct_chars: 0,
            incorrect_chars: 0,
        }
    }

    pub fn input_char(&mut self, ch: char) {
        if matches!(
            self.status,
            SessionStatus::Finished | SessionStatus::Aborted
        ) {
            return;
        }

        if self.status == SessionStatus::Waiting {
            self.status = SessionStatus::Running;
            self.started_at = Some(Instant::now());
        }

        let expected = self.target_text.chars().nth(self.current_index);
        self.typed_input.push(ch);

        if expected == Some(ch) {
            self.correct_chars += 1;
        } else {
            self.incorrect_chars += 1;
            self.mistakes += 1;
        }

        self.current_index += 1;
        self.update_completion_status();
    }

    pub fn backspace(&mut self) {
        if matches!(
            self.status,
            SessionStatus::Finished | SessionStatus::Aborted
        ) {
            return;
        }

        if self.typed_input.pop().is_none() {
            return;
        }

        self.current_index = self.current_index.saturating_sub(1);
        self.recalculate_counts();
    }

    pub fn abort(&mut self) {
        self.status = SessionStatus::Aborted;

        if self.started_at.is_some() {
            self.finished_at = Some(Instant::now());
        }
    }

    pub fn update_time_status(&mut self) {
        if !matches!(self.status, SessionStatus::Running) {
            return;
        }

        if let TestMode::Time(seconds) = self.mode {
            if self.elapsed_seconds() >= f64::from(seconds) {
                self.finish();
            }
        }
    }

    pub fn elapsed_seconds(&self) -> f64 {
        let Some(started_at) = self.started_at else {
            return 0.0;
        };

        let end = self.finished_at.unwrap_or_else(Instant::now);
        end.duration_since(started_at).as_secs_f64()
    }

    pub fn result(&self) -> Option<TestResult> {
        if self.status != SessionStatus::Finished {
            return None;
        }

        let total_typed_chars = self.typed_input.chars().count();
        let elapsed_seconds = self.elapsed_seconds();

        Some(TestResult {
            mode: self.mode,
            wpm: calculate_wpm(self.correct_chars, elapsed_seconds),
            accuracy: calculate_accuracy(self.correct_chars, total_typed_chars),
            mistakes: self.mistakes,
            correct_chars: self.correct_chars,
            incorrect_chars: self.incorrect_chars,
            total_typed_chars,
            elapsed_seconds,
        })
    }

    fn update_completion_status(&mut self) {
        match self.mode {
            TestMode::Words(_) => {
                if self.current_index >= self.target_text.chars().count() {
                    self.finish();
                }
            }
            TestMode::Time(_) => self.update_time_status(),
        }
    }

    fn finish(&mut self) {
        self.status = SessionStatus::Finished;
        self.finished_at = Some(Instant::now());
    }

    fn recalculate_counts(&mut self) {
        self.correct_chars = 0;
        self.incorrect_chars = 0;
        self.mistakes = 0;

        for (index, typed) in self.typed_input.chars().enumerate() {
            if self.target_text.chars().nth(index) == Some(typed) {
                self.correct_chars += 1;
            } else {
                self.incorrect_chars += 1;
                self.mistakes += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{SessionStatus, TestMode, TypingSession};

    #[test]
    fn test_mode_labels_are_correct() {
        assert_eq!(TestMode::Words(10).label(), "10w");
        assert_eq!(TestMode::Words(25).label(), "25w");
        assert_eq!(TestMode::Time(30).label(), "30s");
        assert_eq!(TestMode::Time(120).label(), "120s");
    }

    #[test]
    fn test_mode_support_matches_v0_1_scope() {
        assert!(TestMode::Words(10).is_supported());
        assert!(TestMode::Words(100).is_supported());
        assert!(TestMode::Time(15).is_supported());
        assert!(TestMode::Time(120).is_supported());
        assert!(!TestMode::Words(5).is_supported());
        assert!(!TestMode::Time(45).is_supported());
    }

    #[test]
    fn new_session_starts_waiting() {
        let session = TypingSession::new(TestMode::default(), "hello".to_owned());

        assert_eq!(session.status, SessionStatus::Waiting);
        assert_eq!(session.typed_input, "");
        assert_eq!(session.current_index, 0);
        assert_eq!(session.started_at, None);
        assert_eq!(session.finished_at, None);
        assert_eq!(session.mistakes, 0);
        assert_eq!(session.correct_chars, 0);
        assert_eq!(session.incorrect_chars, 0);
    }

    #[test]
    fn first_character_starts_session() {
        let mut session = TypingSession::new(TestMode::Time(30), "hello".to_owned());

        session.input_char('h');

        assert_eq!(session.status, SessionStatus::Running);
        assert!(session.started_at.is_some());
    }

    #[test]
    fn correct_character_increments_correct_chars() {
        let mut session = TypingSession::new(TestMode::Time(30), "hello".to_owned());

        session.input_char('h');

        assert_eq!(session.correct_chars, 1);
        assert_eq!(session.incorrect_chars, 0);
        assert_eq!(session.mistakes, 0);
    }

    #[test]
    fn incorrect_character_increments_mistakes_and_incorrect_chars() {
        let mut session = TypingSession::new(TestMode::Time(30), "hello".to_owned());

        session.input_char('x');

        assert_eq!(session.correct_chars, 0);
        assert_eq!(session.incorrect_chars, 1);
        assert_eq!(session.mistakes, 1);
    }

    #[test]
    fn backspace_removes_previous_input() {
        let mut session = TypingSession::new(TestMode::Time(30), "hello".to_owned());

        session.input_char('h');
        session.input_char('e');
        session.backspace();

        assert_eq!(session.typed_input, "h");
        assert_eq!(session.current_index, 1);
        assert_eq!(session.correct_chars, 1);
    }

    #[test]
    fn backspace_recalculates_counters_correctly() {
        let mut session = TypingSession::new(TestMode::Time(30), "hello".to_owned());

        session.input_char('h');
        session.input_char('x');
        session.backspace();

        assert_eq!(session.typed_input, "h");
        assert_eq!(session.correct_chars, 1);
        assert_eq!(session.incorrect_chars, 0);
        assert_eq!(session.mistakes, 0);
    }

    #[test]
    fn word_mode_finishes_after_target_text_is_completed() {
        let mut session = TypingSession::new(TestMode::Words(1), "hi".to_owned());

        session.input_char('h');
        assert_eq!(session.status, SessionStatus::Running);
        session.input_char('i');

        assert_eq!(session.status, SessionStatus::Finished);
        assert!(session.finished_at.is_some());
        assert!(session.result().is_some());
    }

    #[test]
    fn abort_sets_status_to_aborted() {
        let mut session = TypingSession::new(TestMode::Time(30), "hello".to_owned());

        session.input_char('h');
        session.abort();

        assert_eq!(session.status, SessionStatus::Aborted);
        assert!(session.finished_at.is_some());
        assert!(session.result().is_none());
    }

    #[test]
    fn time_mode_finishes_when_duration_has_elapsed() {
        let mut session = TypingSession::new(TestMode::Time(15), "hello".to_owned());
        session.input_char('h');
        session.started_at = Some(Instant::now() - Duration::from_secs(16));

        session.update_time_status();

        assert_eq!(session.status, SessionStatus::Finished);
    }

    #[test]
    fn time_mode_does_not_finish_when_target_text_is_exhausted() {
        let mut session = TypingSession::new(TestMode::Time(30), "a".to_owned());

        session.input_char('a');

        assert_eq!(session.status, SessionStatus::Running);
    }
}
