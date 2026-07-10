use crate::core::language_modes::LanguageMode;
use crate::core::test_session::TestMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    SetMode(TestMode),
    SetLanguageMode(LanguageMode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    Empty,
    Unknown(String),
}

pub fn parse_command(input: &str) -> Result<Command, CommandError> {
    let trimmed = input.trim().trim_start_matches(':').trim();

    if trimmed.is_empty() {
        return Err(CommandError::Empty);
    }

    if let Some(language_mode) = LanguageMode::from_label(trimmed) {
        return Ok(Command::SetLanguageMode(language_mode));
    }

    match trimmed {
        "10" => Ok(Command::SetMode(TestMode::Words(10))),
        "25" => Ok(Command::SetMode(TestMode::Words(25))),
        "50" => Ok(Command::SetMode(TestMode::Words(50))),
        "100" => Ok(Command::SetMode(TestMode::Words(100))),
        "15s" => Ok(Command::SetMode(TestMode::Time(15))),
        "30s" => Ok(Command::SetMode(TestMode::Time(30))),
        "60s" => Ok(Command::SetMode(TestMode::Time(60))),
        "120s" => Ok(Command::SetMode(TestMode::Time(120))),
        unknown => Err(CommandError::Unknown(unknown.to_owned())),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_command, Command, CommandError};
    use crate::core::language_modes::{Language, LanguageMode, LanguageVariant};
    use crate::core::test_session::TestMode;

    #[test]
    fn parses_word_count_modes() {
        assert_eq!(
            parse_command("10"),
            Ok(Command::SetMode(TestMode::Words(10)))
        );
        assert_eq!(
            parse_command(":10"),
            Ok(Command::SetMode(TestMode::Words(10)))
        );
        assert_eq!(
            parse_command("25"),
            Ok(Command::SetMode(TestMode::Words(25)))
        );
        assert_eq!(
            parse_command("100"),
            Ok(Command::SetMode(TestMode::Words(100)))
        );
    }

    #[test]
    fn parses_time_modes() {
        assert_eq!(
            parse_command("15s"),
            Ok(Command::SetMode(TestMode::Time(15)))
        );
        assert_eq!(
            parse_command(":30s"),
            Ok(Command::SetMode(TestMode::Time(30)))
        );
        assert_eq!(
            parse_command("60s"),
            Ok(Command::SetMode(TestMode::Time(60)))
        );
        assert_eq!(
            parse_command("120s"),
            Ok(Command::SetMode(TestMode::Time(120)))
        );
    }

    #[test]
    fn rejects_empty_command() {
        assert_eq!(parse_command("   "), Err(CommandError::Empty));
    }

    #[test]
    fn rejects_unsupported_word_count() {
        assert_eq!(
            parse_command("5"),
            Err(CommandError::Unknown("5".to_owned()))
        );
        assert_eq!(
            parse_command("20"),
            Err(CommandError::Unknown("20".to_owned()))
        );
    }

    #[test]
    fn rejects_unsupported_time_value() {
        assert_eq!(
            parse_command("90s"),
            Err(CommandError::Unknown("90s".to_owned()))
        );
    }

    #[test]
    fn rejects_unknown_text_command() {
        assert_eq!(
            parse_command("theme default"),
            Err(CommandError::Unknown("theme default".to_owned()))
        );
        assert_eq!(
            parse_command("quit"),
            Err(CommandError::Unknown("quit".to_owned()))
        );
        assert_eq!(
            parse_command("abc"),
            Err(CommandError::Unknown("abc".to_owned()))
        );
    }

    #[test]
    fn parses_language_modes() {
        assert_eq!(
            parse_command(":english"),
            Ok(Command::SetLanguageMode(LanguageMode::new(
                Language::English,
                LanguageVariant::Basic
            )))
        );
        assert_eq!(
            parse_command("russian-ext"),
            Ok(Command::SetLanguageMode(LanguageMode::new(
                Language::Russian,
                LanguageVariant::Extended
            )))
        );
        assert_eq!(
            parse_command(":azerbaijani-hard"),
            Ok(Command::SetLanguageMode(LanguageMode::new(
                Language::Azerbaijani,
                LanguageVariant::Hard
            )))
        );
        assert_eq!(
            parse_command(":spanish-hard"),
            Ok(Command::SetLanguageMode(LanguageMode::new(
                Language::Spanish,
                LanguageVariant::Hard
            )))
        );
    }

    #[test]
    fn rejects_language_short_aliases() {
        assert_eq!(
            parse_command(":en"),
            Err(CommandError::Unknown("en".to_owned()))
        );
        assert_eq!(
            parse_command(":ru"),
            Err(CommandError::Unknown("ru".to_owned()))
        );
    }
}
