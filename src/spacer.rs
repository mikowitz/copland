use crate::duration::Duration;
use crate::leaf::Leaf;
use crate::to_lilypond::ToLilypond;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spacer {
    written_duration: Duration,
}

impl Spacer {
    pub const fn new(written_duration: Duration) -> Self {
        Self { written_duration }
    }
}

impl Leaf for Spacer {}

impl ToLilypond for Spacer {
    fn to_lilypond(&self) -> Result<String, crate::error::Error> {
        match self.written_duration.to_lilypond() {
            Ok(duration_lilypond) => Ok(format!("s{duration_lilypond}")),
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
        let spacer = Spacer::new(Duration::new(7, 16));

        assert_eq!(spacer.to_lilypond().unwrap(), "s4..");
    }
}
