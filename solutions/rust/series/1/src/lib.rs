pub fn series(digits: &str, len: usize) -> Vec<String> {
    (*digits)
        .as_bytes()
        .windows(len)
        .map(|b| String::from_utf8(b.to_vec()).unwrap())
        .collect()
}
