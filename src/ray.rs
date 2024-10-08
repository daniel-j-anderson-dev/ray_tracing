use crate::vec3::Vec3;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}
impl Ray {
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub const fn origin(&self) -> Vec3 {
        self.origin
    }
    pub const fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (t * self.direction)
    }
}
