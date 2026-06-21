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
        update_stats_scroll_bounds(app, terminal.size()?.height);
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

fn update_stats_scroll_bounds(app: &mut App, terminal_height: u16) {
    if app.page == Page::History {
        let max_scroll = crate::ui::history::scroll_max_for_height(app, terminal_height);
        app.set_stats_scroll_max(max_scroll);
    }
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
        KeyCode::Char('p') => app.open_history(),
        KeyCode::Char('j') if app.page == Page::History => app.scroll_stats_down(),
        KeyCode::Char('k') if app.page == Page::History => app.scroll_stats_up(),
        KeyCode::Char(':') => {
            app.enter_command_mode();
        }
        KeyCode::Esc => {
            app.command_error = None;
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

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;

    use super::{handle_key, update_stats_scroll_bounds};
    use crate::app::{App, InputMode, Page};

    #[test]
    fn normal_escape_keeps_current_page() {
        let mut app = App::default();
        app.page = Page::Result;
        app.input_mode = InputMode::Normal;

        handle_key(&mut app, KeyCode::Esc);

        assert_eq!(app.page, Page::Result);
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn repeated_normal_escape_is_idempotent() {
        let mut app = App::default();
        app.page = Page::History;
        app.input_mode = InputMode::Normal;

        handle_key(&mut app, KeyCode::Esc);
        handle_key(&mut app, KeyCode::Esc);
        handle_key(&mut app, KeyCode::Esc);

        assert_eq!(app.page, Page::History);
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn r_does_nothing_in_normal_mode() {
        for page in [Page::SpeedTest, Page::Result, Page::History] {
            let mut app = App::default();
            app.page = page;
            app.input_mode = InputMode::Normal;
            let target_text = app
                .session
                .as_ref()
                .map(|session| session.target_text.clone());

            handle_key(&mut app, KeyCode::Char('r'));

            assert_eq!(app.page, page);
            assert_eq!(app.input_mode, InputMode::Normal);
            assert_eq!(
                app.session
                    .as_ref()
                    .map(|session| session.target_text.clone()),
                target_text
            );
        }
    }

    #[test]
    fn s_still_starts_typing_from_result_page() {
        let mut app = App::default();
        app.page = Page::Result;
        app.input_mode = InputMode::Normal;

        handle_key(&mut app, KeyCode::Char('s'));

        assert_eq!(app.page, Page::SpeedTest);
        assert_eq!(app.input_mode, InputMode::Typing);
    }

    #[test]
    fn command_escape_closes_command_mode_without_navigation() {
        let mut app = App::default();
        app.page = Page::Result;
        app.input_mode = InputMode::Command;
        app.command_input = "30s".to_owned();

        handle_key(&mut app, KeyCode::Esc);

        assert_eq!(app.page, Page::Result);
        assert_eq!(app.input_mode, InputMode::Normal);
        assert!(app.command_input.is_empty());
    }

    #[test]
    fn j_and_k_scroll_only_history_page_in_normal_mode() {
        let mut app = App::default();
        app.page = Page::History;
        app.input_mode = InputMode::Normal;
        app.set_stats_scroll_max(1);

        handle_key(&mut app, KeyCode::Char('j'));
        assert_eq!(app.stats_scroll_offset, 1);

        handle_key(&mut app, KeyCode::Char('k'));
        assert_eq!(app.stats_scroll_offset, 0);
    }

    #[test]
    fn repeated_j_stays_at_stats_scroll_maximum() {
        let mut app = App::default();
        app.page = Page::History;
        app.input_mode = InputMode::Normal;
        app.set_stats_scroll_max(2);

        for _ in 0..10 {
            handle_key(&mut app, KeyCode::Char('j'));
        }

        assert_eq!(app.stats_scroll_offset, 2);

        handle_key(&mut app, KeyCode::Char('k'));
        assert_eq!(app.stats_scroll_offset, 1);
    }

    #[test]
    fn resize_clamps_stats_scroll_offset_to_new_maximum() {
        let mut app = App::default();
        app.page = Page::History;
        app.set_stats_scroll_max(5);

        for _ in 0..5 {
            handle_key(&mut app, KeyCode::Char('j'));
        }
        assert_eq!(app.stats_scroll_offset, 5);

        update_stats_scroll_bounds(&mut app, 100);

        assert_eq!(app.stats_scroll_offset, 0);
    }

    #[test]
    fn j_and_k_do_not_scroll_outside_history_page() {
        let mut app = App::default();
        app.page = Page::SpeedTest;
        app.input_mode = InputMode::Normal;

        handle_key(&mut app, KeyCode::Char('j'));
        handle_key(&mut app, KeyCode::Char('k'));

        assert_eq!(app.stats_scroll_offset, 0);
    }

    #[test]
    fn j_and_k_are_typing_input_in_typing_mode() {
        let mut app = App::default();
        app.page = Page::SpeedTest;
        app.input_mode = InputMode::Typing;

        handle_key(&mut app, KeyCode::Char('j'));
        handle_key(&mut app, KeyCode::Char('k'));

        assert_eq!(app.stats_scroll_offset, 0);
        assert_eq!(
            app.session
                .as_ref()
                .map(|session| session.typed_input.as_str()),
            Some("jk")
        );
    }
}
