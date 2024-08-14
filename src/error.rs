use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum Error {
    #[error("The Percent type's value must be in the interval: [0.0, 1.0]")]
    PercentageOutOfRange,
}
