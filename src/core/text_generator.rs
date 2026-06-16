use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::core::test_session::TestMode;

const ENGLISH_WORDS: &[&str] = &[
    "about",
    "above",
    "after",
    "again",
    "air",
    "all",
    "almost",
    "along",
    "also",
    "always",
    "among",
    "and",
    "another",
    "answer",
    "any",
    "around",
    "ask",
    "away",
    "back",
    "because",
    "become",
    "before",
    "begin",
    "below",
    "between",
    "book",
    "both",
    "bring",
    "build",
    "but",
    "call",
    "came",
    "can",
    "carry",
    "change",
    "city",
    "close",
    "come",
    "common",
    "country",
    "course",
    "cut",
    "day",
    "different",
    "do",
    "does",
    "door",
    "down",
    "each",
    "early",
    "earth",
    "end",
    "enough",
    "even",
    "every",
    "example",
    "eye",
    "face",
    "family",
    "far",
    "father",
    "feel",
    "few",
    "find",
    "first",
    "follow",
    "food",
    "form",
    "found",
    "four",
    "friend",
    "from",
    "get",
    "give",
    "good",
    "great",
    "group",
    "grow",
    "hand",
    "hard",
    "have",
    "head",
    "hear",
    "help",
    "high",
    "home",
    "house",
    "idea",
    "important",
    "inside",
    "just",
    "keep",
    "kind",
    "know",
    "land",
    "large",
    "last",
    "later",
    "learn",
    "leave",
    "left",
    "letter",
    "life",
    "light",
    "line",
    "list",
    "little",
    "live",
    "long",
    "look",
    "made",
    "make",
    "man",
    "many",
    "mean",
    "men",
    "might",
    "mile",
    "miss",
    "more",
    "most",
    "mother",
    "move",
    "much",
    "must",
    "name",
    "near",
    "need",
    "never",
    "new",
    "next",
    "night",
    "number",
    "often",
    "old",
    "once",
    "one",
    "only",
    "open",
    "other",
    "our",
    "out",
    "over",
    "own",
    "page",
    "part",
    "people",
    "place",
    "plant",
    "play",
    "point",
    "put",
    "read",
    "really",
    "right",
    "river",
    "run",
    "same",
    "say",
    "school",
    "sea",
    "second",
    "see",
    "seem",
    "sentence",
    "set",
    "she",
    "should",
    "show",
    "side",
    "small",
    "so",
    "some",
    "something",
    "sound",
    "spell",
    "still",
    "story",
    "study",
    "such",
    "take",
    "tell",
    "than",
    "that",
    "the",
    "their",
    "them",
    "then",
    "there",
    "these",
    "they",
    "thing",
    "think",
    "this",
    "those",
    "thought",
    "three",
    "through",
    "time",
    "together",
    "too",
    "tree",
    "try",
    "turn",
    "two",
    "under",
    "until",
    "up",
    "use",
    "very",
    "walk",
    "want",
    "water",
    "way",
    "well",
    "went",
    "were",
    "what",
    "when",
    "where",
    "which",
    "while",
    "white",
    "who",
    "why",
    "will",
    "with",
    "word",
    "work",
    "world",
    "would",
    "write",
    "year",
    "you",
    "young",
];

pub fn generate_text(mode: TestMode) -> String {
    let word_count = match mode {
        TestMode::Words(words) => usize::from(words),
        TestMode::Time(15) => 80,
        TestMode::Time(30) => 160,
        TestMode::Time(60) => 320,
        TestMode::Time(120) => 640,
        TestMode::Time(seconds) => usize::from(seconds) * 5,
    };

    generate_words(word_count).join(" ")
}

fn generate_words(word_count: usize) -> Vec<&'static str> {
    let mut rng = thread_rng();

    (0..word_count)
        .map(|_| ENGLISH_WORDS.choose(&mut rng).copied().unwrap_or("the"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::generate_text;
    use crate::core::test_session::TestMode;

    #[test]
    fn generate_text_returns_correct_word_count_for_word_modes() {
        assert_eq!(
            generate_text(TestMode::Words(10))
                .split_whitespace()
                .count(),
            10
        );
        assert_eq!(
            generate_text(TestMode::Words(25))
                .split_whitespace()
                .count(),
            25
        );
        assert_eq!(
            generate_text(TestMode::Words(50))
                .split_whitespace()
                .count(),
            50
        );
        assert_eq!(
            generate_text(TestMode::Words(100))
                .split_whitespace()
                .count(),
            100
        );
    }

    #[test]
    fn generate_text_returns_enough_words_for_time_modes() {
        assert_eq!(
            generate_text(TestMode::Time(15)).split_whitespace().count(),
            80
        );
        assert_eq!(
            generate_text(TestMode::Time(30)).split_whitespace().count(),
            160
        );
        assert_eq!(
            generate_text(TestMode::Time(60)).split_whitespace().count(),
            320
        );
        assert_eq!(
            generate_text(TestMode::Time(120))
                .split_whitespace()
                .count(),
            640
        );
    }

    #[test]
    fn generated_text_is_lowercase_words_only() {
        let text = generate_text(TestMode::Words(100));

        assert!(text.chars().all(|ch| ch.is_ascii_lowercase() || ch == ' '));
    }
}
