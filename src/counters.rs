pub fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}
