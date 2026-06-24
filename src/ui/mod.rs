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
