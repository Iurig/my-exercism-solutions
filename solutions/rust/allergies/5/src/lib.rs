use num_enum::TryFromPrimitive;

pub struct Allergies(Vec<Allergen>);

#[derive(Debug, PartialEq, Eq, Clone, TryFromPrimitive)]
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
                    (score & 1 << index != 0)
                        .then(|| Allergen::try_from(index).ok())
                        .flatten()
                })
                .collect(),
        )
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.clone()
    }
}
