use crate::command::{parse_command, Command, CommandError};
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
    pub command_error: Option<String>,
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
            command_error: None,
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
        self.command_error = None;
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
        self.command_error = None;
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

    pub fn enter_command_mode(&mut self) {
        self.command_input.clear();
        self.command_error = None;
        self.input_mode = InputMode::Command;
    }

    pub fn cancel_command_mode(&mut self) {
        self.command_input.clear();
        self.command_error = None;
        self.input_mode = InputMode::Normal;
    }

    pub fn execute_command(&mut self) {
        let input = self.command_input.clone();

        match parse_command(&input) {
            Ok(Command::SetMode(mode)) => {
                self.current_mode = mode;
                self.command_input.clear();
                self.command_error = None;
                self.start_new_session();
            }
            Err(error) => {
                self.command_input.clear();
                self.command_error = Some(command_error_message(error));
                self.input_mode = InputMode::Normal;
            }
        }
    }
}

fn command_error_message(error: CommandError) -> String {
    match error {
        CommandError::Empty => "empty command".to_owned(),
        CommandError::Unknown(command) => format!("unknown command: {command}"),
    }
}
