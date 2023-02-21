use crate::duration::Duration;
use crate::interval::{IntervalSize, Quality};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("cannot print duration {}/{}", .0.numerator(), .0.denominator())]
    UnprintableDuration(Duration),
    #[error("cannot create interval class {0} {1}")]
    InvalidIntervalClass(Quality, IntervalSize),
    #[error("cannot create accidental from {0}")]
    InvalidAccidentalSize(f32),
    #[error("std::io::Error: {0}")]
    IOError(String),
    #[error("unhandled generic error")]
    GenericError,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}
