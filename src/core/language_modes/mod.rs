mod azerbaijani_basic;
mod azerbaijani_extended;
mod azerbaijani_hard;
mod english_basic;
mod english_extended;
mod english_hard;
mod russian_basic;
mod russian_extended;
mod russian_hard;
mod spanish_basic;
mod spanish_extended;
mod spanish_hard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Russian,
    Azerbaijani,
    Spanish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageVariant {
    Basic,
    Extended,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageMode {
    pub language: Language,
    pub variant: LanguageVariant,
}

#[derive(Debug, Clone, Copy)]
pub struct WordList {
    words: &'static [&'static str],
    target_size: usize,
}

impl Default for LanguageMode {
    fn default() -> Self {
        Self {
            language: Language::English,
            variant: LanguageVariant::Basic,
        }
    }
}

impl LanguageMode {
    pub const ALL: [Self; 12] = [
        Self::new(Language::English, LanguageVariant::Basic),
        Self::new(Language::English, LanguageVariant::Extended),
        Self::new(Language::English, LanguageVariant::Hard),
        Self::new(Language::Russian, LanguageVariant::Basic),
        Self::new(Language::Russian, LanguageVariant::Extended),
        Self::new(Language::Russian, LanguageVariant::Hard),
        Self::new(Language::Azerbaijani, LanguageVariant::Basic),
        Self::new(Language::Azerbaijani, LanguageVariant::Extended),
        Self::new(Language::Azerbaijani, LanguageVariant::Hard),
        Self::new(Language::Spanish, LanguageVariant::Basic),
        Self::new(Language::Spanish, LanguageVariant::Extended),
        Self::new(Language::Spanish, LanguageVariant::Hard),
    ];

    pub const fn new(language: Language, variant: LanguageVariant) -> Self {
        Self { language, variant }
    }

    pub fn label(self) -> &'static str {
        match (self.language, self.variant) {
            (Language::English, LanguageVariant::Basic) => "english",
            (Language::English, LanguageVariant::Extended) => "english-ext",
            (Language::English, LanguageVariant::Hard) => "english-hard",
            (Language::Russian, LanguageVariant::Basic) => "russian",
            (Language::Russian, LanguageVariant::Extended) => "russian-ext",
            (Language::Russian, LanguageVariant::Hard) => "russian-hard",
            (Language::Azerbaijani, LanguageVariant::Basic) => "azerbaijani",
            (Language::Azerbaijani, LanguageVariant::Extended) => "azerbaijani-ext",
            (Language::Azerbaijani, LanguageVariant::Hard) => "azerbaijani-hard",
            (Language::Spanish, LanguageVariant::Basic) => "spanish",
            (Language::Spanish, LanguageVariant::Extended) => "spanish-ext",
            (Language::Spanish, LanguageVariant::Hard) => "spanish-hard",
        }
    }

    pub fn from_label(input: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|mode| mode.label() == input.trim())
    }

    pub fn word_list(self) -> WordList {
        let target_size = match self.variant {
            LanguageVariant::Basic => 235,
            LanguageVariant::Extended => 1200,
            LanguageVariant::Hard => 1000,
        };

        WordList {
            words: words_for(self),
            target_size,
        }
    }
}

impl WordList {
    pub fn size(self) -> usize {
        self.target_size
    }

    pub fn word_at(self, index: usize) -> &'static str {
        self.words[index % self.words.len()]
    }
}

pub fn words_for(mode: LanguageMode) -> &'static [&'static str] {
    match (mode.language, mode.variant) {
        (Language::English, LanguageVariant::Basic) => english_basic::WORDS,
        (Language::English, LanguageVariant::Extended) => english_extended::WORDS,
        (Language::English, LanguageVariant::Hard) => english_hard::WORDS,
        (Language::Russian, LanguageVariant::Basic) => russian_basic::WORDS,
        (Language::Russian, LanguageVariant::Extended) => russian_extended::WORDS,
        (Language::Russian, LanguageVariant::Hard) => russian_hard::WORDS,
        (Language::Azerbaijani, LanguageVariant::Basic) => azerbaijani_basic::WORDS,
        (Language::Azerbaijani, LanguageVariant::Extended) => azerbaijani_extended::WORDS,
        (Language::Azerbaijani, LanguageVariant::Hard) => azerbaijani_hard::WORDS,
        (Language::Spanish, LanguageVariant::Basic) => spanish_basic::WORDS,
        (Language::Spanish, LanguageVariant::Extended) => spanish_extended::WORDS,
        (Language::Spanish, LanguageVariant::Hard) => spanish_hard::WORDS,
    }
}

#[cfg(test)]
mod tests {
    use super::{Language, LanguageMode, LanguageVariant};

    #[test]
    fn parses_all_supported_language_modes() {
        for mode in LanguageMode::ALL {
            assert_eq!(LanguageMode::from_label(mode.label()), Some(mode));
        }
    }

    #[test]
    fn rejects_short_aliases_and_unknown_language_modes() {
        assert_eq!(LanguageMode::from_label("en"), None);
        assert_eq!(LanguageMode::from_label("ru"), None);
        assert_eq!(LanguageMode::from_label("spanish-basic"), None);
        assert_eq!(LanguageMode::from_label("german"), None);
    }

    #[test]
    fn reports_required_word_list_sizes() {
        assert_eq!(
            LanguageMode::new(Language::English, LanguageVariant::Basic)
                .word_list()
                .size(),
            235
        );
        assert_eq!(
            LanguageMode::new(Language::Russian, LanguageVariant::Extended)
                .word_list()
                .size(),
            1200
        );
        assert_eq!(
            LanguageMode::new(Language::Spanish, LanguageVariant::Hard)
                .word_list()
                .size(),
            1000
        );
    }
}
