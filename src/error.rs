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
    #[error("unhandled generic error")]
    GenericError,
}
