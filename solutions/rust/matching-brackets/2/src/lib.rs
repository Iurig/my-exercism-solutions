pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .try_fold(Vec::new(), |mut openned, c| {
            match c {
                '{' | '[' | '(' => openned.push(c),
                '}' if openned.pop()? != '{' => return None,
                ']' if openned.pop()? != '[' => return None,
                ')' if openned.pop()? != '(' => return None,
                _ => {}
            };
            Some(openned)
        })
        .is_some_and(|v| v.is_empty())
}
