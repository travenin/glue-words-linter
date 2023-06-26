use std::io;

const GLUE_WORDS: [&str; 36] = [
    "a", "an", "and", "asked", "be", "by", "every", "for", "from", "get", "go", "have", "if", "in",
    "is", "it", "just", "like", "make", "much", "new", "of", "on", "said", "should", "some",
    "that", "the", "there", "think", "this", "to", "was", "what", "will", "with",
];

#[derive(Debug)]
struct Sentence {
    line_number: usize,
    text: String,
}

impl Sentence {
    fn new(line_number: usize, text: String) -> Self {
        Self { line_number, text }
    }

    fn is_sticky(&self) -> bool {
        glue_words_percentage(&self.text) > 50
    }
}

impl std::fmt::Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: `{}`", self.line_number, self.text)
    }
}

fn main() -> Result<(), io::Error> {
    let input = io::read_to_string(io::stdin())?;

    let sentences = into_sentences(&input);

    let mut sticky_sentences: usize = 0;

    // Must use iter() to avoid moving the ownership of sentences.
    for sentence in sentences.iter() {
        if sentence.is_sticky() {
            sticky_sentences += 1;
            println!(
                "Line {}: {} ({}%)",
                sentence.line_number,
                sentence.text,
                glue_words_percentage(&sentence.text)
            );
        }
    }

    let sticky_sentences_percentage = if sentences.len() == 0 {
        0.0
    } else {
        100.0 * sticky_sentences as f64 / sentences.len() as f64
    };

    println!(
        "\nAmong {} sentences there were {} sticky ones ({:.2}%).",
        sentences.len(),
        sticky_sentences,
        sticky_sentences_percentage
    );

    Ok(())
}

#[derive(PartialEq)]
enum SentenceState {
    NotInSentence,
    InSentence,
}

/// Parse the input string into `Sentence` objects.
fn into_sentences(input: &str) -> Vec<Sentence> {
    let mut sentences = Vec::new();

    let mut line_number: usize = 1;
    let mut sentence_line_number = line_number;

    let mut sentence_text = String::new();
    let mut state = SentenceState::NotInSentence;

    for ch in input.chars() {
        sentence_text.push(ch);

        match ch {
            '.' | '?' | '!' => {
                sentences.push(Sentence::new(
                    sentence_line_number,
                    normalize_sentence(&sentence_text),
                ));

                sentence_text = String::new();
                state = SentenceState::NotInSentence;
            }

            '\n' => {
                line_number += 1;
            }

            ch if ch.is_alphanumeric() => {
                if state == SentenceState::NotInSentence {
                    sentence_line_number = line_number;
                    state = SentenceState::InSentence;
                }
            }

            _ => {}
        }
    }

    // If the last sentence did not end with a punctuation, add it.
    if state == SentenceState::InSentence {
        sentences.push(Sentence::new(
            sentence_line_number,
            normalize_sentence(&sentence_text),
        ));
    }

    sentences
}

/// Remove extra any whitespace and newlines.
fn normalize_sentence(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Return the percentage of glue words in the sentence.
fn glue_words_percentage(sentence: &str) -> i32 {
    let words = sentence.split_whitespace();

    let mut count_words = 0;
    let mut count_glue_words = 0;

    for word in words {
        count_words += 1;

        let cleaned = clean_word(word);
        if GLUE_WORDS.contains(&cleaned.as_str()) {
            count_glue_words += 1;
        }
    }

    if count_words == 0 {
        0
    } else {
        (100.0 * count_glue_words as f32 / count_words as f32).round() as i32
    }
}

/// Make the given word lowercase and remove any punctuations.
fn clean_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_word() {
        assert_eq!(clean_word("a"), "a");
        assert_eq!(clean_word("A"), "a");
        assert_eq!(clean_word("Word."), "word");
        assert_eq!(clean_word("HelLo"), "hello");
        assert_eq!(clean_word("!!!"), "");
        assert_eq!(clean_word(""), "");
        assert_eq!(clean_word("100"), "");
    }

    #[test]
    fn test_glue_word_percentage() {
        assert_eq!(glue_words_percentage("a"), 100);
        assert_eq!(glue_words_percentage("crab"), 0);
        assert_eq!(glue_words_percentage("What is this?"), 100);
        assert_eq!(glue_words_percentage("It's a crab"), 33);
        assert_eq!(glue_words_percentage("A some crab"), 67);
    }

    #[test]
    fn test_normalize_sentence() {
        assert_eq!(normalize_sentence("a"), "a");
        assert_eq!(normalize_sentence("a b"), "a b");
        assert_eq!(normalize_sentence("a \n b"), "a b");
        assert_eq!(normalize_sentence("    a\nb \tc\n\n  \n  d "), "a b c d");
    }

    #[test]
    fn test_into_sentences_ending_punctuation() {
        let input: &str = r#"The first sentence.
                             The second
                             sentence! The
                             3rd
                             sentence?"#;

        let sentences = into_sentences(input);
        assert_eq!(sentences.len(), 3);
        assert_eq!(sentences[0].line_number, 1);
        assert_eq!(sentences[0].text, "The first sentence.");
        assert_eq!(sentences[1].line_number, 2);
        assert_eq!(sentences[1].text, "The second sentence!");
        assert_eq!(sentences[2].line_number, 3);
        assert_eq!(sentences[2].text, "The 3rd sentence?");
    }

    #[test]
    fn test_into_sentences_no_ending_punctuation() {
        let input = r#"The first sentence.
                             The second sentence"#;

        let sentences = into_sentences(input);
        assert_eq!(sentences.len(), 2);
        assert_eq!(sentences[0].line_number, 1);
        assert_eq!(sentences[0].text, "The first sentence.");
        assert_eq!(sentences[1].line_number, 2);
        assert_eq!(sentences[1].text, "The second sentence");
    }
}
