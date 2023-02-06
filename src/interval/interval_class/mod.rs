use super::{
    IntervalSize, IntervalSize::*, Polarity, Polarity::*, Quality, Quality::*, Quartertone,
    Quartertone::*,
};
use crate::error::Error;
use std::fmt;
use std::ops::Neg;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntervalClass {
    pub size: IntervalSize,
    pub quality: Quality,
    pub polarity: Option<Polarity>,
    pub quartertone: Option<Quartertone>,
}

impl IntervalClass {
    pub fn new(quality: Quality, size: IntervalSize) -> Result<Self, Error> {
        if Self::valid_quality_and_size_pair(quality, size) {
            let (quality, size) = Self::correct_quality_and_size(quality, size);
            let polarity = match (quality, size) {
                (Perfect, Unison) => None,
                _ => Some(Positive),
            };
            let quartertone = None;
            Ok(Self {
                size,
                quality,
                polarity,
                quartertone,
            })
        } else {
            Err(Error::InvalidIntervalClass(quality, size))
        }
    }

    pub fn staff_spaces(&self) -> i32 {
        self.size.staff_spaces()
    }

    pub fn quarter_sharp(&self) -> Self {
        self.apply_quartertone(QuarterSharp)
    }

    pub fn quarter_flat(&self) -> Self {
        self.apply_quartertone(QuarterFlat)
    }

    pub fn is_perfect_unison(&self) -> bool {
        self.size == Unison && self.quality == Perfect && self.quartertone.is_none()
    }

    pub fn is_perfect_octave(&self) -> bool {
        self.size == Octave && self.quality == Perfect && self.quartertone.is_none()
    }

    fn correct_quality_and_size(quality: Quality, size: IntervalSize) -> (Quality, IntervalSize) {
        match (quality, size) {
            (Perfect, Octave) => (Perfect, Octave),
            (q, Octave) => (q, Unison),
            (_, _) => (quality, size),
        }
    }

    fn valid_quality_and_size_pair(quality: Quality, size: IntervalSize) -> bool {
        let perfect_interval = [Unison, Fourth, Fifth, Octave].contains(&size);
        let major_minor_interval = [Second, Third, Sixth, Seventh].contains(&size);
        let major_minor_quality = [Major, Minor].contains(&quality);
        if perfect_interval && major_minor_quality {
            false
        } else {
            !(major_minor_interval && quality == Perfect)
        }
    }

    fn apply_quartertone(&self, quartertone: Quartertone) -> Self {
        let quartertone = Some(quartertone);
        let polarity = self.polarity.or(Some(Positive));
        Self {
            polarity,
            quartertone,
            ..*self
        }
    }

    pub fn semitones(&self) -> f32 {
        self.polarity_to_float()
            * (self.size.to_float()
                + self.quality.to_float(self.size)
                + self.quartertone_to_float())
    }

    fn polarity_to_float(&self) -> f32 {
        self.polarity.map_or(1., Polarity::to_float)
    }

    fn quartertone_to_float(&self) -> f32 {
        self.quartertone.map_or(0., Quartertone::to_float)
    }
}

impl Neg for IntervalClass {
    type Output = Self;

    fn neg(self) -> Self {
        let polarity = self.polarity.map(|pol| -pol);

        Self { polarity, ..self }
    }
}

impl fmt::Display for IntervalClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let polarity = self.polarity.map_or(String::new(), |pol| pol.to_string());
        let quartertone = self.quartertone.map_or(String::new(), |qt| qt.to_string());
        write!(f, "{polarity}{}{quartertone}{}", self.quality, self.size)
    }
}

#[cfg(test)]
mod tests;
