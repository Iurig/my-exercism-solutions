pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    for i in 0..list.len().saturating_sub(1) {
        proverb += &([
            "For want of a ",
            list[i],
            " the ",
            list[i + 1],
            " was lost.\n",
        ]
        .concat());
    }
    if !list.is_empty() {
        proverb += &(["And all for the want of a ", list[0], "."].concat());
    }
    proverb
}
