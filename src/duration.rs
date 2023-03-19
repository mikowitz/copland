use std::ops::{Add, Div, Mul, Neg, Sub};

use num::Integer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Duration {
    numerator: i32,
    denominator: i32,
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Self::Output {
        let (a, b) = self.to_pair();
        let (c, d) = rhs.to_pair();
        Duration::new(a * d + b * c, b * d)
    }
}

impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Self::Output {
        let (a, b) = self.to_pair();
        let (c, d) = rhs.to_pair();
        Duration::new(a * d - b * c, b * d)
    }
}

impl Mul<Duration> for Duration {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Self::Output {
        let (a, b) = self.to_pair();
        let (c, d) = rhs.to_pair();
        Duration::new(a * c, b * d)
    }
}

impl Mul<i32> for Duration {
    type Output = Duration;

    fn mul(self, rhs: i32) -> Self::Output {
        let (a, b) = self.to_pair();
        Duration::new(a * rhs, b)
    }
}

impl Div<Duration> for Duration {
    type Output = Duration;

    fn div(self, rhs: Duration) -> Self::Output {
        let (a, b) = self.to_pair();
        let (c, d) = rhs.to_pair();
        Duration::new(a * d, b * c)
    }
}

impl Div<i32> for Duration {
    type Output = Duration;

    fn div(self, rhs: i32) -> Self::Output {
        let (a, b) = self.to_pair();
        Duration::new(a, b * rhs)
    }
}

impl Neg for Duration {
    type Output = Duration;

    fn neg(self) -> Self::Output {
        Duration::new(-self.numerator, self.denominator)
    }
}

impl Duration {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        let (numerator, denominator) = reduce(numerator, denominator);
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

fn reduce(a: i32, b: i32) -> (i32, i32) {
    let g = a.gcd(&b);

    match (a / g, b / g) {
        (a, b) if b < 0 => (-a, -b),
        (a, b) => (a, b),
    }
}

#[cfg(test)]
mod tests {
    use super::Duration;

    #[test]
    fn new() {
        assert_eq!(Duration::new(1, 4).to_pair(), (1, 4));
        assert_eq!(Duration::new(2, 4).to_pair(), (1, 2));
        assert_eq!(Duration::new(-1, 4).to_pair(), (-1, 4));
        assert_eq!(Duration::new(1, -4).to_pair(), (-1, 4));
        assert_eq!(Duration::new(-1, -4).to_pair(), (1, 4));
    }

    #[test]
    fn from() {
        let d = Duration::new(1, 4);
        assert_eq!(Duration::from(d).to_pair(), (1, 4));
    }

    #[test]
    fn to_float() {
        assert_eq!(Duration::new(1, 2).to_float(), 0.5);
        assert_eq!(Duration::new(3, 2).to_float(), 1.5);
        assert_eq!(Duration::new(1, 4).to_float(), 0.25);
    }

    #[test]
    fn add() {
        let d1 = Duration::new(1, 4);
        let d2 = Duration::new(1, 3);

        assert_eq!(d1 + d2, Duration::new(7, 12));
    }

    #[test]
    fn subtract() {
        let d1 = Duration::new(1, 4);
        let d2 = Duration::new(1, 3);

        assert_eq!(d1 - d2, Duration::new(-1, 12));
        assert_eq!(d2 - d1, Duration::new(1, 12));
    }

    #[test]
    fn multiply() {
        let d1 = Duration::new(1, 4);
        let d2 = Duration::new(1, 3);

        assert_eq!(d1 * d2, Duration::new(1, 12));
        assert_eq!(d1 * 3, Duration::new(3, 4));
    }

    #[test]
    fn divide() {
        let d1 = Duration::new(1, 3);
        let d2 = Duration::new(2, 1);

        assert_eq!(d1 / d2, Duration::new(1, 6));
        assert_eq!(d2 / d1, Duration::new(6, 1));
        assert_eq!(d1 / 3, Duration::new(1, 9));
    }

    #[test]
    fn negate() {
        let d1 = Duration::new(1, 3);

        assert_eq!(-d1, Duration::new(-1, 3));
    }

    #[test]
    fn abs() {
        let d1 = Duration::new(-1, 3);
        let d2 = Duration::new(1, 4);

        assert_eq!(d1.abs(), Duration::new(1, 3));
        assert_eq!(d2.abs(), d2);
    }
}
