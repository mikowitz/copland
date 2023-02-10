use std::fmt;
use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Polarity {
    Positive,
    Negative,
}

impl Polarity {
    #[must_use]
    pub fn is_positive(self) -> bool {
        self == Self::Positive
    }

    #[must_use]
    pub fn is_negative(self) -> bool {
        self == Self::Negative
    }

    #[must_use]
    pub const fn to_float(self) -> f32 {
        match self {
            Self::Positive => 1.,
            Self::Negative => -1.,
        }
    }
}

impl Neg for Polarity {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}

impl From<i32> for Polarity {
    fn from(int: i32) -> Self {
        if int.is_positive() {
            Self::Positive
        } else {
            Self::Negative
        }
    }
}

impl fmt::Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pol = match self {
            Self::Positive => "+",
            Self::Negative => "-",
        };
        write!(f, "{pol}")
    }
}
