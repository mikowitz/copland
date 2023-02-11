use copland::interval::*;
use copland::pitch::*;

static P1: Interval = Interval::new(Quality::Perfect, 1);
static C4: Pitch = Pitch::new(
    PitchClass::new(DiatonicPitchClass::C, Accidental::Natural),
    4,
);

fn main() {
    println!("{P1} {C4}");
    println!("{} {}", Interval::default(), Pitch::default());
}
