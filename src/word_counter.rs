use std::collections::HashMap;
use std::fmt;

pub struct WordCounter {
    words_map: HashMap<String, u32>,
}

impl WordCounter {
    pub fn new() -> Self {
        WordCounter {
            words_map: HashMap::new(),
        }
    }

    pub fn from_str(input: &str) -> Self {
        let mut counter = Self::new();
        for word in input.lines()
            .map(|line| crate::clean_line(line))
            .flat_map(|line| to_words(line))
        {
            counter.add(&word);
        }
        counter
    }

    pub fn add(&mut self, item: &str) {
        let word = item.trim().to_lowercase();
        let count = self.words_map.entry(word).or_insert(0);
        *count += 1;
    }

    pub fn words(&self) -> Vec<&String> {
        let mut words = self.words_map.keys().collect::<Vec<&String>>();
        words.sort_unstable_by(|&a, &b| a.cmp(b));
        words
    }

    pub fn get(&self, word: &str) -> u32 {
        *self.words_map.get(word).unwrap_or(&0)
    }

    pub fn total_count(&self) -> u32 {
        self.words_map.values().sum()
    }
}

impl std::fmt::Display for WordCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let _ = writeln!(f, "WordCounter, total count: {}", self.total_count())?;
        let mut pairs = self.words_map.iter().collect::<Vec<(&String, &u32)>>();
        pairs.sort_unstable_by(|(_, x), (_, y)| y.cmp(&x));
        for (word, count) in &pairs {
            let _ = writeln!(f, "{}: {}", word, count)?;
        }
        Ok(())
    }
}

fn to_words(line: String) -> Vec<String> {
    line
    .split_whitespace()
    .map(|word| word.to_owned())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_counter_has_no_words() {
        let counter = WordCounter::new();
        assert!(counter.words().is_empty());
        assert!(counter.total_count() == 0);
        assert!(counter.get("random") == 0);
    }

    #[test]
    fn counter_from_string() {
        let text = "first line\nSecond LiNe\n THIRD LINE\n";
        let expected_words = vec!["first", "line", "second", "third"];
        
        let counter = WordCounter::from_str(text);

        assert_eq!(counter.words(), expected_words); 
        assert_eq!(counter.total_count(), 6);
        assert_eq!(counter.get("line"), 3);
        assert_eq!(counter.get("first"), 1);
        assert_eq!(counter.get("second"), 1);
        assert_eq!(counter.get("third"), 1);
        assert_eq!(counter.get("not-contained"), 0);
    }   

    #[test]
    fn add() {
        let mut counter = WordCounter::new();

        for word in ["word", "Word", " word ", " WORD "].iter() {
            counter.add(word);
        }

        assert_eq!(counter.get("word"), 4);
    }
}