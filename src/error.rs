#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    PercentageOutOfRange,
}
impl Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Error::*;
        match self {
            PercentageOutOfRange => write!(
                f,
                "The Percent type's value must be in the interval: [0.0, 1.0]"
            ),
        }
    }
}
impl core::error::Error for Error {}
