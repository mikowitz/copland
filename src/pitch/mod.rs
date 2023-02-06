use crate::interval::Interval;
use std::fmt;

mod pitch_class;
pub use pitch_class::PitchClass;

mod accidental;
pub use accidental::Accidental;

mod diatonic_pitch_class;
pub use diatonic_pitch_class::DiatonicPitchClass;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Octave {
    octave: i32,
}

impl Octave {
    fn as_float(self) -> f32 {
        12. * (self.octave as f32 - 4.)
    }
}

impl fmt::Display for Octave {
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pitch {
    pitch_class: PitchClass,
    octave: Octave,
}

impl Pitch {
    pub fn new(pitch_class: PitchClass, octave: i32) -> Self {
        let octave = Octave { octave };
        Self {
            pitch_class,
            octave,
        }
    }

    pub fn semitones(self) -> f32 {
        self.pitch_class.as_float() + self.octave.as_float()
    }

    pub fn pitch_class(self) -> PitchClass {
        self.pitch_class
    }

    pub fn transpose(self, interval: Interval) -> Self {
        let pitch_number = self.semitones() + interval.semitones();
        let current_dpc = self.pitch_class().diatonic_pitch_class();
        let next_dpc = current_dpc.shift(interval.staff_spaces());

        let pc = next_dpc as i32;
        let nearest_neighbor = to_nearest_octave(pitch_number, pc);
        let semitones = pitch_number - nearest_neighbor;
        let octave_number = ((pitch_number - semitones) / 12.).floor() as i32 + 4;

        let (dpc, accidental, octaves) = simplify(next_dpc, semitones, octave_number);

        Self::new(PitchClass::new(dpc, accidental), octaves)
    }
}

fn simplify(
    dpc: DiatonicPitchClass,
    mut semitones: f32,
    mut octave: i32,
) -> (DiatonicPitchClass, Accidental, i32) {
    if semitones.abs() <= 2. {
        return (dpc, Accidental::from_float(semitones), octave);
    }
    let mut dpc_index = dpc.index();

    while semitones > 2. {
        let mut step_size = 2;
        if dpc_index == 2 {
            step_size = 1;
        } else if dpc_index == 6 {
            step_size = 1;
            octave += 1;
        }
        dpc_index = (dpc_index + 1).rem_euclid(7);
        semitones -= step_size as f32;
    }

    while semitones < -2. {
        let mut step_size = 2;
        if dpc_index == 3 {
            step_size = 1;
        } else if dpc_index == 0 {
            step_size = 1;
            octave -= 1;
        }
        dpc_index = (dpc_index - 1).rem_euclid(7);
        semitones += step_size as f32;
    }

    let new_dpc = DiatonicPitchClass::from_index(dpc_index);
    let new_accidental = Accidental::from_float(semitones);

    (new_dpc, new_accidental, octave)
}

fn to_nearest_octave(pitch_number: f32, pc_number: i32) -> f32 {
    let target_pc = pitch_number.rem_euclid(12.);
    let down = (target_pc - pc_number as f32).rem_euclid(12.);
    let up = (pc_number as f32 - target_pc).rem_euclid(12.);
    if up < down {
        pitch_number + up
    } else {
        pitch_number - down
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.pitch_class, self.octave)
    }
}

#[cfg(test)]
mod tests;
