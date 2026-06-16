#[allow(dead_code)]
pub fn calculate_wpm(correct_chars: usize, elapsed_seconds: f64) -> f64 {
    if elapsed_seconds <= 0.0 {
        return 0.0;
    }

    let minutes = elapsed_seconds / 60.0;
    (correct_chars as f64 / 5.0) / minutes
}

#[allow(dead_code)]
pub fn calculate_accuracy(correct_chars: usize, total_typed_chars: usize) -> f64 {
    if total_typed_chars == 0 {
        return 0.0;
    }

    (correct_chars as f64 / total_typed_chars as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use super::{calculate_accuracy, calculate_wpm};

    #[test]
    fn calculates_character_based_wpm() {
        assert_eq!(calculate_wpm(150, 60.0), 30.0);
    }

    #[test]
    fn accuracy_is_zero_without_input() {
        assert_eq!(calculate_accuracy(0, 0), 0.0);
    }
}
