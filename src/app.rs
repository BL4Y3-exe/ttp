use crate::core::test_session::{SessionStatus, TestMode, TestResult, TypingSession};
use crate::core::text_generator::generate_text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    SpeedTest,
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
    pub last_result: Option<TestResult>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: false,
            page: Page::SpeedTest,
            input_mode: InputMode::Normal,
            command_input: String::new(),
            current_mode: TestMode::default(),
            session: Some(TypingSession::new(
                TestMode::default(),
                generate_text(TestMode::default()),
            )),
            last_result: None,
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

    pub fn start_new_session(&mut self) {
        self.session = Some(TypingSession::new(
            self.current_mode,
            generate_text(self.current_mode),
        ));
        self.page = Page::SpeedTest;
        self.input_mode = InputMode::Typing;
    }

    pub fn ensure_ready_session(&mut self) {
        let needs_new_session = self.session.as_ref().is_none_or(|session| {
            matches!(
                session.status,
                SessionStatus::Finished | SessionStatus::Aborted
            )
        });

        if needs_new_session {
            self.session = Some(TypingSession::new(
                self.current_mode,
                generate_text(self.current_mode),
            ));
        }
    }

    pub fn enter_typing_mode(&mut self) {
        self.ensure_ready_session();
        self.page = Page::SpeedTest;
        self.input_mode = InputMode::Typing;
    }

    pub fn complete_session_if_finished(&mut self) {
        let Some(session) = self.session.as_ref() else {
            return;
        };

        if session.status != SessionStatus::Finished {
            return;
        }

        if let Some(result) = session.result() {
            self.current_mode = result.mode;
            self.last_result = Some(result);
        }

        self.page = Page::Result;
        self.input_mode = InputMode::Normal;
    }
}
