pub fn reply(message: &str) -> &str {
    let message_no_whitespace = message.split_whitespace().collect::<String>();
    if message_no_whitespace.is_empty() {
        "Fine. Be that way!"
    } else {
        match (
            message_no_whitespace == message_no_whitespace.to_ascii_uppercase()
                && message_no_whitespace != message_no_whitespace.to_ascii_lowercase(),
            message_no_whitespace.ends_with('?'),
        ) {
            //(yell, question) => "Response.",
            (true, true) => "Calm down, I know what I'm doing!",
            (true, false) => "Whoa, chill out!",
            (false, true) => "Sure.",
            (false, false) => "Whatever.",
        }
    }
}
