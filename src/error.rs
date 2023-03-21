use crate::duration::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Duration {:?} cannot be printed as a single notehead.", .0.to_pair())]
    UnprintableDuration(Duration),
}
