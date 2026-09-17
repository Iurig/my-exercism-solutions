use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card {
    val: CardValue,
    suit: CardSuit,
}

#[derive(Clone, Copy, Debug)]
pub struct Hand([Card; 5]);

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardValue {
    Number(u8),
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CardSuit {
    S,
    H,
    D,
    C,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseCardError {
    InvalidSuit(String),
    InvalidValue(String),
    WrongCardAmount(usize),
}
impl std::error::Error for ParseCardError {}
impl std::fmt::Display for ParseCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSuit(suit) => write!(f, "Invalid Suit: {suit:?}"),
            Self::InvalidValue(value) => write!(f, "Invalid Value: {value:?}"),
            Self::WrongCardAmount(amount) => write!(f, "Expected 5 cards, got {amount}"),
        }
    }
}

impl CardValue {
    fn discriminant(self) -> u8 {
        match self {
            CardValue::A => 14,
            CardValue::K => 13,
            CardValue::Q => 12,
            CardValue::J => 11,
            CardValue::Number(value) => value,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// This is a total preorder, or a weak order, not a total order, due to it fufilling the connectivity
/// requirement when considered together with the equivalence relation of two poker hands tying,
/// instead of the strict equality a total order requires.
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match determine_type(*self).cmp(&determine_type(*other)) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => card_values_for_hand(*self).cmp(&card_values_for_hand(*other)),
        }
    }
}

impl TryFrom<&str> for CardSuit {
    type Error = ParseCardError;
    fn try_from(c: &str) -> Result<CardSuit, Self::Error> {
        match c {
            "S" => Ok(CardSuit::S),
            "H" => Ok(CardSuit::H),
            "D" => Ok(CardSuit::D),
            "C" => Ok(CardSuit::C),
            x => Err(ParseCardError::InvalidSuit(x.to_string())),
        }
    }
}

impl TryFrom<&str> for CardValue {
    type Error = ParseCardError;
    fn try_from(type_str: &str) -> Result<Self, Self::Error> {
        if type_str.len() == 1 {
            if type_str
                .chars()
                .nth(0)
                .ok_or(ParseCardError::InvalidValue(String::new()))?
                .is_ascii_digit()
            {
                Ok(CardValue::Number(type_str.as_bytes()[0] - b'0'))
            } else {
                match type_str.chars().nth(0).unwrap() {
                    'A' => Ok(CardValue::A),
                    'K' => Ok(CardValue::K),
                    'Q' => Ok(CardValue::Q),
                    'J' => Ok(CardValue::J),
                    x => Err(ParseCardError::InvalidValue(x.to_string())),
                }
            }
        } else {
            if &type_str[0..2] == "10" {
                Ok(CardValue::Number(10))
            } else {
                Err(ParseCardError::InvalidValue(type_str.to_string()))
            }
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = ParseCardError;
    fn try_from(card_str: &str) -> Result<Self, Self::Error> {
        let (value, suit) = card_str.split_at(card_str.len() - 1);
        Ok(Card {
            val: CardValue::try_from(value)?,
            suit: CardSuit::try_from(suit)?,
        })
    }
}

impl TryFrom<&str> for Hand {
    type Error = ParseCardError;
    fn try_from(hand_as_string: &str) -> Result<Self, Self::Error> {
        let mut cards = hand_as_string
            .split_ascii_whitespace()
            .map(Card::try_from)
            .collect::<Result<Vec<Card>, Self::Error>>()?;
        cards.sort();
        Ok(Hand(cards.try_into().map_err(|e: Vec<Card>| {
            ParseCardError::WrongCardAmount(e.len())
        })?))
    }
}

pub fn card_values_for_hand(hand: Hand) -> Vec<u8> {
    match determine_type(hand) {
        HandType::Straight | HandType::StraightFlush => {
            vec![if CardValue::discriminant(hand.0[4].val) == 14 {
                CardValue::discriminant(hand.0[3].val)
            } else {
                CardValue::discriminant(hand.0[4].val)
            }]
        }
        _ => hand
            .of_a_kind()
            .iter()
            .map(|(_amount, value)| *value)
            .collect(),
    }
}

/// Auxiliary functions for determining a hand type
impl Hand {
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
                .and_modify(|instances| *instances += 1u8)
                .or_insert(1);
        }
        let mut kind_vec = kind_map
            .iter()
            .map(|(&card, &count)| (count, card.discriminant()))
            .collect::<Vec<(u8, u8)>>();
        kind_vec.sort_unstable();
        kind_vec.reverse();
        kind_vec
    }
    fn is_straight(&self) -> bool {
        let mut sorted_hand_small_ace = *self;
        for i in 0..5 {
            if sorted_hand_small_ace.0[i].val == CardValue::A {
                sorted_hand_small_ace.0[i].val = CardValue::Number(1);
            }
        }
        sorted_hand_small_ace.0.sort();
        self.0
            .iter()
            .map(|c| c.val.discriminant() - self.0[0].val.discriminant())
            .collect::<Vec<u8>>()
            == vec![0, 1, 2, 3, 4]
            || sorted_hand_small_ace
                .0
                .iter()
                .map(|c| c.val.discriminant() - sorted_hand_small_ace.0[0].val.discriminant())
                .collect::<Vec<u8>>()
                == vec![0, 1, 2, 3, 4]
    }
}
/// Should not panic, if it does, there's a logical error inside
#[allow(clippy::missing_panics_doc)]
pub fn determine_type(hand: Hand) -> HandType {
    if hand.is_straight_flush() {
        if hand.0[4].val == CardValue::K {
            HandType::RoyalFlush
        } else {
            HandType::StraightFlush
        }
    } else {
        let kinds = hand.of_a_kind();
        let kind_numbers = kinds
            .iter()
            .map(|(amount, _value)| *amount)
            .collect::<Vec<u8>>();
        match kind_numbers.as_slice() {
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            _ if hand.is_flush() => HandType::Flush,
            _ if hand.is_straight() => HandType::Straight,
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

/// # Panics
///
/// Panics if the `str`s given aren't parseable as hands, ready for error handling.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut max_hand = Hand::try_from(hands[0]).unwrap();
    for &h in hands {
        max_hand = std::cmp::max(Hand::try_from(h).unwrap(), max_hand);
    }

    let mut winning = Vec::new();
    for &h in hands {
        if Hand::try_from(h).unwrap() == max_hand {
            winning.push(h);
        }
    }
    winning
}
