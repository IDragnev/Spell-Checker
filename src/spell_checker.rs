use std::collections::HashSet;
use crate::word_counter::WordCounter;

pub const ALPHABET_EN: &'static str = "abcdefghijklmnopqrstuvwxyz";
pub const ALPHABET_BG: &'static str = "абвгдежзийклмнопрстуфхцчшщъьюя";

pub struct SpellChecker {
    corpus: WordCounter,
    alphabet: String,
}

impl SpellChecker {
    pub fn new(corpus: &str, alphabet: &str) -> Self {
        SpellChecker {
            corpus: WordCounter::from_str(corpus),
            alphabet: alphabet.to_owned(),
        }
    }

    pub fn correction(&self, word: &str) -> String {
        self.candidates(word)
        .into_iter()
        .max_by(|a, b| self.probability(a).partial_cmp(&self.probability(b)).unwrap())
        .expect("candidates returned empty range")
    }

    pub fn probability(&self, word: &str) -> f64 {
        if self.corpus.total_count() > 0 {
            self.corpus.get(word) as f64 / self.corpus.total_count() as f64
        }
        else {
            0.0
        }
    }

    pub fn candidates(&self, word: &str) -> Vec<String> {
        let known_words = |edits| -> Option<Vec<String>> {
            let words = self.known(&edits);
            if !words.is_empty() {
                Some(words.iter().map(|&s| s.to_owned()).collect())
            }
            else { None }
        };
        
        let edits = [word].iter().map(|s| s.to_string()).collect();
        known_words(edits)
        .or_else(|| known_words(self.edits1(word)))
        .or_else(|| known_words(self.edits2(word)))
        .unwrap_or_else(|| vec![word.to_owned()])
    }
    
    pub fn known<'a>(&self, words: &'a HashSet<String>) -> Vec<&'a String> {
        words
        .iter()
        .filter(|word| self.corpus.get(word) > 0)
        .collect()
    }

    pub fn edits1(&self, word: &str) -> HashSet<String> {
        use std::iter::FromIterator;
        let splits = 
            (0..word.len() + 1)
            .map(|i| (&word[..i], &word[i..]))
            .collect::<Vec<(&str, &str)>>();
        let deletes = Self::single_deletes(&splits);
        let inserts = self.single_inserts(&splits);
        let replaces = self.single_replaces(&splits);
        let transposes = Self::adjacent_transposes(&splits);
        HashSet::from_iter(
            deletes
            .into_iter()
            .chain(inserts.into_iter())
            .chain(replaces.into_iter())
            .chain(transposes.into_iter())
        )
    }

    fn single_deletes(splits: &[(&str, &str)]) -> Vec<String> {
        splits
        .iter()
        .filter(|(_, right)| !right.is_empty())
        .map(|(left, right)| format!("{}{}", left, &right[1..]))
        .collect()
    }

    fn adjacent_transposes(splits: &[(&str, &str)]) -> Vec<String> {
        splits
        .iter()
        .filter(|(_, right)| right.len() > 1)
        .map(|(left, right)| { 
            let right_nth = |i| right.chars().nth(i).unwrap();
            format!("{}{}{}{}", left, right_nth(1), right_nth(0), &right[2..])
        })
        .collect()   
    }

    fn single_replaces(&self, splits: &[(&str, &str)]) -> Vec<String> {
        splits
        .iter()
        .filter(|(_, right)| !right.is_empty())
        .flat_map(|(left, right)| {
            self.alphabet.chars().map(move |c| {
                format!("{}{}{}", left, c, &right[1..])
            })
        })
        .collect()
    }

    fn single_inserts(&self, splits: &[(&str, &str)]) -> Vec<String> {
        splits
        .iter()
        .flat_map(|(left, right)| {
            self.alphabet.chars().map(move |c| {
                format!("{}{}{}", left, c, right)
            })
        })
        .collect()
    }
    
    pub fn edits2(&self, word: &str) -> HashSet<String> {
        self.edits1(word)
        .into_iter()
        .flat_map(|e1| self.edits1(&e1))
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edits1_with_empty_alphabet() {
        let checker = SpellChecker::new("", "");
        let word = "ab";
        let expected_words = as_set(&["a", "b", "ba"]);
        
        assert_eq!(checker.edits1(word), expected_words);
    }
    
    #[test]
    fn edits1_with_nonempty_alphabet() {
        let checker = SpellChecker::new("", "c");
        let word = "ab";
        let expected_words = as_set(&["cb", "b", "acb", "abc", "ba", "a", "cab", "ac"]);

        assert_eq!(checker.edits1(word), expected_words); 
    }

    #[test]
    fn edits2_with_empty_alphabet() {
        let checker = SpellChecker::new("", "");
        let word = "ab";
        let expected_words = as_set(&["", "ab", "a", "b"]);

        assert_eq!(checker.edits2(word), expected_words);
    }

    #[test]
    fn edits2_with_nonempty_alphabet() {
        let checker = SpellChecker::new("", "c");
        let word = "ab";
        let expected_words = as_set(&[
            "", "a", "cb", "cac", "bc", "cba", "acbc", "cab", "ac",
            "acc", "abcc", "ab", "c", "accb", "cbc", "ca", "cc", "cacb",
            "ccb", "acb", "abc", "cabc", "bca", "ccab", "b", "bac",
        ]);
        
        assert_eq!(checker.edits2(word), expected_words);
    }

    #[test]
    fn known_words_with_empty_corpus() {
        let checker = SpellChecker::new("", ALPHABET_EN);
        let words = as_set(&["a", "b"]);

        let known_words = checker.known(&words);
        
        assert!(known_words.is_empty());
    }

    #[test]
    fn known_words_with_nonempty_corpus_and_words_which_are_not_in_the_corpus() {
        let checker = SpellChecker::new("one two three", ALPHABET_EN);
        let words = as_set(&["a", "b"]);

        let known_words = checker.known(&words);
        
        assert!(known_words.is_empty());
    }
    
    #[test]
    fn known_words_with_nonempty_corpus_and_words_which_are_in_the_corpus() {
        let checker = SpellChecker::new("one two three", ALPHABET_EN);
        let words = as_set(&["one", "a", "b"]);
        let expected_word = "one".to_owned();

        let known_words = checker.known(&words);
        
        assert_eq!(known_words.len(), 1);
        assert!(known_words.contains(&&expected_word));
    }

    fn as_set(words: &[&str]) -> HashSet<String> {
        words.iter().map(|&s| s.to_owned()).collect()
    }
}