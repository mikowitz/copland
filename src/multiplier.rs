use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Multiplier {
    numerator: i32,
    denominator: i32,
}

impl Multiplier {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        let (numerator, denominator) = correct_polarity(numerator, denominator);
        Self {
            numerator,
            denominator,
        }
    }

    pub fn to_pair(&self) -> (i32, i32) {
        (self.numerator, self.denominator)
    }

    pub fn to_float(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    pub fn abs(&self) -> Self {
        Self::new(self.numerator.abs(), self.denominator.abs())
    }
}

impl Add<Multiplier> for Multiplier {
    type Output = Multiplier;

    fn add(self, rhs: Multiplier) -> Self::Output {
        let (a, b) = self.to_pair();
        let (c, d) = rhs.to_pair();
        Multiplier::new(a * d + b * c, b * d)
    }
}

impl Sub<Multiplier> for Multiplier {
    type Output = Multiplier;

    fn sub(self, rhs: Multiplier) -> Self::Output {
        let (a, b) = self.to_pair();
        let (c, d) = rhs.to_pair();
        Multiplier::new(a * d - b * c, b * d)
    }
}

impl Mul<Multiplier> for Multiplier {
    type Output = Multiplier;

    fn mul(self, rhs: Multiplier) -> Self::Output {
        let (a, b) = self.to_pair();
        let (c, d) = rhs.to_pair();
        Multiplier::new(a * c, b * d)
    }
}

impl Mul<i32> for Multiplier {
    type Output = Multiplier;

    fn mul(self, rhs: i32) -> Self::Output {
        Multiplier::new(self.numerator * rhs, self.denominator)
    }
}

impl Div<Multiplier> for Multiplier {
    type Output = Multiplier;

    fn div(self, rhs: Multiplier) -> Self::Output {
        let (a, b) = self.to_pair();
        let (c, d) = rhs.to_pair();
        Multiplier::new(a * d, b * c)
    }
}

impl Div<i32> for Multiplier {
    type Output = Multiplier;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: i32) -> Self::Output {
        Multiplier::new(self.numerator, self.denominator * rhs)
    }
}

impl Neg for Multiplier {
    type Output = Multiplier;

    fn neg(self) -> Self::Output {
        Multiplier::new(-self.numerator, self.denominator)
    }
}

fn correct_polarity(a: i32, b: i32) -> (i32, i32) {
    match (a, b) {
        (a, b) if b < 0 => (-a, -b),
        (a, b) => (a, b),
    }
}

#[cfg(test)]
mod tests {
    use super::Multiplier;

    #[test]
    fn new() {
        assert_eq!(Multiplier::new(2, 3).to_pair(), (2, 3));
        assert_eq!(Multiplier::new(-2, 3).to_pair(), (-2, 3));
        assert_eq!(Multiplier::new(2, -3).to_pair(), (-2, 3));
        assert_eq!(Multiplier::new(-2, -3).to_pair(), (2, 3));
    }

    #[test]
    fn to_float() {
        assert_eq!(Multiplier::new(2, 3).to_float(), 2. / 3.);
    }

    #[test]
    fn add() {
        let m1 = Multiplier::new(2, 3);
        let m2 = Multiplier::new(1, 4);
        assert_eq!(m1 + m2, Multiplier::new(11, 12));
    }

    #[test]
    fn subtract() {
        let m1 = Multiplier::new(2, 3);
        let m2 = Multiplier::new(1, 4);
        assert_eq!(m1 - m2, Multiplier::new(5, 12));
        assert_eq!(m2 - m1, Multiplier::new(-5, 12));
    }

    #[test]
    fn multiply() {
        let m1 = Multiplier::new(2, 3);
        let m2 = Multiplier::new(1, 4);
        assert_eq!(m1 * m2, Multiplier::new(2, 12));
        assert_eq!(m1 * 3, Multiplier::new(6, 3));
    }

    #[test]
    fn divide() {
        let m1 = Multiplier::new(2, 3);
        let m2 = Multiplier::new(1, 4);
        assert_eq!(m1 / m2, Multiplier::new(8, 3));
        assert_eq!(m2 / m1, Multiplier::new(3, 8));
        assert_eq!(m1 / 3, Multiplier::new(2, 9));
    }

    #[test]
    fn negate() {
        let m1 = Multiplier::new(2, 3);
        let m2 = Multiplier::new(-1, 4);

        assert_eq!(-m1, Multiplier::new(-2, 3));
        assert_eq!(-m2, Multiplier::new(1, 4));
    }

    #[test]
    fn abs() {
        let m1 = Multiplier::new(2, 3);
        let m2 = Multiplier::new(-1, 4);

        assert_eq!(m1.abs(), Multiplier::new(2, 3));
        assert_eq!(m2.abs(), Multiplier::new(1, 4));
    }
}
