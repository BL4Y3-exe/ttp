use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub text: Color,
    pub muted: Color,
    pub accent: Color,
}

pub fn palette() -> Palette {
    Palette {
        text: Color::Gray,
        muted: Color::DarkGray,
        accent: Color::Cyan,
    }
}
