use rand::thread_rng;
use rand::Rng;

use crate::core::language_modes::{LanguageMode, WordList};
use crate::core::test_session::TestMode;

pub fn generate_text(mode: TestMode, language_mode: LanguageMode) -> String {
    let word_count = generated_word_count(mode);
    generate_words(word_count, language_mode.word_list()).join(" ")
}

#[allow(dead_code)]
pub fn generate_default_language_text(mode: TestMode) -> String {
    generate_text(mode, LanguageMode::default())
}

fn generated_word_count(mode: TestMode) -> usize {
    match mode {
        TestMode::Words(words) => usize::from(words),
        TestMode::Time(15) => 80,
        TestMode::Time(30) => 160,
        TestMode::Time(60) => 320,
        TestMode::Time(120) => 640,
        TestMode::Time(seconds) => usize::from(seconds) * 5,
    }
}

fn generate_words(word_count: usize, word_list: WordList) -> Vec<&'static str> {
    let mut rng = thread_rng();

    (0..word_count)
        .map(|_| {
            let index = rng.gen_range(0..word_list.size());
            word_list.word_at(index)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::generate_text;
    use crate::core::language_modes::{Language, LanguageMode, LanguageVariant};
    use crate::core::test_session::TestMode;

    #[test]
    fn generate_text_returns_correct_word_count_for_word_modes() {
        for words in [10, 25, 50, 100] {
            assert_eq!(
                generate_text(TestMode::Words(words), LanguageMode::default())
                    .split_whitespace()
                    .count(),
                usize::from(words)
            );
        }
    }

    #[test]
    fn generate_text_returns_enough_words_for_time_modes() {
        assert_eq!(
            generate_text(TestMode::Time(15), LanguageMode::default())
                .split_whitespace()
                .count(),
            80
        );
        assert_eq!(
            generate_text(TestMode::Time(30), LanguageMode::default())
                .split_whitespace()
                .count(),
            160
        );
        assert_eq!(
            generate_text(TestMode::Time(60), LanguageMode::default())
                .split_whitespace()
                .count(),
            320
        );
        assert_eq!(
            generate_text(TestMode::Time(120), LanguageMode::default())
                .split_whitespace()
                .count(),
            640
        );
    }

    #[test]
    fn english_basic_generated_text_is_lowercase_ascii_words_only() {
        let text = generate_text(TestMode::Words(100), LanguageMode::default());

        assert!(text.chars().all(|ch| ch.is_ascii_lowercase() || ch == ' '));
    }

    #[test]
    fn non_english_language_modes_generate_unicode_text() {
        for language_mode in [
            LanguageMode::new(Language::Russian, LanguageVariant::Basic),
            LanguageMode::new(Language::Azerbaijani, LanguageVariant::Extended),
            LanguageMode::new(Language::Spanish, LanguageVariant::Hard),
        ] {
            let text = generate_text(TestMode::Words(25), language_mode);

            assert_eq!(text.split_whitespace().count(), 25);
            assert!(text.chars().any(|character| !character.is_ascii()));
        }
    }
}
