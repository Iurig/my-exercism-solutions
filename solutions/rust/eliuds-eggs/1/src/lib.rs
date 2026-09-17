pub fn egg_count(display_value: u32) -> usize {
    (0..32)
        .map(|i| match (1 << i) & display_value {
            0 => 0,
            _ => 1,
        })
        .sum()
}
