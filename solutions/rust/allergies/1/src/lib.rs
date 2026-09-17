use itertools::sorted;
use num_enum::TryFromPrimitive;
use std::collections::HashSet;

pub struct Allergies(HashSet<Allergen>);

#[derive(Debug, PartialEq, Eq, Clone, Hash, TryFromPrimitive, PartialOrd, Ord)]
#[repr(u32)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(
            (Allergen::Eggs as u32..=Allergen::Cats as u32)
                .filter_map(|index| {
                    (score & 2_u32.pow(index) != 0).then_some(Allergen::try_from(index).unwrap())
                })
                .collect(),
        )
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        sorted(self.0.iter().cloned()).collect::<Vec<Allergen>>()
    }
}
