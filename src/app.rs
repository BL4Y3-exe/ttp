use crate::command::{parse_command, Command, CommandError};
use crate::core::language_modes::LanguageMode;
use crate::core::test_session::{SessionStatus, TestMode, TestResult, TypingSession};
use crate::core::text_generator::generate_text;
use crate::storage::config::{load_config, save_config, AppConfig};
use crate::storage::database::Database;
use crate::storage::models::SavedTestResult;

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
    pub current_language_mode: LanguageMode,
    pub session: Option<TypingSession>,
    pub last_result: Option<TestResult>,
    pub command_error: Option<String>,
    pub config: AppConfig,
    pub database: Option<Database>,
    pub recent_results: Vec<SavedTestResult>,
    pub all_results: Vec<SavedTestResult>,
    pub stats_scroll_offset: usize,
    stats_max_scroll: usize,
    result_saved: bool,
}

impl Default for App {
    fn default() -> Self {
        let mode = TestMode::default();
        let language_mode = LanguageMode::default();

        Self {
            should_quit: false,
            page: Page::SpeedTest,
            input_mode: InputMode::Normal,
            command_input: String::new(),
            current_mode: mode,
            current_language_mode: language_mode,
            session: Some(TypingSession::new(mode, generate_text(mode, language_mode))),
            last_result: None,
            command_error: None,
            config: AppConfig::default(),
            database: None,
            recent_results: Vec::new(),
            all_results: Vec::new(),
            stats_scroll_offset: 0,
            stats_max_scroll: 0,
            result_saved: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        let mut storage_error = None;

        let config = match load_config() {
            Ok(config) => config,
            Err(error) => {
                storage_error = Some(format!("config error: {error:#}"));
                AppConfig::default()
            }
        };

        let current_mode = TestMode::from_label(&config.last_selected_mode).unwrap_or_default();
        let current_language_mode =
            LanguageMode::from_label(&config.language_mode).unwrap_or_default();

        let database = match Database::open().and_then(|database| {
            database.init()?;
            Ok(database)
        }) {
            Ok(database) => Some(database),
            Err(error) => {
                storage_error = Some(format!("storage error: {error:#}"));
                None
            }
        };

        Self {
            should_quit: false,
            page: Page::SpeedTest,
            input_mode: InputMode::Normal,
            command_input: String::new(),
            current_mode,
            current_language_mode,
            session: Some(TypingSession::new(
                current_mode,
                generate_text(current_mode, current_language_mode),
            )),
            last_result: None,
            command_error: storage_error,
            config,
            database,
            recent_results: Vec::new(),
            all_results: Vec::new(),
            stats_scroll_offset: 0,
            stats_max_scroll: 0,
            result_saved: false,
        }
    }

    pub fn start_new_session(&mut self) {
        self.command_error = None;
        self.session = Some(TypingSession::new(
            self.current_mode,
            generate_text(self.current_mode, self.current_language_mode),
        ));
        self.result_saved = false;
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
                generate_text(self.current_mode, self.current_language_mode),
            ));
            self.result_saved = false;
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

        if self.result_saved || session.status != SessionStatus::Finished {
            return;
        }

        if let Some(result) = session.result() {
            self.current_mode = result.mode;
            self.save_result(&result);
            self.last_result = Some(result);
        }

        self.result_saved = true;
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
                self.config.last_selected_mode = mode.label();
                let config_error = if let Err(error) = save_config(&self.config) {
                    Some(format!("config error: {error:#}"))
                } else {
                    None
                };
                self.command_input.clear();
                self.start_new_session();
                self.command_error = config_error;
            }
            Ok(Command::SetLanguageMode(language_mode)) => {
                self.current_language_mode = language_mode;
                self.config.language_mode = language_mode.label().to_owned();
                let config_error = if let Err(error) = save_config(&self.config) {
                    Some(format!("config error: {error:#}"))
                } else {
                    None
                };
                self.command_input.clear();
                self.start_new_session();
                self.command_error = config_error;
            }
            Err(error) => {
                self.command_input.clear();
                self.command_error = Some(command_error_message(error));
                self.input_mode = InputMode::Normal;
            }
        }
    }

    fn save_result(&mut self, result: &TestResult) {
        let Some(database) = self.database.as_ref() else {
            return;
        };

        let saved_result =
            SavedTestResult::from_test_result_with_language(result, self.current_language_mode);

        if let Err(error) = database.insert_test_result(&saved_result) {
            self.command_error = Some(format!("storage error: {error:#}"));
        }
    }

    pub fn open_history(&mut self) {
        self.input_mode = InputMode::Normal;
        self.page = Page::History;
        self.stats_scroll_offset = 0;
        self.stats_max_scroll = 0;

        let Some(database) = self.database.as_ref() else {
            self.recent_results.clear();
            self.all_results.clear();
            return;
        };

        match (
            database.recent_test_results(15),
            database.all_test_results(),
        ) {
            (Ok(recent_results), Ok(all_results)) => {
                self.recent_results = recent_results;
                self.all_results = all_results;
                self.command_error = None;
            }
            (Err(error), _) | (_, Err(error)) => {
                self.recent_results.clear();
                self.all_results.clear();
                self.command_error = Some(format!("storage error: {error:#}"));
            }
        }
    }

    pub fn scroll_stats_down(&mut self) {
        if self.page == Page::History {
            self.stats_scroll_offset = self
                .stats_scroll_offset
                .saturating_add(1)
                .min(self.stats_max_scroll);
        }
    }

    pub fn scroll_stats_up(&mut self) {
        if self.page == Page::History {
            self.stats_scroll_offset = self.stats_scroll_offset.saturating_sub(1);
        }
    }

    pub fn set_stats_scroll_max(&mut self, max_scroll: usize) {
        self.stats_max_scroll = max_scroll;
        self.stats_scroll_offset = self.stats_scroll_offset.min(max_scroll);
    }
}

fn command_error_message(error: CommandError) -> String {
    match error {
        CommandError::Empty => "empty command".to_owned(),
        CommandError::Unknown(command) => format!("unknown command: {command}"),
    }
}
