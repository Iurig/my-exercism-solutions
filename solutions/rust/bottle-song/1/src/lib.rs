trait ToWords {
    fn to_words(&self) -> String;
}
pub fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
impl ToWords for u32 {
    fn to_words(&self) -> String {
        let word = match &self {
            0 => "no",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            _ => "error",
        };
        String::from(word)
    }
}
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    if take_down == 0 {
        String::from("")
    } else {
        capitalize(&start_bottles.to_words())
            + " green bottle"
            + if start_bottles != 1 { "s" } else { "" }
            + " hanging on the wall,\n"
            + &capitalize(&start_bottles.to_words())
            + " green bottle"
            + if start_bottles != 1 { "s" } else { "" }
            + " hanging on the wall,\n"
            + "And if one green bottle should accidentally fall,\n"
            + "There'll be "
            + &((start_bottles - 1).to_words())
            + " green bottle"
            + if start_bottles != 2 { "s" } else { "" }
            + " hanging on the wall.\n\n"
            + &recite(start_bottles - 1, take_down - 1)
    }
}
