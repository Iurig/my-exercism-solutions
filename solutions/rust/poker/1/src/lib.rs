use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Hash, Eq, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord)]
pub enum CardType {
    A,
    K,
    Q,
    J,
    Number(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardSuit {
    S,
    H,
    D,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    val: CardType,
    suit: CardSuit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord)]
pub struct Hand([Card; 5]);

pub trait Value {
    fn value(&self) -> u8;
}

impl Value for HandType {
    fn value(&self) -> u8 {
        match self {
            HandType::HighCard => 0,
            HandType::OnePair => 1,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::Straight => 4,
            HandType::Flush => 5,
            HandType::FullHouse => 6,
            HandType::FourOfAKind => 7,
            HandType::StraightFlush => 8,
            HandType::RoyalFlush => 9,
        }
    }
}

impl Value for CardType {
    fn value(&self) -> u8 {
        match self {
            CardType::A => 14,
            CardType::K => 13,
            CardType::Q => 12,
            CardType::J => 11,
            CardType::Number(value) => *value,
        }
    }
}

impl Index<usize> for Hand {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Hand {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl PartialOrd for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        u8::partial_cmp(&self.value(), &other.value())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        u8::partial_cmp(&self.value(), &other.value())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match HandType::cmp(&determine_type(*self), &determine_type(*other)) {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => Some(Vec::<u8>::cmp(
                &card_values_for_hand(*self),
                &card_values_for_hand(*other),
            )),
        }
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            val: CardType::A,
            suit: CardSuit::S,
        }
    }
}

impl From<&str> for CardSuit {
    fn from(c: &str) -> Self {
        match c {
            "S" => CardSuit::S,
            "H" => CardSuit::H,
            "D" => CardSuit::D,
            "C" => CardSuit::C,
            _ => panic!(),
        }
    }
}

impl From<&str> for CardType {
    fn from(type_str: &str) -> Self {
        if type_str.len() == 1 {
            if type_str.chars().nth(0).unwrap().is_digit(10) {
                CardType::Number(type_str.chars().nth(0).unwrap().to_digit(10).unwrap() as u8)
            } else {
                match type_str.chars().nth(0).unwrap() {
                    'A' => CardType::A,
                    'K' => CardType::K,
                    'Q' => CardType::Q,
                    'J' => CardType::J,
                    _ => panic!(),
                }
            }
        } else {
            if &type_str[0..2] == "10" {
                CardType::Number(10)
            } else {
                panic!()
            }
        }
    }
}

impl From<&str> for Card {
    fn from(card_str: &str) -> Self {
        let mut card = Card::default();
        card.suit = CardSuit::from(&card_str[card_str.len() - 1..]);
        if card_str.len() == 2 {
            card.val = CardType::from(&card_str[0..1]);
        } else if card_str.len() == 3 {
            card.val = CardType::from(&card_str[0..2]);
        } else {
            panic!()
        }
        card
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut hand = Hand([Card::default(); 5]);
        for (i, c) in value.split_ascii_whitespace().enumerate() {
            hand[i] = Card::from(c);
        }
        hand
    }
}

trait HandTypeChecking {
    fn is_flush(&self) -> bool;
    fn is_straight_flush(&self) -> bool;
    fn of_a_kind(&self) -> Vec<(u8, u8)>;
    fn is_straight(&self) -> bool;
}

impl HandTypeChecking for Hand {
    fn is_flush(&self) -> bool {
        self.0.iter().all(|&card| card.suit == self[0].suit)
    }
    fn is_straight_flush(&self) -> bool {
        self.is_flush() && self.is_straight()
    }
    fn of_a_kind(&self) -> Vec<(u8, u8)> {
        let mut kind_map = HashMap::new();
        for c in self.0 {
            kind_map
                .entry(c.val)
                .and_modify(|instances| *instances += 1)
                .or_insert(1);
        }
        let mut kind_vec = kind_map
            .iter()
            .map(|(&card, &count)| (count as u8, CardType::value(&card)))
            .collect::<Vec<(u8, u8)>>();
        kind_vec.sort();
        kind_vec.reverse();
        kind_vec
    }
    fn is_straight(&self) -> bool {
        let mut sorted_hand = Hand(self.0);
        sorted_hand.0.sort();
        let mut sorted_hand_small_ace = sorted_hand.clone();
        for i in 0..5 {
            if sorted_hand_small_ace[i].val == CardType::A {
                sorted_hand_small_ace[i].val = CardType::Number(1);
            }
        }
        sorted_hand_small_ace.0.sort();
        sorted_hand
            .0
            .iter()
            .map(|c| CardType::value(&c.val) - CardType::value(&sorted_hand[0].val))
            .collect::<Vec<u8>>()
            == vec![0, 1, 2, 3, 4]
            || sorted_hand_small_ace
                .0
                .iter()
                .map(|c| CardType::value(&c.val) - CardType::value(&sorted_hand_small_ace[0].val))
                .collect::<Vec<u8>>()
                == vec![0, 1, 2, 3, 4]
    }
}

pub fn card_values_for_hand(hand: Hand) -> Vec<u8> {
    let mut sorted_hand = hand;
    sorted_hand.0.sort();
    match determine_type(sorted_hand) {
        HandType::StraightFlush => vec![if CardType::value(&sorted_hand[4].val) == 14 {
            CardType::value(&sorted_hand[3].val)
        } else {
            CardType::value(&sorted_hand[4].val)
        }],
        HandType::Straight => vec![if CardType::value(&sorted_hand[4].val) == 14 {
            CardType::value(&sorted_hand[3].val)
        } else {
            CardType::value(&sorted_hand[4].val)
        }],
        _ => sorted_hand
            .of_a_kind()
            .iter()
            .map(|(_amount, value)| *value)
            .collect(),
    }
}

pub fn determine_type(hand: Hand) -> HandType {
    let mut sorted_hand = hand;
    sorted_hand.0.sort();
    if sorted_hand.is_straight_flush() {
        if sorted_hand[4].val == CardType::K {
            HandType::RoyalFlush
        } else {
            HandType::StraightFlush
        }
    } else {
        let kinds = sorted_hand.of_a_kind();
        let kind_numbers = kinds
            .iter()
            .map(|(amount, _value)| *amount)
            .collect::<Vec<u8>>();
        match kind_numbers.as_slice() {
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            _ if sorted_hand.is_flush() => HandType::Flush,
            _ if sorted_hand.is_straight() => HandType::Straight,
            _ => match kind_numbers.as_slice() {
                [3, 1, 1] => HandType::ThreeOfAKind,
                [2, 2, 1] => HandType::TwoPair,
                [2, 1, 1, 1] => HandType::OnePair,
                [1, 1, 1, 1, 1] => HandType::HighCard,
                _ => panic!(),
            },
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut max_hand = Hand::from(hands[0]);
    for &h in hands {
        max_hand = std::cmp::max(Hand::from(h), max_hand);
    }

    let mut winning = Vec::new();
    for &h in hands {
        if Hand::from(h) >= max_hand && Hand::from(h) <= max_hand {
            winning.push(h);
        }
    }
    winning
}
