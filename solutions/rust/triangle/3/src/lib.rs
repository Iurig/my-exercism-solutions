pub struct Triangle<T>([T; 3]);

impl<T> Triangle<T>
where
    T: PartialOrd + std::ops::Add<Output = T> + Default + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        (0..3)
            .all(|i| sides[i] < sides[(i + 1) % 3] + sides[(i + 2) % 3] && sides[i] > T::default())
            .then_some(Triangle(sides))
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
