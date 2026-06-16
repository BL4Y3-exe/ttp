pub mod components;
pub mod history;
pub mod result;
pub mod speed_test;

use ratatui::Frame;

use crate::app::{App, InputMode, Page};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    match app.page {
        Page::SpeedTest => speed_test::render(frame, app),
        Page::Result => result::render(frame, app),
        Page::History => history::render(frame, app),
    }

    if app.input_mode == InputMode::Command {
        components::top_panel::render_command_line(frame, &app.command_input);
    }
}
