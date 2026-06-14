#[derive(Debug, PartialEq, Eq)]
pub enum Error {}
impl Error {}
impl core::fmt::Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // use Error::*;
        // match self {
        //     _ => (),
        // }
        Ok(())
    }
}
impl core::error::Error for Error {}
