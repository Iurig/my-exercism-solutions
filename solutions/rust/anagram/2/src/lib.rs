use std::collections::HashSet;
pub fn sort_and_convert_to_vector(word: &str) -> Vec<char> {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort_unstable();
    sorted
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_case_word = word.to_lowercase();
    let lower_case_sorted_word = sort_and_convert_to_vector(&lower_case_word);
    possible_anagrams
        .iter()
        .copied()
        .filter(|x| {
            let lower_case_x = x.to_lowercase();
            lower_case_word != lower_case_x
                && sort_and_convert_to_vector(&lower_case_x) == lower_case_sorted_word
        })
        .collect()
}
