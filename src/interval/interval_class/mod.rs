use super::{IntervalSize, Polarity, Quality, Quartertone};
use crate::error::Error;
use std::fmt;
use std::ops::Neg;

#[must_use]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntervalClass {
    pub size: IntervalSize,
    pub quality: Quality,
    pub polarity: Option<Polarity>,
    pub quartertone: Option<Quartertone>,
}

impl IntervalClass {
    /// Returns a new `IntervalClass` from a given `Quality` and `IntervalSize`
    ///
    /// # Errors
    ///
    /// Will return an error if given an invalid quality and interval size pair,
    /// for example, attempting to construct a Perfect Second.
    pub fn new(quality: Quality, size: IntervalSize) -> Result<Self, Error> {
        if Self::valid_quality_and_size_pair(quality, size) {
            let (quality, size) = Self::correct_quality_and_size(quality, size);
            let polarity = match (quality, size) {
                (Quality::Perfect, IntervalSize::Unison) => None,
                _ => Some(Polarity::Positive),
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

    #[must_use]
    pub const fn staff_spaces(&self) -> i32 {
        self.size.staff_spaces()
    }

    pub fn quarter_sharp(&self) -> Self {
        self.apply_quartertone(Quartertone::QuarterSharp)
    }

    pub fn quarter_flat(&self) -> Self {
        self.apply_quartertone(Quartertone::QuarterFlat)
    }

    #[must_use]
    pub fn is_perfect_unison(&self) -> bool {
        self.size == IntervalSize::Unison
            && self.quality == Quality::Perfect
            && self.quartertone.is_none()
    }

    #[must_use]
    pub fn is_perfect_octave(&self) -> bool {
        self.size == IntervalSize::Octave
            && self.quality == Quality::Perfect
            && self.quartertone.is_none()
    }

    const fn correct_quality_and_size(
        quality: Quality,
        size: IntervalSize,
    ) -> (Quality, IntervalSize) {
        match (quality, size) {
            (Quality::Perfect, IntervalSize::Octave) => (Quality::Perfect, IntervalSize::Octave),
            (q, IntervalSize::Octave) => (q, IntervalSize::Unison),
            (_, _) => (quality, size),
        }
    }

    fn valid_quality_and_size_pair(quality: Quality, size: IntervalSize) -> bool {
        let perfect_interval = size.can_be_perfect();
        let major_minor_interval = !perfect_interval;
        let major_minor_quality = [Quality::Major, Quality::Minor].contains(&quality);
        if perfect_interval && major_minor_quality {
            false
        } else {
            !(major_minor_interval && quality == Quality::Perfect)
        }
    }

    fn apply_quartertone(&self, quartertone: Quartertone) -> Self {
        let quartertone = Some(quartertone);
        let polarity = self.polarity.or(Some(Polarity::Positive));
        Self {
            polarity,
            quartertone,
            ..*self
        }
    }

    #[must_use]
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
