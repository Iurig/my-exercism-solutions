pub fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'y'
}

pub fn translate_word(word: &str) -> String {
    if word[..2] == *"xr" {
        return String::from(word) + "ay";
    }
    for (i, c) in word.char_indices() {
        if is_vowel(c) && !(c == 'y' && i == 0 && word[..2] != *"yt") {
            if word[i.saturating_sub(1)..i + 1] == *"qu" {
                return String::from(&word[i + 1..]) + &word[..i + 1] + "ay";
            } else {
                return String::from(&word[i..]) + &word[..i] + "ay";
            }
        }
    }
    String::from(word) + "ay"
}

pub fn translate(input: &str) -> String {
    let mut output = input
        .split_ascii_whitespace()
        .map(translate_word)
        .fold(String::new(), |sentence, word| {
            sentence.clone() + &word + " "
        });
    output.pop();
    output
}
