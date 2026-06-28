use std::ops::Range;
use std::time::Instant;

use crate::core::scoring::{calculate_accuracy, calculate_wpm};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestMode {
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

    pub fn from_label(input: &str) -> Option<Self> {
        match input.trim() {
            "10" | "10w" => Some(Self::Words(10)),
            "25" | "25w" => Some(Self::Words(25)),
            "50" | "50w" => Some(Self::Words(50)),
            "100" | "100w" => Some(Self::Words(100)),
            "15s" => Some(Self::Time(15)),
            "30s" => Some(Self::Time(30)),
            "60s" => Some(Self::Time(60)),
            "120s" => Some(Self::Time(120)),
            _ => None,
        }
    }

    pub fn mode_type(&self) -> &'static str {
        match self {
            Self::Words(_) => "words",
            Self::Time(_) => "time",
        }
    }

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
    pub total_keystrokes: usize,
    words: Vec<WordProgress>,
    word_ranges: Vec<Range<usize>>,
    current_word_index: usize,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct WordProgress {
    target: String,
    input: String,
    finished: bool,
    missed_chars: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderChar {
    Pending(char),
    Correct(char),
    Wrong(char),
    Missed(char),
    Extra(char),
    Caret(char),
}

impl TypingSession {
    pub fn new(mode: TestMode, target_text: String) -> Self {
        let (words, word_ranges) = word_progress_from_target(&target_text);

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
            total_keystrokes: 0,
            words,
            word_ranges,
            current_word_index: 0,
        }
    }

    pub fn input_char(&mut self, ch: char) {
        if matches!(
            self.status,
            SessionStatus::Finished | SessionStatus::Aborted
        ) {
            return;
        }

        if ch == ' ' {
            self.finish_current_word_from_space();
            return;
        }

        if self.words.is_empty() {
            return;
        }

        self.start_if_waiting();

        let word_index = self
            .current_word_index
            .min(self.words.len().saturating_sub(1));
        let input_index = self.words[word_index].input.chars().count();
        let expected = self.words[word_index].target.chars().nth(input_index);

        self.words[word_index].input.push(ch);
        self.total_keystrokes += 1;

        if expected == Some(ch) {
            self.correct_chars += 1;
        } else {
            self.record_errors(1);
        }

        self.sync_legacy_input();
        self.update_current_index();
        self.update_completion_status();
    }

    pub fn render_chars_for_word(&self, word_index: usize) -> Vec<RenderChar> {
        let Some(word) = self.words.get(word_index) else {
            return Vec::new();
        };

        let target_chars = word.target.chars().collect::<Vec<_>>();
        let input_chars = word.input.chars().collect::<Vec<_>>();
        let mut rendered = Vec::with_capacity(target_chars.len().max(input_chars.len()));
        let active = self.status != SessionStatus::Finished
            && self.status != SessionStatus::Aborted
            && word_index == self.current_word_index;

        for (index, expected) in target_chars.iter().copied().enumerate() {
            if let Some(typed) = input_chars.get(index).copied() {
                if typed == expected {
                    rendered.push(RenderChar::Correct(typed));
                } else {
                    rendered.push(RenderChar::Wrong(typed));
                }
            } else if word.finished && index < input_chars.len() + word.missed_chars {
                rendered.push(RenderChar::Missed(expected));
            } else if active && index == input_chars.len() {
                rendered.push(RenderChar::Caret(expected));
            } else {
                rendered.push(RenderChar::Pending(expected));
            }
        }

        rendered.extend(
            input_chars
                .iter()
                .copied()
                .skip(target_chars.len())
                .map(RenderChar::Extra),
        );

        rendered
    }

    pub fn render_chars_at_target_index(&self, target_index: usize) -> Vec<RenderChar> {
        let Some(word_index) = self.word_index_at_target_index(target_index) else {
            return Vec::new();
        };
        let Some(range) = self.word_ranges.get(word_index) else {
            return Vec::new();
        };

        let offset = target_index.saturating_sub(range.start);
        let mut rendered = self
            .render_chars_for_word(word_index)
            .get(offset)
            .cloned()
            .into_iter()
            .collect::<Vec<_>>();

        if target_index + 1 == range.end {
            let target_len = self.words[word_index].target.chars().count();
            rendered.extend(
                self.render_chars_for_word(word_index)
                    .into_iter()
                    .skip(target_len),
            );
        }

        rendered
    }

    pub fn word_index_at_target_index(&self, target_index: usize) -> Option<usize> {
        self.word_ranges
            .iter()
            .position(|range| range.start <= target_index && target_index < range.end)
    }

    pub fn active_target_index(&self) -> usize {
        self.current_index
    }

    fn start_if_waiting(&mut self) {
        if self.status == SessionStatus::Waiting {
            self.status = SessionStatus::Running;
            self.started_at = Some(Instant::now());
        }
    }

    pub fn backspace(&mut self) {
        if matches!(
            self.status,
            SessionStatus::Finished | SessionStatus::Aborted
        ) {
            return;
        }

        let Some(word) = self.words.get_mut(self.current_word_index) else {
            return;
        };

        if word.input.pop().is_none() {
            return;
        }

        self.sync_legacy_input();
        self.update_current_index();
        self.update_completion_status();
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

    pub fn completed_words(&self) -> usize {
        self.words
            .iter()
            .enumerate()
            .filter(|(index, word)| {
                word.finished
                    || (*index == self.current_word_index
                        && *index + 1 == self.words.len()
                        && word.input == word.target)
            })
            .count()
    }

    pub fn result(&self) -> Option<TestResult> {
        if self.status != SessionStatus::Finished {
            return None;
        }

        let total_typed_chars = self.total_keystrokes;
        let elapsed_seconds = self.elapsed_seconds();

        Some(TestResult {
            mode: self.mode,
            wpm: calculate_wpm(self.correct_chars, elapsed_seconds),
            accuracy: calculate_accuracy(self.mistakes, self.total_keystrokes),
            mistakes: self.mistakes,
            correct_chars: self.correct_chars,
            incorrect_chars: self.incorrect_chars,
            total_typed_chars,
            elapsed_seconds,
        })
    }

    fn update_completion_status(&mut self) {
        if self.last_word_is_exactly_correct() {
            self.finish();
            return;
        }

        match self.mode {
            TestMode::Words(_) | TestMode::Time(_) => {
                if self.words.last().is_some_and(|word| word.finished) {
                    self.finish();
                }
            }
        }

        if matches!(self.mode, TestMode::Time(_)) {
            self.update_time_status();
        }
    }

    fn finish(&mut self) {
        self.status = SessionStatus::Finished;
        self.finished_at = Some(Instant::now());
    }

    fn finish_current_word_from_space(&mut self) {
        if self.words.is_empty() {
            return;
        }

        let word_index = self
            .current_word_index
            .min(self.words.len().saturating_sub(1));
        let input_len = self.words[word_index].input.chars().count();

        if input_len == 0 {
            return;
        }

        self.start_if_waiting();
        self.total_keystrokes += 1;

        let target_len = self.words[word_index].target.chars().count();
        let missed_chars = target_len.saturating_sub(input_len);
        self.words[word_index].missed_chars = missed_chars;
        self.words[word_index].finished = true;
        self.record_errors(missed_chars);

        if word_index + 1 < self.words.len() {
            self.current_word_index += 1;
        }

        self.sync_legacy_input();
        self.update_current_index();
        self.update_completion_status();
    }

    fn record_errors(&mut self, count: usize) {
        self.incorrect_chars += count;
        self.mistakes += count;
    }

    fn last_word_is_exactly_correct(&self) -> bool {
        let Some(last_word) = self.words.last() else {
            return false;
        };

        self.current_word_index + 1 == self.words.len()
            && !last_word.finished
            && last_word.input == last_word.target
    }

    fn sync_legacy_input(&mut self) {
        self.typed_input = self
            .words
            .iter()
            .enumerate()
            .take_while(|(index, word)| *index <= self.current_word_index || word.finished)
            .map(|(_, word)| word.input.as_str())
            .collect::<Vec<_>>()
            .join(" ");
    }

    fn update_current_index(&mut self) {
        if self.words.is_empty() {
            self.current_index = 0;
            return;
        }

        if self.status == SessionStatus::Finished {
            self.current_index = self.target_text.chars().count();
            return;
        }

        let word_index = self
            .current_word_index
            .min(self.words.len().saturating_sub(1));
        let Some(range) = self.word_ranges.get(word_index) else {
            self.current_index = self.target_text.chars().count();
            return;
        };

        let input_len = self.words[word_index].input.chars().count();
        self.current_index = range.start + input_len.min(range.end.saturating_sub(range.start));
    }
}

fn word_progress_from_target(target_text: &str) -> (Vec<WordProgress>, Vec<Range<usize>>) {
    let chars = target_text.chars().collect::<Vec<_>>();
    let mut words = Vec::new();
    let mut ranges = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        while index < chars.len() && chars[index].is_whitespace() {
            index += 1;
        }

        if index >= chars.len() {
            break;
        }

        let start = index;
        while index < chars.len() && !chars[index].is_whitespace() {
            index += 1;
        }

        let target = chars[start..index].iter().collect::<String>();
        words.push(WordProgress {
            target,
            input: String::new(),
            finished: false,
            missed_chars: 0,
        });
        ranges.push(start..index);
    }

    (words, ranges)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{RenderChar, SessionStatus, TestMode, TypingSession};

    #[test]
    fn test_mode_labels_are_correct() {
        assert_eq!(TestMode::Words(10).label(), "10w");
        assert_eq!(TestMode::Words(25).label(), "25w");
        assert_eq!(TestMode::Words(50).label(), "50w");
        assert_eq!(TestMode::Words(100).label(), "100w");
        assert_eq!(TestMode::Time(15).label(), "15s");
        assert_eq!(TestMode::Time(30).label(), "30s");
        assert_eq!(TestMode::Time(60).label(), "60s");
        assert_eq!(TestMode::Time(120).label(), "120s");
    }

    #[test]
    fn test_mode_from_label_parses_supported_modes() {
        assert_eq!(TestMode::from_label("30s"), Some(TestMode::Time(30)));
        assert_eq!(TestMode::from_label("25w"), Some(TestMode::Words(25)));
        assert_eq!(TestMode::from_label("25"), Some(TestMode::Words(25)));
    }

    #[test]
    fn test_mode_from_label_rejects_invalid_labels() {
        assert_eq!(TestMode::from_label("90s"), None);
        assert_eq!(TestMode::from_label("5w"), None);
        assert_eq!(TestMode::from_label("theme"), None);
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
        assert_eq!(session.correct_chars, 2);
    }

    #[test]
    fn backspace_does_not_remove_historical_errors() {
        let mut session = TypingSession::new(TestMode::Time(30), "hello".to_owned());

        session.input_char('h');
        session.input_char('x');
        session.backspace();

        assert_eq!(session.typed_input, "h");
        assert_eq!(session.correct_chars, 1);
        assert_eq!(session.incorrect_chars, 1);
        assert_eq!(session.mistakes, 1);
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
    fn time_mode_finishes_when_last_word_is_exactly_correct() {
        let mut session = TypingSession::new(TestMode::Time(30), "a".to_owned());

        session.input_char('a');

        assert_eq!(session.status, SessionStatus::Finished);
    }

    #[test]
    fn completed_words_counts_words_after_boundary_space() {
        let mut session = TypingSession::new(TestMode::Words(2), "hello world".to_owned());

        for character in "hello".chars() {
            session.input_char(character);
        }
        assert_eq!(session.completed_words(), 0);

        session.input_char(' ');
        assert_eq!(session.completed_words(), 1);
    }

    #[test]
    fn completed_words_counts_final_word_without_trailing_space() {
        let mut session = TypingSession::new(TestMode::Words(2), "hello world".to_owned());

        for character in "hello world".chars() {
            session.input_char(character);
        }

        assert_eq!(session.completed_words(), 2);
    }

    #[test]
    fn space_at_beginning_of_word_is_ignored() {
        let mut session = TypingSession::new(TestMode::Words(2), "form those".to_owned());

        session.input_char(' ');

        assert_eq!(session.status, SessionStatus::Waiting);
        assert_eq!(session.current_index, 0);
        assert_eq!(session.typed_input, "");
        assert_eq!(session.total_keystrokes, 0);
        assert_eq!(session.mistakes, 0);
    }

    #[test]
    fn space_after_partial_word_marks_remaining_letters_missed() {
        let mut session = TypingSession::new(TestMode::Words(2), "through say".to_owned());

        for character in "thr".chars() {
            session.input_char(character);
        }
        session.input_char(' ');

        assert_eq!(session.current_index, 8);
        assert_eq!(session.completed_words(), 1);
        assert_eq!(session.total_keystrokes, 4);
        assert_eq!(session.mistakes, 4);
        assert_eq!(
            session.render_chars_for_word(0),
            vec![
                RenderChar::Correct('t'),
                RenderChar::Correct('h'),
                RenderChar::Correct('r'),
                RenderChar::Missed('o'),
                RenderChar::Missed('u'),
                RenderChar::Missed('g'),
                RenderChar::Missed('h'),
            ]
        );
    }

    #[test]
    fn extra_letters_stay_attached_to_current_word() {
        let mut session = TypingSession::new(TestMode::Words(2), "form those".to_owned());

        for character in "formm".chars() {
            session.input_char(character);
        }

        assert_eq!(session.current_index, 4);
        assert_eq!(session.completed_words(), 0);
        assert_eq!(session.total_keystrokes, 5);
        assert_eq!(session.mistakes, 1);
        assert_eq!(
            session.render_chars_for_word(0),
            vec![
                RenderChar::Correct('f'),
                RenderChar::Correct('o'),
                RenderChar::Correct('r'),
                RenderChar::Correct('m'),
                RenderChar::Extra('m'),
            ]
        );
        assert_eq!(
            session.render_chars_for_word(1),
            vec![
                RenderChar::Pending('t'),
                RenderChar::Pending('h'),
                RenderChar::Pending('o'),
                RenderChar::Pending('s'),
                RenderChar::Pending('e'),
            ]
        );
    }

    #[test]
    fn correct_last_word_auto_finishes() {
        let mut session = TypingSession::new(TestMode::Words(2), "form say".to_owned());

        for character in "form say".chars() {
            session.input_char(character);
        }

        assert_eq!(session.status, SessionStatus::Finished);
        assert_eq!(session.completed_words(), 2);
    }

    #[test]
    fn wrong_last_word_does_not_auto_finish_until_corrected_or_committed() {
        let mut session = TypingSession::new(TestMode::Words(2), "form say".to_owned());

        for character in "form sat".chars() {
            session.input_char(character);
        }

        assert_eq!(session.status, SessionStatus::Running);

        session.backspace();
        session.input_char('y');

        assert_eq!(session.status, SessionStatus::Finished);
    }

    #[test]
    fn wrong_last_word_can_finish_with_space() {
        let mut session = TypingSession::new(TestMode::Words(2), "form say".to_owned());

        for character in "form sat".chars() {
            session.input_char(character);
        }
        session.input_char(' ');

        assert_eq!(session.status, SessionStatus::Finished);
    }

    #[test]
    fn corrected_mistake_still_affects_accuracy() {
        let mut session = TypingSession::new(TestMode::Words(1), "form".to_owned());

        session.input_char('f');
        session.input_char('x');
        session.backspace();
        for character in "orm".chars() {
            session.input_char(character);
        }

        let result = session.result().expect("finished result");
        assert_eq!(session.status, SessionStatus::Finished);
        assert_eq!(session.typed_input, "form");
        assert_eq!(result.mistakes, 1);
        assert!(result.accuracy < 100.0);
        assert_eq!(result.accuracy, 80.0);
    }
}
