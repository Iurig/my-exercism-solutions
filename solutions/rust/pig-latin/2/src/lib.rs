pub fn is_vowel(index: usize, c: char) -> bool {
    match index {
        0 => "aeiou".contains(c),
        _ => "aeiouy".contains(c),
    }
}

pub fn consonants_until(word: &str) -> usize {
    if ["yt", "xr"].contains(&&word[..2]) {
        0
    } else {
        for (i, c) in word.char_indices() {
            if is_vowel(i, c) {
                if word[i.saturating_sub(1)..i + 1] == *"qu" {
                    return i + 1;
                } else {
                    return i;
                }
            }
        }
        0
    }
}

pub fn translate_word(word: &str) -> String {
    String::from(&word[consonants_until(word)..]) + &word[..consonants_until(word)] + "ay"
}

pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}
