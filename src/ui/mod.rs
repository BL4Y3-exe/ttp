pub mod components;
pub mod history;
pub mod result;
pub mod speed_test;

use ratatui::Frame;

use crate::app::{App, InputMode, Page};
use crate::ui::components::shell;

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let layout = shell::layout(frame.area());
    shell::render_header(frame, layout.header, app);

    match app.page {
        Page::SpeedTest => speed_test::render(frame, layout.main, app),
        Page::Result => result::render(frame, layout.main, app),
        Page::History => history::render(frame, layout.main, app),
    }

    shell::render_footer(frame, layout.footer, app);

    if app.input_mode == InputMode::Command {
        components::top_panel::render_command_line(frame, &app.command_input);
    } else if let Some(error) = app.command_error.as_deref() {
        components::top_panel::render_command_error(frame, error);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::{backend::TestBackend, Terminal};

    use super::render;
    use crate::app::{App, InputMode, Page};

    #[test]
    fn all_pages_render_at_compact_and_large_terminal_sizes() {
        for (width, height) in [(12, 6), (24, 12), (48, 24), (120, 48)] {
            let backend = TestBackend::new(width, height);
            let mut terminal = Terminal::new(backend).expect("test terminal");

            for (page, input_mode) in [
                (Page::SpeedTest, InputMode::Normal),
                (Page::SpeedTest, InputMode::Typing),
                (Page::Result, InputMode::Normal),
                (Page::History, InputMode::Normal),
            ] {
                let mut app = App::default();
                app.page = page;
                app.input_mode = input_mode;

                terminal
                    .draw(|frame| render(frame, &app))
                    .expect("page should render");
            }
        }
    }
}
