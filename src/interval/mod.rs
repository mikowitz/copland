#![allow(clippy::module_name_repetitions)]

mod interval_class;
pub use interval_class::IntervalClass;

mod interval_size;
pub use interval_size::IntervalSize;

mod polarity;
pub use polarity::Polarity;

mod quality;
pub use quality::Quality;

mod quartertone;
pub use quartertone::Quartertone;

use core::fmt;
use std::ops::Neg;

type Octaves = i32;

#[must_use]
#[derive(Clone, Copy, Debug)]
pub struct Interval {
    interval_class: IntervalClass,
    octaves: Octaves,
    polarity: Option<Polarity>,
}

impl Interval {
    pub fn new(quality: Quality, size: i32) -> Self {
        let (sign, interval_class_size, octaves) = calculate_octaves(quality, size);

        let mut interval_class =
            IntervalClass::new(quality, IntervalSize::from(interval_class_size)).
            expect("Should never error at this point, but got {quality} {interval_class_size} trying to create a new interval.");
        let polarity = if interval_class.is_perfect_unison() {
            None
        } else {
            Some(Polarity::from(sign))
        };
        if let Some(pol) = polarity {
            if pol.is_negative() {
                interval_class = -interval_class;
            }
        }
        Self {
            interval_class,
            octaves,
            polarity,
        }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn staff_spaces(&self) -> i32 {
        let raw_staff_spaces = self.interval_class.staff_spaces() + 7 * self.octaves;
        raw_staff_spaces * (self.polarity_to_float() as i32)
    }

    pub fn quarter_sharp(&self) -> Self {
        let mut interval_class = self.interval_class;
        let mut octaves = self.octaves;
        if self.interval_class.is_perfect_octave() {
            interval_class = IntervalClass::new(Quality::Perfect, IntervalSize::Unison).expect(
                "Should never error creating a Perfect Unison. Something has gone very wrong.",
            );
            octaves += 1;
        }
        Self {
            interval_class: interval_class.quarter_sharp(),
            octaves,
            ..*self
        }
    }

    pub fn quarter_flat(&self) -> Self {
        let mut interval_class = self.interval_class;
        let mut octaves = self.octaves;
        if self.interval_class.is_perfect_octave() {
            interval_class = IntervalClass::new(Quality::Perfect, IntervalSize::Unison).expect(
                "Should never error creating a Perfect Unison. Something has gone very wrong.",
            );
            octaves += 1;
        }
        Self {
            interval_class: interval_class.quarter_flat(),
            octaves,
            ..*self
        }
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn semitones(&self) -> f32 {
        (12. * self.polarity_to_float())
            .mul_add(self.octaves as f32, self.interval_class.semitones())
    }

    fn polarity_to_float(&self) -> f32 {
        self.polarity.map_or(1., Polarity::to_float)
    }
}

impl Neg for Interval {
    type Output = Self;

    fn neg(self) -> Self {
        let polarity = self.polarity.map(|pol| -pol);
        let interval_class = -self.interval_class;

        Self {
            interval_class,
            polarity,
            ..self
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ic = self.interval_class.to_string();
        ic.pop();
        let size = self.interval_class.size as i32 + 7 * self.octaves;

        write!(f, "{ic}{size}")
    }
}

fn calculate_octaves(quality: Quality, size: i32) -> (i32, i32, i32) {
    let (normal_size, mut octaves) = normalize_size_with_octaves(size.abs(), 0);
    let mut final_size = normal_size;
    let sign = if size > 0 { 1 } else { -1 };

    if is_perfect_octave_or_octaves(normal_size, size, quality) {
        octaves -= 1;
        final_size = 8;
    }

    (sign, final_size, octaves)
}

fn is_perfect_octave_or_octaves(normal_size: i32, size: i32, quality: Quality) -> bool {
    normal_size == 1 && size >= 8 && quality == Quality::Perfect
}

fn normalize_size_with_octaves(size: i32, octaves: i32) -> (i32, i32) {
    if size > 7 {
        normalize_size_with_octaves(size - 7, octaves + 1)
    } else {
        (size, octaves)
    }
}

#[cfg(test)]
mod tests;
