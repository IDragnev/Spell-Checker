use std::collections::HashMap;

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
        let mut counter = WordCounter {
            words_map: HashMap::new(),
        };     
        for word in input.lines()
            .map(|line| crate::clean_line(line))
            .flat_map(|line| line.split_whitespace())
        {
            let count = counter.add(word);
        }

        counter
    }
 
    pub fn add(&mut self, item: &str) {
        let word = item.trim().to_lowercase();
        let count = self.words_map.entry(word).or_insert(0);
        *count += 1;
    }

    pub fn words(&self) -> Vec<&String> {
        let vec = self.words_map
            .iter()
            .map(|(word, _)| word)
            .collect::<Vec<&String>>();
        vec.sort_unstable_by(|&a, &b| a.cmp(b));
        vec
    }

    pub fn get(&self, word: &str) -> u32 {
        *self.words_map.get(word).unwrap_or(&0)
    }

    pub fn total_count(&self) -> u32 {
        self.words_map
        .iter()
        .fold(0, |sum, (_, count)| sum + count)
    }
}

/// Искаме да можем да напечатаме един `WordCounter` с цел дебъгване.
///
/// - Първи ред: `WordCounter, total count: {}`, форматирано с `total_count`.
/// - Останалите редове: Всяка една дума, изчистена както е описано горе с `add`, с брой на
/// срещането ѝ, примерно: "foo: 13"
///
/// Всеки ред се очаква да завършва с `\n`, включително последния. Думите трябва да са сортирани по
/// брой на срещанията, най-честите трябва да са първи. Примерно:
///
///     WordCounter, total count: 25
///     foo: 13
///     bar: 12
///
impl std::fmt::Display for WordCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        unimplemented!()
    }
}