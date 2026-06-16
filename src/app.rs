use crate::core::test_session::{TestMode, TypingSession};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    SpeedTest,
    #[allow(dead_code)]
    Result,
    History,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Typing,
    Command,
}

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub page: Page,
    pub input_mode: InputMode,
    pub command_input: String,
    pub current_mode: TestMode,
    pub session: Option<TypingSession>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: false,
            page: Page::SpeedTest,
            input_mode: InputMode::Normal,
            command_input: String::new(),
            current_mode: TestMode::default(),
            session: None,
        }
    }
}

impl App {
    pub fn input_mode_label(&self) -> &'static str {
        match self.input_mode {
            InputMode::Normal => "normal",
            InputMode::Typing => "typing",
            InputMode::Command => "command",
        }
    }
}
