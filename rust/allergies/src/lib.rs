pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {    

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
        self.score!=0 && self.score & (*allergen as u32) == (*allergen as u32)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::VALUES.into_iter().filter(|x|self.is_allergic_to( x)).collect()
    }
}
