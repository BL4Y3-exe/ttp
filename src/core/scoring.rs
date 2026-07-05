pub fn calculate_wpm(correct_chars: usize, elapsed_seconds: f64) -> f64 {
    if elapsed_seconds <= 0.0 {
        return 0.0;
    }

    let minutes = elapsed_seconds / 60.0;
    (correct_chars as f64 / 5.0) / minutes
}

pub fn calculate_correct_chars(target_words: &[&str], typed_words: &[&str]) -> usize {
    target_words
        .iter()
        .enumerate()
        .filter(|(index, target)| {
            typed_words
                .get(*index)
                .is_some_and(|typed| typed == *target)
        })
        .map(|(index, target)| {
            let trailing_space = usize::from(index + 1 < target_words.len());

            target.chars().count() + trailing_space
        })
        .sum()
}

pub fn calculate_accuracy(total_errors: usize, total_keystrokes: usize) -> f64 {
    if total_keystrokes == 0 {
        return 100.0;
    }

    let accuracy = (1.0 - (total_errors as f64 / total_keystrokes as f64)) * 100.0;
    accuracy.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::{calculate_accuracy, calculate_correct_chars, calculate_wpm};

    #[test]
    fn calculates_character_based_wpm() {
        assert_eq!(calculate_wpm(150, 60.0), 30.0);
    }

    #[test]
    fn all_correct_words_include_spaces() {
        let target = ["hello", "world"];
        let typed = ["hello", "world"];
        let correct_chars = calculate_correct_chars(&target, &typed);

        assert_eq!(correct_chars, 11);
        assert_eq!(calculate_wpm(correct_chars, 6.0), 22.0);
    }

    #[test]
    fn final_word_counts_without_trailing_space() {
        let target = ["hello", "world"];
        let typed = ["hello", "world"];

        assert_eq!(calculate_correct_chars(&target, &typed), 11);
    }

    #[test]
    fn corrected_word_counts_when_final_input_matches() {
        let target = ["form"];
        let typed = ["form"];

        assert_eq!(calculate_correct_chars(&target, &typed), 4);
    }

    #[test]
    fn incorrect_word_does_not_contribute_characters() {
        let target = ["hello", "world"];
        let typed = ["hello", "word"];

        assert_eq!(calculate_correct_chars(&target, &typed), 6);
    }

    #[test]
    fn spaces_count_only_after_correct_words_followed_by_target_words() {
        let target = ["one", "two", "three"];
        let typed = ["one", "too", "three"];

        assert_eq!(calculate_correct_chars(&target, &typed), 9);
    }

    #[test]
    fn elapsed_seconds_normalizes_wpm_to_minutes() {
        assert_eq!(calculate_wpm(10, 30.0), 4.0);
    }

    #[test]
    fn accuracy_is_perfect_without_input() {
        assert_eq!(calculate_accuracy(0, 0), 100.0);
    }

    #[test]
    fn calculates_accuracy_percentage() {
        assert_eq!(calculate_accuracy(5, 50), 90.0);
    }

    #[test]
    fn clamps_accuracy_to_zero() {
        assert_eq!(calculate_accuracy(12, 10), 0.0);
    }

    #[test]
    fn wpm_is_zero_without_elapsed_time() {
        assert_eq!(calculate_wpm(100, 0.0), 0.0);
    }
}
