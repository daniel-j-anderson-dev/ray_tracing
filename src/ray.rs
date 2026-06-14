use crate::r3_vector::R3Vector;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: R3Vector,
    pub direction: R3Vector,
}
impl Ray {
    /// `P(t) = A + tB` where `A` is the origin vector and `B` is the direction vector
    /// ![](https://raytracing.github.io/images/fig-1.02-lerp.jpg)
    pub fn at(&self, t: f64) -> R3Vector {
        self.origin + (t * self.direction)
    }
}
