pub fn series(digits: &str, len: usize) -> Vec<String> {
    (*digits)
        .as_bytes()
        .windows(len)
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .collect()
}
