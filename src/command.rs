#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Command {
    Placeholder(String),
}

#[allow(dead_code)]
pub fn parse_command(input: &str) -> Option<Command> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        None
    } else {
        Some(Command::Placeholder(trimmed.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_command, Command};

    #[test]
    fn ignores_empty_commands() {
        assert_eq!(parse_command("   "), None);
    }

    #[test]
    fn preserves_placeholder_command_text() {
        assert_eq!(
            parse_command("30s"),
            Some(Command::Placeholder("30s".to_owned()))
        );
    }
}
