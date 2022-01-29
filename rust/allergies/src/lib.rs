pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq)]
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

impl Allergen {
    fn score(&self)-> u32 {
        match &self {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16, 
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }

    const VALUES: [Self; 8] = [
        Self::Eggs,
        Self::Peanuts,
        Self::Shellfish,
        Self::Strawberries,
        Self::Tomatoes,
        Self::Chocolate,
        Self::Pollen,
        Self::Cats,
    ];
}

impl Allergies {
    pub fn new(score: u32) -> Self {
       Allergies{
           score
       }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score!=0 && self.score & allergen.score() == allergen.score()
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::VALUES.into_iter().filter(|x|self.is_allergic_to( x)).collect()
    }
}
