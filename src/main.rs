use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");

    let line_count = count_lines(&buffer);
    let word_count = count_words(&buffer);
    let char_count = count_chars(&buffer);

    println!("{:>8} {:>8} {:>8}", line_count, word_count, char_count);
}

fn count_lines(input: &str) -> usize {
    input.lines().count()
}

fn count_words(input: &str) -> usize {
    input.split_whitespace().count()
}

fn count_chars(input: &str) -> usize {
    input.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines() {
        let test_cases = [
            ("empty string", "", 0),
            ("single word", "hello", 1),
            ("multiple words", "hello word", 1),
            ("much whitespace", "hello     world", 1),
            ("newlines", "hello\nworld", 2),
            ("tabs", "hello\tworld", 1),
        ];

        for (name, input, expected) in &test_cases {
            let result = count_lines(input);
            assert_eq!(result, *expected, "Failed on test case: {}", name);
        }
    }

    #[test]
    fn test_count_words() {
        let test_cases = [
            ("empty string", "", 0),
            ("single word", "hello", 1),
            ("multiple words", "hello word", 2),
            ("much whitespace", "hello     world", 2),
            ("newlines", "hello\nworld", 2),
            ("tabs", "hello\tworld", 2),
        ];

        for (name, input, expected) in &test_cases {
            let result = count_words(input);
            assert_eq!(result, *expected, "Failed on test case: {}", name);
        }
    }

    #[test]
    fn test_char_count() {
        let test_cases = [
            ("empty string", "", 0),
            ("single char", "h", 1),
            ("multiple words", "hello word", 10),
            ("newlines", "hello\nworld", 11),
            ("tabs", "hello\tworld", 11),
        ];

        for (name, input, expected) in &test_cases {
            let result = count_chars(input);
            assert_eq!(result, *expected, "Failed on test case: {}", name)
        }
    }
}
