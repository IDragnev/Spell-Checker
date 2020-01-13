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
        let known_words = |edits| {
            let words = self.known(&edits);
            if !words.is_empty() {
                let mut vec = words.iter()
                    .map(|&s| s.to_owned())
                    .collect::<Vec<String>>();
                vec.sort_unstable_by(|a, b| a.cmp(b));
                Some(vec)
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
        let splits = word
            .char_indices()
            .map(|(i, _)| (&word[..i], &word[i..]))
            .chain([(word, "")].iter().copied())
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
        .map(|(left, right)| {
            format!("{}{}", left, drop_leading_chars(1, right))
        })
        .collect()
    }

    fn adjacent_transposes(splits: &[(&str, &str)]) -> Vec<String> {
        splits
        .iter()
        .filter(|(_, right)| right.chars().count() > 1)
        .map(|(left, right)| {
            let r = drop_leading_chars(2, right);
            let right_nth = |i| right.chars().nth(i).unwrap();
            format!("{}{}{}{}", left, right_nth(1), right_nth(0), r)
        })
        .collect()      
    }

    fn single_replaces(&self, splits: &[(&str, &str)]) -> Vec<String> {
        splits
        .iter()
        .filter(|(_, right)| !right.is_empty())
        .flat_map(|(left, right)| {
            self.alphabet.chars().map(move |c| {
                format!("{}{}{}", left, c, drop_leading_chars(1, right))
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

fn drop_leading_chars(n: usize, s: &str) -> &str {
    s
    .char_indices()
    .skip(n)
    .next()
    .map(|(i, _)| &s[i..])
    .unwrap_or("")
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
    fn edits1_with_nonempty_en_alphabet() {
        let checker = SpellChecker::new("", "c");
        let word = "ab";
        let expected_words = as_set(&["cb", "b", "acb", "abc", "ba", "a", "cab", "ac"]);

        assert_eq!(checker.edits1(word), expected_words); 
    }
    
    #[test]
    fn edits1_with_nonempty_bg_alphabet() {
        let checker = SpellChecker::new("", "з");
        let word = "ей";
        let expected_words = as_set(&["ез", "езй", "й", "зй", "ейз", "зей", "е", "йе"]);

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
    fn edits2_with_nonempty_en_alphabet() {
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
    fn edits2_with_nonempty_bg_alphabet() {
        let checker = SpellChecker::new("", "з");
        let word = "ей";
        let expected_words = as_set(&[
            "", "зез", "езйз", "ейз", "з", "зей", "зз", "зйе", "ез",
            "езй", "йез", "зейз", "ейзз", "еззй", "ззй", "зй", "зе",
            "йзе", "зезй", "е", "ззей", "ей", "йз", "езз", "й", "зйз",
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
        let checker = SpellChecker::new("one two three изненада", ALPHABET_EN);
        let words = as_set(&["a", "й"]);

        let known_words = checker.known(&words);
        
        assert!(known_words.is_empty());
    }
    
    #[test]
    fn known_words_with_nonempty_corpus_and_words_which_are_in_the_corpus() {
        let checker = SpellChecker::new("one two three изненада", ALPHABET_EN);
        let words = as_set(&["a", "b", "изненада"]);
        let expected_word = "изненада".to_owned();

        let known_words = checker.known(&words);
        
        assert_eq!(known_words.len(), 1);
        assert!(known_words.contains(&&expected_word));
    }

    fn as_set(words: &[&str]) -> HashSet<String> {
        words.iter().map(|&s| s.to_owned()).collect()
    }

    #[test]
    fn candidates_with_very_different_word_from_ones_on_corpus() {
        let checker = SpellChecker::new("ice isle spie crie dice mice mic", ALPHABET_EN);
        let word = "hamlet".to_owned();

        let candidates = checker.candidates(&word);

        assert_eq!(candidates.len(), 1);
        assert!(candidates.contains(&word));
    }
    
    #[test]
    fn candidates_with_one_letter_difference() {
        let checker = SpellChecker::new("ice isle spie crie dice mice mic", ALPHABET_EN);
        let word = "ide".to_owned();
        let expected = "ice".to_owned();

        let candidates = checker.candidates(&word);

        assert_eq!(candidates.len(), 1);
        assert!(candidates.contains(&expected));
    }
    
    #[test]
    fn candidates_with_two_letters_difference() {
        let checker = SpellChecker::new("ice isle spie crie dice mice mic", ALPHABET_EN);
        let word = "idde".to_owned();
        let expected = ["dice", "ice", "isle"];

        let candidates = checker.candidates(&word);

        assert_eq!(candidates, expected);
    }

    #[test]
    fn correction_with_very_different_word_from_ones_on_corpus() {
        let checker = SpellChecker::new("ice isle spie crie dice mice mic", ALPHABET_EN);
        let word = "hamlet".to_owned();

        let correction = checker.correction(&word);

        assert_eq!(correction, word);
    }

    #[test]
    fn correction_with_one_letter_difference() {
        let checker = SpellChecker::new("ice isle spie crie dice mice mic", ALPHABET_EN);
        let word = "ide".to_owned();
        let expected = "ice".to_owned();

        let correction = checker.correction(&word);

        assert_eq!(correction, expected);
    }

    #[test]
    fn correction_with_two_letters_difference() {
        let checker = SpellChecker::new("ice isle spie crie dice mice mic", ALPHABET_EN);
        let word = "idde";
        let expected = "isle";

        let correction = checker.correction(&word);

        assert_eq!(correction, expected);
    }
}