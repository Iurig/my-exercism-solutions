use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    val: CardValue,
    suit: CardSuit,
}

#[derive(Clone, Copy, Debug)]
pub struct Hand([Card; 5]);

#[derive(Debug, PartialEq, Hash, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardValue {
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

trait Value {
    fn value(&self) -> u8;
}

trait HandTypeChecking {
    fn is_flush(&self) -> bool;
    fn is_straight_flush(&self) -> bool;
    fn of_a_kind(&self) -> Vec<(u8, u8)>;
    fn is_straight(&self) -> bool;
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

impl Value for CardValue {
    fn value(&self) -> u8 {
        match self {
            CardValue::A => 14,
            CardValue::K => 13,
            CardValue::Q => 12,
            CardValue::J => 11,
            CardValue::Number(value) => *value,
        }
    }
}

impl HandTypeChecking for Hand {
    fn is_flush(&self) -> bool {
        self.0.iter().all(|&card| card.suit == self.0[0].suit)
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
            .map(|(&card, &count)| (count as u8, CardValue::value(&card)))
            .collect::<Vec<(u8, u8)>>();
        kind_vec.sort();
        kind_vec.reverse();
        kind_vec
    }
    fn is_straight(&self) -> bool {
        let mut sorted_hand = Hand(self.0);
        sorted_hand.0.sort();
        let mut sorted_hand_small_ace = sorted_hand;
        for i in 0..5 {
            if sorted_hand_small_ace.0[i].val == CardValue::A {
                sorted_hand_small_ace.0[i].val = CardValue::Number(1);
            }
        }
        sorted_hand_small_ace.0.sort();
        sorted_hand
            .0
            .iter()
            .map(|c| CardValue::value(&c.val) - CardValue::value(&sorted_hand.0[0].val))
            .collect::<Vec<u8>>()
            == vec![0, 1, 2, 3, 4]
            || sorted_hand_small_ace
                .0
                .iter()
                .map(|c| {
                    CardValue::value(&c.val) - CardValue::value(&sorted_hand_small_ace.0[0].val)
                })
                .collect::<Vec<u8>>()
                == vec![0, 1, 2, 3, 4]
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self >= other && self <= other
    }
}

impl Eq for Hand {}

macro_rules! impl_partial_ord {
    ($($str: ident), *) => {
        $(
            impl PartialOrd for $str {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
        )*
    };
}

macro_rules! impl_ord_on_value {
    ($($str: ident), *) => {
        $(
            impl Ord for $str {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.value().cmp(&other.value())
                }
            }
        )*
    };
}

impl_partial_ord!(CardValue, HandType, Hand);

impl_ord_on_value!(CardValue, HandType);

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match determine_type(*self).cmp(&determine_type(*other)) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => card_values_for_hand(*self).cmp(&card_values_for_hand(*other)),
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

impl From<&str> for CardValue {
    fn from(type_str: &str) -> Self {
        if type_str.len() == 1 {
            if type_str.chars().nth(0).unwrap().is_ascii_digit() {
                CardValue::Number(type_str.chars().nth(0).unwrap().to_digit(10).unwrap() as u8)
            } else {
                match type_str.chars().nth(0).unwrap() {
                    'A' => CardValue::A,
                    'K' => CardValue::K,
                    'Q' => CardValue::Q,
                    'J' => CardValue::J,
                    _ => panic!(),
                }
            }
        } else {
            if &type_str[0..2] == "10" {
                CardValue::Number(10)
            } else {
                panic!()
            }
        }
    }
}

impl From<&str> for Card {
    fn from(card_str: &str) -> Self {
        let mut card = Card {
            val: CardValue::A,
            suit: CardSuit::from(&card_str[card_str.len() - 1..]),
        };
        if card_str.len() == 2 {
            card.val = CardValue::from(&card_str[0..1]);
        } else if card_str.len() == 3 {
            card.val = CardValue::from(&card_str[0..2]);
        } else {
            panic!()
        }
        card
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut hand = Hand(
            [Card {
                val: CardValue::A,
                suit: CardSuit::S,
            }; 5],
        );
        for (i, c) in value.split_ascii_whitespace().enumerate() {
            hand.0[i] = Card::from(c);
        }
        hand
    }
}

pub fn card_values_for_hand(hand: Hand) -> Vec<u8> {
    let mut sorted_hand = hand;
    sorted_hand.0.sort();
    match determine_type(sorted_hand) {
        HandType::StraightFlush => vec![if CardValue::value(&sorted_hand.0[4].val) == 14 {
            CardValue::value(&sorted_hand.0[3].val)
        } else {
            CardValue::value(&sorted_hand.0[4].val)
        }],
        HandType::Straight => vec![if CardValue::value(&sorted_hand.0[4].val) == 14 {
            CardValue::value(&sorted_hand.0[3].val)
        } else {
            CardValue::value(&sorted_hand.0[4].val)
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
        if sorted_hand.0[4].val == CardValue::K {
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
        if Hand::from(h) == max_hand {
            winning.push(h);
        }
    }
    winning
}
