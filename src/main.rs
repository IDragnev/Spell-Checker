mod word_counter;

pub fn clean_line(input: &str) -> String {
    input
    .chars()
    .filter(|&a| is_valid_symbol(a))
    .collect()
}

fn is_valid_symbol(c: char) -> bool {
    c == '-' ||
    c == '\'' ||
    c.is_alphabetic() ||
    c.is_whitespace()
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_line_with_already_cleaned_line() {
        let line = "i'm a clean-mf-line";
        assert_eq!(line, clean_line(line));
    }
    #[test]
    fn clean_line_leaves_leading_and_trailing_spaces() {
        let line = " abc \n";        
        assert_eq!(line, clean_line(line));
    }
    #[test]
    fn clean_line_with_characters_to_remove() {
        let line = "abc-1 @#";
        assert_eq!(clean_line(line), "abc- ");
    }
}