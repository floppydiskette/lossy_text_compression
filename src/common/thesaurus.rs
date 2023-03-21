use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ThesaurusWord {
    pub word: String,
    pub synonyms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Thesaurus {
    pub words: Vec<ThesaurusWord>,
}

const THESAURUS: &str = include_str!("../../en_thesaurus.jsonl");

impl Thesaurus {
    pub fn new() -> Thesaurus {
        let mut words = Vec::new();
        for line in THESAURUS.lines() {
            let thesaurus_word: ThesaurusWord = jsonl::read(&mut line.as_bytes()).unwrap();
            words.push(thesaurus_word);
        }
        Thesaurus { words }
    }

    pub fn get_shortest_synonym(&self, word: &str) -> Option<String> {
        let mut shortest_synonym: Option<String> = Some(word.to_string());
        for thesaurus_word in &self.words {
            if thesaurus_word.word.to_ascii_lowercase() == word.to_ascii_lowercase() {
                for synonym in &thesaurus_word.synonyms {
                    if let Some(shortest_synonym_tmp) = &shortest_synonym {
                        if synonym.len() < shortest_synonym_tmp.len() {
                            shortest_synonym = Some(synonym.clone());
                        }
                    } else {
                        shortest_synonym = Some(synonym.clone());
                    }
                }
            }
        }
        shortest_synonym
    }

    pub fn get_longest_synonym(&self, word: &str) -> Option<String> {
        let mut longest_synonym: Option<String> = Some(word.to_string());
        for thesaurus_word in &self.words {
            if thesaurus_word.word.to_ascii_lowercase() == word.to_ascii_lowercase() {
                for synonym in &thesaurus_word.synonyms {
                    if let Some(longest_synonym_tmp) = &longest_synonym {
                        if synonym.len() > longest_synonym_tmp.len() {
                            longest_synonym = Some(synonym.clone());
                        }
                    } else {
                        longest_synonym = Some(synonym.clone());
                    }
                }
            }
        }
        longest_synonym
    }
}

static THESAURUS_REF: Lazy<Thesaurus> = Lazy::new(Thesaurus::new);

pub fn compress(input: impl AsRef<str>) -> String {
    let mut output = String::new();
    for word in input.as_ref().split_whitespace() {
        if let Some(shortest_synonym) = THESAURUS_REF.get_shortest_synonym(word) {
            output.push_str(&shortest_synonym);
        } else {
            output.push_str(word);
        }
        output.push(' ');
    }
    output
}

pub fn decompress(input: impl AsRef<str>) -> String {
    let mut output = String::new();
    for word in input.as_ref().split_whitespace() {
        if let Some(longest_synonym) = THESAURUS_REF.get_longest_synonym(word) {
            output.push_str(&longest_synonym);
        } else {
            output.push_str(word);
        }
        output.push(' ');
    }
    output.trim().to_string()
}