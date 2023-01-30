use crate::error::Error;

pub trait ToLilypond {
    fn to_lilypond(&self) -> Result<String, Error>;
}
