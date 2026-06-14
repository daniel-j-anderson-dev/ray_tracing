use core::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct R3Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Constructors and conversions
impl R3Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn from_tuple((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }

    pub const fn from_array([x, y, z]: [f64; 3]) -> Self {
        Self { x, y, z }
    }

    pub const fn to_tuple(&self) -> (f64, f64, f64) {
        let &Self { x, y, z } = self;
        (x, y, z)
    }
    
    pub const fn to_array(&self) -> [f64; 3] {
        let &Self { x, y, z } = self;
        [x, y, z]
    }
}

/// Conical R3 vector operations
impl R3Vector {
    pub const fn norm_squared(&self) -> f64 {
        self.dot_product(self)
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn normalize(&self) -> Option<R3Vector> {
        let norm = self.norm();
        if norm == 0.0 {
            None
        } else {
            Some(self.scalar_divide(norm))
        }
    }

    pub const fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub const fn cross_product(&self, other: &Self) -> R3Vector {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

///arithmetic
impl R3Vector {
    pub const fn negate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub const fn scalar_multiply(&self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub const fn scalar_divide(&self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }

    pub const fn add(&self, rhs: &Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    pub const fn subtract(&self, rhs: &Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// utilities
impl R3Vector {
    /// Applies `f` to each component of `self`
    pub fn map(&self, mut f: impl FnMut(f64) -> f64) -> Self {
        let f = &mut f;
        Self {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}

// trait impls

impl Display for R3Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.x, self.y, self.z)
    }
}
impl Neg for R3Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.negate()
    }
}
impl Mul<f64> for R3Vector {
    type Output = R3Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        self.scalar_multiply(rhs)
    }
}
impl Mul<R3Vector> for f64 {
    type Output = R3Vector;
    fn mul(self, rhs: R3Vector) -> Self::Output {
        rhs.scalar_multiply(self)
    }
}
impl Div<f64> for R3Vector {
    type Output = R3Vector;
    fn div(self, rhs: f64) -> Self::Output {
        self.scalar_divide(rhs)
    }
}
impl AddAssign<R3Vector> for R3Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(self, &rhs);
    }
}
impl MulAssign<f64> for R3Vector {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.scalar_multiply(rhs);
    }
}
impl DivAssign<f64> for R3Vector {
    fn div_assign(&mut self, rhs: f64) {
        *self = self.scalar_divide(rhs);
    }
}
impl Add for R3Vector {
    type Output = Self;
    fn add(self, rhs: R3Vector) -> Self::Output {
        Self::add(&self, &rhs)
    }
}
impl Sub for R3Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(&rhs)
    }
}
