use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("cannot print duration {0}/{1}")]
    UnprintableDuration(i32, i32),
}
