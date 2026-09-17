#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist_eq(sub_list: &[i32], super_list: &[i32]) -> bool {
    sub_list.len() <= super_list.len()
        && (0..(super_list.len() - sub_list.len() + 1))
            .any(|i| sub_list[..] == super_list[i..i + sub_list.len()])
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (
        sublist_eq(first_list, second_list),
        sublist_eq(second_list, first_list),
    ) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
