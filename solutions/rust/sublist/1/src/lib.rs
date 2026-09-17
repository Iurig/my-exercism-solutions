#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist_eq(first_list: &[i32], second_list: &[i32]) -> bool {
    first_list.len() <= second_list.len()
        && (0..(second_list.len() - first_list.len() + 1))
            .any(|i| &first_list[..] == &second_list[i..i + first_list.len()])
        || first_list.is_empty()
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if sublist_eq(first_list, second_list) {
        if sublist_eq(second_list, first_list) {
            return Comparison::Equal;
        } else {
            return Comparison::Sublist;
        }
    } else if sublist_eq(second_list, first_list) {
        return Comparison::Superlist;
    }
    return Comparison::Unequal;
}
