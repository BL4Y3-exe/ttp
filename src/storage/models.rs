#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TestResult {
    pub mode_type: String,
    pub mode_value: u16,
    pub wpm: f64,
    pub accuracy: f64,
    pub mistakes: usize,
}
