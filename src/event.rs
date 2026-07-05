use std::io::{self, Stdout};
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
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
        let terminal_size = terminal.size()?;
        update_stats_scroll_bounds(
            app,
            ratatui::layout::Rect::new(0, 0, terminal_size.width, terminal_size.height),
        );
        terminal.draw(|frame| crate::ui::render(frame, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key(app, key);
                }
            }
        }
    }

    Ok(())
}

fn update_stats_scroll_bounds(app: &mut App, terminal_area: ratatui::layout::Rect) {
    if app.page == Page::History {
        let max_scroll = crate::ui::history::scroll_max_for_area(app, terminal_area);
        app.set_stats_scroll_max(max_scroll);
    }
}

fn handle_key(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => handle_normal_key(app, key),
        InputMode::Typing => handle_typing_key(app, key.code),
        InputMode::Command => handle_command_key(app, key.code),
    }
}

fn handle_normal_key(app: &mut App, key: KeyEvent) {
    match control_key(key.code) {
        Some(ControlKey::Quit) => app.should_quit = true,
        Some(ControlKey::Start) => app.enter_typing_mode(),
        Some(ControlKey::Profile) => app.open_history(),
        Some(ControlKey::Down) if app.page == Page::History => app.scroll_stats_down(),
        Some(ControlKey::Up) if app.page == Page::History => app.scroll_stats_up(),
        Some(ControlKey::Command) => {
            app.enter_command_mode();
        }
        Some(ControlKey::Escape) => {
            app.command_error = None;
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlKey {
    Quit,
    Start,
    Profile,
    Down,
    Up,
    Command,
    Escape,
}

fn control_key(key: KeyCode) -> Option<ControlKey> {
    match key {
        KeyCode::Esc => Some(ControlKey::Escape),
        KeyCode::Char(character) => control_key_from_character(character),
        _ => None,
    }
}

fn control_key_from_character(character: char) -> Option<ControlKey> {
    match character {
        'q' | 'Q' | 'й' | 'Й' => Some(ControlKey::Quit),
        's' | 'S' | 'ы' | 'Ы' => Some(ControlKey::Start),
        'p' | 'P' | 'з' | 'З' => Some(ControlKey::Profile),
        'j' | 'J' | 'о' | 'О' => Some(ControlKey::Down),
        'k' | 'K' | 'л' | 'Л' => Some(ControlKey::Up),
        ':' | 'ж' | 'Ж' => Some(ControlKey::Command),
        _ => None,
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
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::{handle_key, update_stats_scroll_bounds};
    use crate::app::{App, InputMode, Page};

    #[test]
    fn normal_escape_keeps_current_page() {
        let mut app = App::default();
        app.page = Page::Result;
        app.input_mode = InputMode::Normal;

        handle_key(&mut app, key(KeyCode::Esc));

        assert_eq!(app.page, Page::Result);
        assert_eq!(app.input_mode, InputMode::Normal);
    }

    #[test]
    fn repeated_normal_escape_is_idempotent() {
        let mut app = App::default();
        app.page = Page::History;
        app.input_mode = InputMode::Normal;

        handle_key(&mut app, key(KeyCode::Esc));
        handle_key(&mut app, key(KeyCode::Esc));
        handle_key(&mut app, key(KeyCode::Esc));

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

            handle_key(&mut app, key(KeyCode::Char('r')));

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

        handle_key(&mut app, key(KeyCode::Char('s')));

        assert_eq!(app.page, Page::SpeedTest);
        assert_eq!(app.input_mode, InputMode::Typing);
    }

    #[test]
    fn command_escape_closes_command_mode_without_navigation() {
        let mut app = App::default();
        app.page = Page::Result;
        app.input_mode = InputMode::Command;
        app.command_input = "30s".to_owned();

        handle_key(&mut app, key(KeyCode::Esc));

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

        handle_key(&mut app, key(KeyCode::Char('j')));
        assert_eq!(app.stats_scroll_offset, 1);

        handle_key(&mut app, key(KeyCode::Char('k')));
        assert_eq!(app.stats_scroll_offset, 0);
    }

    #[test]
    fn repeated_j_stays_at_stats_scroll_maximum() {
        let mut app = App::default();
        app.page = Page::History;
        app.input_mode = InputMode::Normal;
        app.set_stats_scroll_max(2);

        for _ in 0..10 {
            handle_key(&mut app, key(KeyCode::Char('j')));
        }

        assert_eq!(app.stats_scroll_offset, 2);

        handle_key(&mut app, key(KeyCode::Char('k')));
        assert_eq!(app.stats_scroll_offset, 1);
    }

    #[test]
    fn resize_clamps_stats_scroll_offset_to_new_maximum() {
        let mut app = App::default();
        app.page = Page::History;
        app.set_stats_scroll_max(5);

        for _ in 0..5 {
            handle_key(&mut app, key(KeyCode::Char('j')));
        }
        assert_eq!(app.stats_scroll_offset, 5);

        update_stats_scroll_bounds(&mut app, ratatui::layout::Rect::new(0, 0, 100, 100));

        assert_eq!(app.stats_scroll_offset, 0);
    }

    #[test]
    fn j_and_k_do_not_scroll_outside_history_page() {
        let mut app = App::default();
        app.page = Page::SpeedTest;
        app.input_mode = InputMode::Normal;

        handle_key(&mut app, key(KeyCode::Char('j')));
        handle_key(&mut app, key(KeyCode::Char('k')));

        assert_eq!(app.stats_scroll_offset, 0);
    }

    #[test]
    fn j_and_k_are_typing_input_in_typing_mode() {
        let mut app = App::default();
        app.page = Page::SpeedTest;
        app.input_mode = InputMode::Typing;

        handle_key(&mut app, key(KeyCode::Char('j')));
        handle_key(&mut app, key(KeyCode::Char('k')));

        assert_eq!(app.stats_scroll_offset, 0);
        assert_eq!(
            app.session
                .as_ref()
                .map(|session| session.typed_input.as_str()),
            Some("jk")
        );
    }

    #[test]
    fn russian_layout_physical_control_keys_work_in_normal_mode() {
        let mut app = App::default();
        app.page = Page::SpeedTest;
        app.input_mode = InputMode::Normal;

        handle_key(&mut app, key(KeyCode::Char('ы')));

        assert_eq!(app.page, Page::SpeedTest);
        assert_eq!(app.input_mode, InputMode::Typing);

        app.input_mode = InputMode::Normal;
        handle_key(&mut app, key(KeyCode::Char('з')));

        assert_eq!(app.page, Page::History);
    }

    #[test]
    fn russian_layout_j_and_k_scroll_history_in_normal_mode() {
        let mut app = App::default();
        app.page = Page::History;
        app.input_mode = InputMode::Normal;
        app.set_stats_scroll_max(2);

        handle_key(&mut app, key(KeyCode::Char('о')));
        assert_eq!(app.stats_scroll_offset, 1);

        handle_key(&mut app, key(KeyCode::Char('л')));
        assert_eq!(app.stats_scroll_offset, 0);
    }

    #[test]
    fn typing_mode_keeps_real_layout_characters_as_input() {
        let mut app = App::default();
        app.page = Page::SpeedTest;
        app.input_mode = InputMode::Typing;

        handle_key(&mut app, key(KeyCode::Char('ы')));

        assert_eq!(
            app.session
                .as_ref()
                .map(|session| session.typed_input.as_str()),
            Some("ы")
        );
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }
}
