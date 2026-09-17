pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb =
        (0..list.len().saturating_sub(1)).fold(String::new(), |partial_proverb, i| {
            partial_proverb + &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1])
        });
    if !list.is_empty() {
        proverb += &format!("And all for the want of a {}.", list[0]);
    }
    proverb
}
