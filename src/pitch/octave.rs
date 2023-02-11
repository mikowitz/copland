use std::fmt;

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Octave {
    octave: i32,
}

impl Octave {
    pub const fn new(octave: i32) -> Self {
        Self { octave }
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_float(self) -> f32 {
        12. * (self.octave as f32 - 4.)
    }
}

impl Default for Octave {
    fn default() -> Self {
        Self { octave: 4 }
    }
}

impl fmt::Display for Octave {
    #[allow(clippy::cast_sign_loss)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let o = match self.octave {
            3 => String::new(),
            o if o < 3 => ",".repeat((3 - o) as usize),
            o if o > 3 => "'".repeat((o - 3) as usize),
            _ => todo!(),
        };
        write!(f, "{o}")
    }
}
