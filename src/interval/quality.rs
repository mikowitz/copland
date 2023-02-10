use super::IntervalSize;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quality {
    Perfect,
    Major,
    Minor,
    Diminished(u32),
    Augmented(u32),
}

impl Quality {
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn to_float(self, interval_size: IntervalSize) -> f32 {
        match self {
            Self::Perfect | Self::Major => 0.,
            Self::Minor => -1.,
            Self::Diminished(i) => {
                let base = -1. * i as f32;
                if interval_size.can_be_perfect() {
                    base
                } else {
                    base - 1.
                }
            }
            Self::Augmented(i) => 1. * i as f32,
        }
    }
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let quality = match self {
            Self::Perfect => "P".to_string(),
            Self::Major => "M".to_string(),
            Self::Minor => "m".to_string(),
            Self::Diminished(i) => "d".repeat(*i as usize),
            Self::Augmented(i) => "A".repeat(*i as usize),
        };
        write!(f, "{quality}")
    }
}
