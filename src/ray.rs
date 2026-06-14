use crate::r3_vector::R3Vector;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: R3Vector,
    direction: R3Vector,
}
impl Ray {
    pub const fn new(origin: R3Vector, direction: R3Vector) -> Self {
        Self { origin, direction }
    }
    pub const fn origin(&self) -> R3Vector {
        self.origin
    }
    pub const fn direction(&self) -> R3Vector {
        self.direction
    }

    pub fn at(&self, t: f64) -> R3Vector {
        self.origin + (t * self.direction)
    }
}
