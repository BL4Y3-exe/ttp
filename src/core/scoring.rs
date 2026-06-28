pub fn calculate_wpm(correct_chars: usize, elapsed_seconds: f64) -> f64 {
    if elapsed_seconds <= 0.0 {
        return 0.0;
    }

    let minutes = elapsed_seconds / 60.0;
    (correct_chars as f64 / 5.0) / minutes
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
    use super::{calculate_accuracy, calculate_wpm};

    #[test]
    fn calculates_character_based_wpm() {
        assert_eq!(calculate_wpm(150, 60.0), 30.0);
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
