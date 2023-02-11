use crate::error::Error;
use std::fmt;

#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Accidental {
    DoubleFlat,
    ThreeQuarterFlat,
    Flat,
    QuarterFlat,
    #[default]
    Natural,
    QuarterSharp,
    Sharp,
    ThreeQuarterSharp,
    DoubleSharp,
}

impl Accidental {
    #[must_use]
    pub const fn as_float(&self) -> f32 {
        match self {
            Self::DoubleFlat => -2.,
            Self::ThreeQuarterFlat => -1.5,
            Self::Flat => -1.,
            Self::QuarterFlat => -0.5,
            Self::Natural => 0.,
            Self::QuarterSharp => 0.5,
            Self::Sharp => 1.,
            Self::ThreeQuarterSharp => 1.5,
            Self::DoubleSharp => 2.,
        }
    }

    /// Returns an accidental from a float in the range [-2, 2]
    ///
    /// # Errors
    ///
    /// Will return an error if the input float is not in the range [-2, 2]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_float(f: f32) -> Result<Self, Error> {
        match (f * 2.) as i32 {
            -4 => Ok(Self::DoubleFlat),
            -3 => Ok(Self::ThreeQuarterFlat),
            -2 => Ok(Self::Flat),
            -1 => Ok(Self::QuarterFlat),
            0 => Ok(Self::Natural),
            1 => Ok(Self::QuarterSharp),
            2 => Ok(Self::Sharp),
            3 => Ok(Self::ThreeQuarterSharp),
            4 => Ok(Self::DoubleSharp),
            _ => Err(Error::InvalidAccidentalSize(f)),
        }
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let acc = match self {
            Self::DoubleFlat => "ff",
            Self::ThreeQuarterFlat => "tqf",
            Self::Flat => "f",
            Self::QuarterFlat => "qf",
            Self::Natural => "",
            Self::QuarterSharp => "qs",
            Self::Sharp => "s",
            Self::ThreeQuarterSharp => "tqs",
            Self::DoubleSharp => "ss",
        };
        write!(f, "{acc}")
    }
}
