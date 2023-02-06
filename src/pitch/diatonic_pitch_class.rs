use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiatonicPitchClass {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}

impl DiatonicPitchClass {
    pub fn index(&self) -> i32 {
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
            0 => Self::C,
            1 => Self::D,
            2 => Self::E,
            3 => Self::F,
            4 => Self::G,
            5 => Self::A,
            6 => Self::B,
            _ => todo!(),
        }
    }

    pub fn shift(self, distance: i32) -> Self {
        match distance {
            0 => self,
            d if d < 0 => self.prev().shift(d + 1),
            d if d > 0 => self.next().shift(d - 1),
            _ => todo!()
        }
    }

    pub fn next(self) -> Self {
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

    pub fn prev(self) -> Self {
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
