pub struct Triangle<T: std::cmp::PartialOrd + std::ops::Add<Output = T> + Default + Copy>([T; 3]);

impl<T: std::cmp::PartialOrd + std::ops::Add<Output = T> + Default + Copy> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        for i in 0..3 {
            if sides[i] > sides[(i + 1) % 3] + sides[(i + 2) % 3] || sides[i] <= T::default() {
                return None;
            }
        }
        Some(Triangle { 0: sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.0[0] == self.0[1] && self.0[1] == self.0[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.0[0] == self.0[1] || self.0[1] == self.0[2] || self.0[0] == self.0[2]
    }
}
