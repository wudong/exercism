pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|x| *x == 0) {
            return Option::None;
        }
        let mut sides_sorted = sides.clone();
        sides_sorted.sort_unstable();
        if sides_sorted[0] + sides_sorted[1] <= sides_sorted[2] {
            return Option::None;
        }

        Option::Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.equal_sides() == 3
    }

    pub fn is_scalene(&self) -> bool {
        self.equal_sides() == 0
    }

    fn equal_sides(&self) -> i32 {
        let eq1 = if self.sides[0] == self.sides[1] { 1 } else { 0 };
        let eq2 = if self.sides[1] == self.sides[2] { 1 } else { 0 };
        let eq3 = if self.sides[0] == self.sides[2] { 1 } else { 0 };
        eq1 + eq2 + eq3
    }

    pub fn is_isosceles(&self) -> bool {
        self.equal_sides() == 1
    }
}
