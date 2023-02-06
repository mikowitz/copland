use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Accidental {
    DoubleFlat,
    ThreeQuarterFlat,
    Flat,
    QuarterFlat,
    Natural,
    QuarterSharp,
    Sharp,
    ThreeQuarterSharp,
    DoubleSharp,
}

impl Accidental {
    pub fn as_float(&self) -> f32 {
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

    pub fn from_float(f: f32) -> Self {
        match normalize_accidental((f * 2.) as i32) {
            -4 => Self::DoubleFlat,
            -3 => Self::ThreeQuarterFlat,
            -2 => Self::Flat,
            -1 => Self::QuarterFlat,
            0 => Self::Natural,
            1 => Self::QuarterSharp,
            2 => Self::Sharp,
            3 => Self::ThreeQuarterSharp,
            4 => Self::DoubleSharp,
            _ => todo!(),
        }
    }
}

fn normalize_accidental(i: i32) -> i32 {
    if i < -4 {
        normalize_accidental(i + 12)
    } else if i > 4 {
        normalize_accidental(i - 4)
    } else {
        i
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
