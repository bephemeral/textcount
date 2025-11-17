pub fn count_bytes(text: &String) -> usize {
    text.bytes().count()
}

pub fn count_chars(text: &String) -> usize {
    text.chars().count()
}

pub fn count_lines(text: &String) -> usize {
    text.lines().count()
}

pub fn get_max_line_length(text: &String) -> usize {
    text.lines().map(|line| line.len()).max().unwrap_or(0)
}

pub fn count_words(text: &String) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bytes() {
        assert_eq!(2, count_bytes(&String::from("Hi")));
        assert_eq!(4, count_bytes(&String::from("Зд")));
    }

    #[test]
    fn test_count_chars() {
        assert_eq!(2, count_chars(&String::from("Hi")));
        assert_eq!(2, count_chars(&String::from("Зд")));
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(2, count_lines(&String::from("Hi\nHi")));
        assert_eq!(2, count_lines(&String::from("Hi\nHi\n")));
    }

    #[test]
    fn test_max_line_length() {
        assert_eq!(5, get_max_line_length(&String::from("Hi\nHello")));
    }

    #[test]
    fn test_count_words() {
        assert_eq!(
            9,
            count_words(&String::from(
                "The quick\nbrown  fox jumps over the lazy dog."
            ))
        );
    }
}
