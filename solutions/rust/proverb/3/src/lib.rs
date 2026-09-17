use std::iter;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        String::new()
    } else {
        list.windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(iter::once(format!(
                "And all for the want of a {}.",
                list[0]
            )))
            .collect()
    }
}
