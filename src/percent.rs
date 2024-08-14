use crate::error::Error;

/// Represents a [f64] value in the interval `[0.0, 1.0]`
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percent(f64);
impl TryFrom<f64> for Percent {
    type Error = Error;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value >= 0.0 && value <= 1.0 {
            Ok(Percent(value))
        } else {
            Err(Error::PercentageOutOfRange)
        }
    }
}
impl Percent {
    pub fn get(&self) -> f64 {
        self.0
    }
}
