use crate::duration::Duration;
use crate::leaf::Leaf;
use crate::to_lilypond::ToLilypond;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rest {
    written_duration: Duration,
}

impl Rest {
    pub const fn new(written_duration: Duration) -> Self {
        Self { written_duration }
    }
}

impl Leaf for Rest {}

impl ToLilypond for Rest {
    fn to_lilypond(&self) -> Result<String, crate::error::Error> {
        match self.written_duration.to_lilypond() {
            Ok(duration_lilypond) => Ok(format!("r{duration_lilypond}")),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duration::Duration;

    #[test]
    fn to_lilypond() {
        let rest = Rest::new(Duration::new(3, 4));

        assert_eq!(rest.to_lilypond().unwrap(), "r2.");
    }
}
