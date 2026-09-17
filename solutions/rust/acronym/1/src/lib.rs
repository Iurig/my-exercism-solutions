fn is_all_uppercase(word: &str) -> bool {
    word.to_uppercase() == word
}

pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace(|c: char| c.is_ascii_punctuation() && c != '-', "")
        .split(|c: char| c.is_whitespace() || c == '-')
        .flat_map(|word| {
            word.char_indices().filter_map(move |(i, c)| {
                if i == 0 {
                    Some(c.to_ascii_uppercase())
                } else if !is_all_uppercase(word) && c.is_ascii_uppercase() {
                    Some(c)
                } else {
                    None
                }
            })
        })
        .filter(char::is_ascii_uppercase)
        .collect()
}
