use crate::core::test_session::{TestMode, TypingSession};
use crate::core::text_generator::generate_default_language_text;

#[derive(Debug)]
#[allow(dead_code)]
pub struct TypingEngine {
    pub session: TypingSession,
}

impl TypingEngine {
    #[allow(dead_code)]
    pub fn new(mode: TestMode) -> Self {
        Self {
            session: TypingSession::new(mode, generate_default_language_text(mode)),
        }
    }

    #[allow(dead_code)]
    pub fn restart(&mut self) {
        let mode = self.session.mode;
        self.session = TypingSession::new(mode, generate_default_language_text(mode));
    }
}

impl Default for TypingEngine {
    fn default() -> Self {
        Self::new(TestMode::default())
    }
}
