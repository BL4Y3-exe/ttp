use std::io::{self, Stdout};
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::{App, InputMode, Page};
use crate::core::test_session::{SessionStatus, TypingSession};
use crate::core::text_generator::generate_text;

type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn run(mut app: App) -> Result<()> {
    let mut terminal = init_terminal()?;
    let result = run_app(&mut terminal, &mut app);
    restore_terminal(&mut terminal)?;

    result
}

fn init_terminal() -> Result<Tui> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

fn restore_terminal(terminal: &mut Tui) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app(terminal: &mut Tui, app: &mut App) -> Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| crate::ui::render(frame, app))?;
        update_active_session(app);

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key(app, key.code);
                }
            }
        }
    }

    Ok(())
}

fn handle_key(app: &mut App, key: KeyCode) {
    match app.input_mode {
        InputMode::Normal => handle_normal_key(app, key),
        InputMode::Typing => handle_typing_key(app, key),
        InputMode::Command => handle_command_key(app, key),
    }
}

fn handle_normal_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('s') | KeyCode::Char('r') => {
            if key == KeyCode::Char('r') || app.session.is_none() {
                start_new_session(app);
            }
            app.page = Page::SpeedTest;
            app.input_mode = InputMode::Typing;
        }
        KeyCode::Char('p') => {
            app.page = Page::History;
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Char(':') => {
            app.command_input.clear();
            app.input_mode = InputMode::Command;
        }
        KeyCode::Esc => {
            app.page = Page::SpeedTest;
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

fn handle_typing_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            if let Some(session) = app.session.as_mut() {
                session.abort();
            }
            app.page = Page::SpeedTest;
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Backspace => {
            if let Some(session) = app.session.as_mut() {
                session.backspace();
            }
        }
        KeyCode::Char(character) => {
            if let Some(session) = app.session.as_mut() {
                session.input_char(character);

                if session.status == SessionStatus::Finished {
                    let _ = session.result();
                    app.page = Page::Result;
                    app.input_mode = InputMode::Normal;
                }
            }
        }
        _ => {}
    }
}

fn handle_command_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter | KeyCode::Esc => {
            app.command_input.clear();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Backspace => {
            app.command_input.pop();
        }
        KeyCode::Char(character) => {
            app.command_input.push(character);
        }
        _ => {}
    }
}

fn start_new_session(app: &mut App) {
    let target_text = generate_text(app.current_mode);
    app.session = Some(TypingSession::new(app.current_mode, target_text));
}

fn update_active_session(app: &mut App) {
    let Some(session) = app.session.as_mut() else {
        return;
    };

    session.update_time_status();

    if app.input_mode == InputMode::Typing && session.status == SessionStatus::Finished {
        let _ = session.result();
        app.page = Page::Result;
        app.input_mode = InputMode::Normal;
    }
}
