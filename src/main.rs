use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");
    let word_count = count_words(&buffer);
    println!("{}", word_count);
}

fn count_words(input: &str) -> usize {
    input.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_words() {
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
}
