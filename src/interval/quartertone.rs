use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quartertone {
    QuarterSharp,
    QuarterFlat,
}

impl Quartertone {
    #[must_use]
    pub const fn to_float(self) -> f32 {
        match self {
            Self::QuarterSharp => 0.5,
            Self::QuarterFlat => -0.5,
        }
    }
}

impl fmt::Display for Quartertone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let qt = match self {
            Self::QuarterSharp => "+",
            Self::QuarterFlat => "~",
        };
        write!(f, "{qt}")
    }
}
