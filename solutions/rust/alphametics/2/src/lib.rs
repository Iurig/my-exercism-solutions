use std::{
    collections::{BTreeMap, HashMap, HashSet},
    iter::once,
};

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Combination {
    letter_choices: BTreeMap<char, i32>,
    carry: i32,
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let input_without_spaces = input.replace(' ', "");
    let mut input_as_vec: Vec<&str> = input_without_spaces.split(['+', '=']).collect();
    let sum = input_as_vec
        .pop()
        .expect("received &str must contain one element.");
    let max_len = std::cmp::max(
        sum.len(),
        input_as_vec
            .iter()
            .map(|&s| s.len())
            .max()
            .expect("sum must have elements"),
    );
    let leading_letters = input_as_vec
        .iter()
        .map(|term| term.chars().nth(0))
        .chain(once(sum.chars().nth(0)))
        .filter_map(|x| x)
        .collect::<HashSet<_>>();
    let mut count = 0;

    let combinations = (0..max_len).fold(
        HashSet::from([Combination::default()]),
        |possible_combinations, digit| {
            let to_decide = input_as_vec
                .iter()
                .map(|term| term.chars().rev().nth(digit))
                .chain(once(sum.chars().rev().nth(digit)))
                .flatten()
                .collect::<HashSet<_>>();
            possible_combinations
                .iter()
                .flat_map(|combination| {
                    let free_digits = (0..=9)
                        .filter(|d| !combination.letter_choices.values().any(|v| v == d))
                        .collect::<HashSet<_>>();

                    to_decide
                        .iter()
                        .filter(|&letter| !combination.letter_choices.contains_key(letter))
                        .fold(
                            HashSet::from([combination.clone()]),
                            |current_digit_combinations, &letter| {
                                current_digit_combinations
                                    .into_iter()
                                    .flat_map(|comb: Combination| {
                                        count += 1;
                                        if count % 100 == 0 {}
                                        free_digits
                                            .iter()
                                            .filter(|&d| {
                                                (d != &0 || !leading_letters.contains(&letter))
                                                    && !comb.letter_choices.values().any(|v| v == d)
                                            })
                                            .filter_map(|&d| {
                                                (!comb.letter_choices.values().any(|&v| v == d))
                                                    .then_some(Combination {
                                                        letter_choices: {
                                                            comb.letter_choices
                                                                .iter()
                                                                .chain(once((&letter, &d)))
                                                                .map(|(a, b)| (*a, *b))
                                                                .collect()
                                                        },
                                                        carry: comb.carry,
                                                    })
                                            })
                                            .collect::<HashSet<_>>()
                                    })
                                    .collect()
                            },
                        )
                        .iter()
                        .filter_map(|comb| {
                            {
                                (input_as_vec
                                    .iter()
                                    .filter_map(|&term| {
                                        term.chars()
                                            .rev()
                                            .nth(digit)
                                            .and_then(|c| comb.letter_choices.get(&c))
                                    })
                                    .copied()
                                    .sum::<i32>()
                                    + comb.carry
                                    - comb
                                        .letter_choices
                                        .get(&sum.chars().rev().nth(digit).unwrap_or('0'))
                                        .copied()
                                        .unwrap_or_default())
                                    % 10
                                    == 0
                            }
                            .then_some(Combination {
                                letter_choices: comb.letter_choices.clone(),
                                carry: (comb.carry
                                    + input_as_vec
                                        .iter()
                                        .filter_map(|term| {
                                            term.chars()
                                                .rev()
                                                .nth(digit)
                                                .and_then(|c| comb.letter_choices.get(&c))
                                        })
                                        .copied()
                                        .sum::<i32>())
                                    / 10,
                            })
                        })
                        .collect::<HashSet<_>>()
                })
                .collect()
        },
    );

    combinations
        .iter()
        .filter(|comb| {
            !comb
                .letter_choices
                .iter()
                .any(|(key, &value)| leading_letters.contains(key) && value == 0)
        })
        .next()
        .map(|c| {
            c.letter_choices
                .iter()
                .map(|(&k, &v)| (k, v as u8))
                .collect()
        })
}
