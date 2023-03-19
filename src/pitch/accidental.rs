use crate::has_semitones::HasSemitones;

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

impl HasSemitones for Accidental {
    fn semitones(&self) -> f32 {
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
}
