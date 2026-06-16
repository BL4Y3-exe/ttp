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
        update_active_session(app);
        terminal.draw(|frame| crate::ui::render(frame, app))?;

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
        KeyCode::Char('s') => app.enter_typing_mode(),
        KeyCode::Char('r') => app.start_new_session(),
        KeyCode::Char('p') => app.open_history(),
        KeyCode::Char(':') => {
            app.enter_command_mode();
        }
        KeyCode::Esc => {
            app.command_error = None;
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
            app.command_error = None;
            app.page = Page::SpeedTest;
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Backspace => {
            if let Some(session) = app.session.as_mut() {
                session.backspace();
            }
        }
        KeyCode::Char(character) => {
            app.ensure_ready_session();
            if let Some(session) = app.session.as_mut() {
                session.input_char(character);
            }
            app.complete_session_if_finished();
        }
        _ => {}
    }
}

fn handle_command_key(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => app.execute_command(),
        KeyCode::Esc => app.cancel_command_mode(),
        KeyCode::Backspace => {
            app.command_input.pop();
        }
        KeyCode::Char(character) => {
            app.command_input.push(character);
        }
        _ => {}
    }
}

fn update_active_session(app: &mut App) {
    let Some(session) = app.session.as_mut() else {
        return;
    };

    session.update_time_status();
    app.complete_session_if_finished();
}
