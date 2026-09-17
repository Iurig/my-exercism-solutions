trait ToWords {
    fn to_words(&self) -> Option<&'static str>;
}
pub fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
impl ToWords for u32 {
    fn to_words(&self) -> Option<&'static str> {
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
            _ => return None,
        };
        Some(word)
    }
}
pub fn pluralize(bottles: u32) -> String {
    if bottles != 1 {
        String::from("s")
    } else {
        String::new()
    }
}
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    if take_down == 0 {
        String::from("")
    } else {
        let first_word = capitalize(start_bottles.to_words().unwrap());
        let first_plural = pluralize(start_bottles);
        let second_word = (start_bottles - 1).to_words().unwrap();
        let second_plural = pluralize(start_bottles - 1);
        format!(
            "{first_word} green bottle{first_plural} hanging on the wall,\n\
            {first_word} green bottle{first_plural} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {second_word} green bottle{second_plural} hanging on the wall.\n\n\
            {}",
            recite(start_bottles - 1, take_down - 1)
        )
    }
}
