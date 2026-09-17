use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_case_sorted_word: String = {
            let mut v = word.to_lowercase().chars().collect::<Vec<char>>();
            v.sort_unstable();
            v
        }.iter().collect();
    possible_anagrams.iter().copied().filter(|x| {
        let mut v = x.to_lowercase().chars().collect::<Vec<char>>();
        v.sort_unstable();
        v
    }.iter().collect::<String>() == lower_case_sorted_word && word.to_lowercase() != x.to_lowercase()).collect()
}
