use std::fmt;

#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DiatonicPitchClass {
    #[default]
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl DiatonicPitchClass {
    #[must_use]
    pub const fn index(&self) -> i32 {
        match self {
            Self::C => 0,
            Self::D => 1,
            Self::E => 2,
            Self::F => 3,
            Self::G => 4,
            Self::A => 5,
            Self::B => 6,
        }
    }

    pub fn from_index(index: i32) -> Self {
        match index {
            i32::MIN..=-1i32 => Self::from_index(index.rem_euclid(7)),
            0 => Self::C,
            1 => Self::D,
            2 => Self::E,
            3 => Self::F,
            4 => Self::G,
            5 => Self::A,
            6 => Self::B,
            7i32..=i32::MAX => Self::from_index(index.rem_euclid(7)),
        }
    }

    pub fn shift(self, distance: i32) -> Self {
        match distance {
            0 => self,
            i32::MIN..=-1i32 => self.prev().shift(distance + 1),
            1i32..=i32::MAX => self.next().shift(distance - 1),
        }
    }

    pub const fn next(self) -> Self {
        match self {
            Self::C => Self::D,
            Self::D => Self::E,
            Self::E => Self::F,
            Self::F => Self::G,
            Self::G => Self::A,
            Self::A => Self::B,
            Self::B => Self::C,
        }
    }

    pub const fn prev(self) -> Self {
        match self {
            Self::C => Self::B,
            Self::D => Self::C,
            Self::E => Self::D,
            Self::F => Self::E,
            Self::G => Self::F,
            Self::A => Self::G,
            Self::B => Self::A,
        }
    }
}

impl fmt::Display for DiatonicPitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dpc = match self {
            Self::C => "c",
            Self::D => "d",
            Self::E => "e",
            Self::F => "f",
            Self::G => "g",
            Self::A => "a",
            Self::B => "b",
        };
        write!(f, "{dpc}")
    }
}

#[cfg(test)]
mod tests {
    use super::DiatonicPitchClass::*;

    #[test]
    fn shift() {
        assert_eq!(C.shift(2), E);
        assert_eq!(C.shift(-1), B);
    }
}
