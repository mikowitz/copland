use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Quality {
    Perfect,
    Major,
    Minor,
    Diminished(i32),
    Augmented(i32),
}

impl Quality {
    pub fn to_float(self) -> f32 {
        match self {
            Self::Perfect => 0.,
            Self::Major => 0.,
            Self::Minor => -1.,
            Self::Diminished(i) => -1. * i as f32,
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
