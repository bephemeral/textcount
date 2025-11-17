pub fn count_words(text: &String) -> usize {
    text.split_whitespace().count()
}
