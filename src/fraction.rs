use num::Integer;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        let (numerator, denominator) = Self::reduce(numerator, denominator);

        Self {
            numerator,
            denominator,
        }
    }

    pub fn as_tuple(&self) -> (i32, i32) {
        (self.numerator, self.denominator)
    }

    fn reduce(a: i32, b: i32) -> (i32, i32) {
        let g = a.gcd(&b);
        match (a / g, b / g) {
            (a, b) if b < 0 => (-a, -b),
            (a, b) => (a, b),
        }
    }
}

impl Add<Fraction> for Fraction {
    type Output = Self;

    fn add(self, rhs: Fraction) -> Self::Output {
        let (n1, d1) = self.as_tuple();
        let (n2, d2) = rhs.as_tuple();

        Self::new(n1 * d2 + n2 * d1, d1 * d2)
    }
}

impl Div<Self> for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (n1, d1) = self.as_tuple();
        let (n2, d2) = rhs.as_tuple();

        Self::new(n1 * d2, d1 * n2)
    }
}

impl Div<i32> for Fraction {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self::new(self.numerator, self.denominator * rhs)
    }
}

impl Mul<Self> for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (n1, d1) = self.as_tuple();
        let (n2, d2) = rhs.as_tuple();

        Self::new(n1 * n2, d1 * d2)
    }
}

impl Mul<i32> for Fraction {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.numerator * rhs, self.denominator)
    }
}

impl Sub<Fraction> for Fraction {
    type Output = Self;

    fn sub(self, rhs: Fraction) -> Self::Output {
        let (n1, d1) = self.as_tuple();
        let (n2, d2) = rhs.as_tuple();

        Self::new(n1 * d2 - n2 * d1, d1 * d2)
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.numerator, self.denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let f = Fraction::new(1, 2);
        assert_eq!(f.as_tuple(), (1, 2));

        let f = Fraction::new(1, -2);
        assert_eq!(f.as_tuple(), (-1, 2));

        let f = Fraction::new(-1, 2);
        assert_eq!(f.as_tuple(), (-1, 2));

        let f = Fraction::new(-1, -2);
        assert_eq!(f.as_tuple(), (1, 2));
    }

    #[test]
    fn add() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(1, 3);

        assert_eq!(f1 + f2, Fraction::new(5, 6));
    }

    #[test]
    fn sub() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(1, 3);

        assert_eq!(f1 - f2, Fraction::new(1, 6));
        assert_eq!(f2 - f1, Fraction::new(-1, 6));
    }

    #[test]
    fn neg() {
        let f1 = Fraction::new(1, 2);

        assert_eq!(-f1, Fraction::new(-1, 2));
    }

    #[test]
    fn mul() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(1, 3);

        assert_eq!(f1 * f2, Fraction::new(1, 6));
        assert_eq!(f1 * 4, Fraction::new(2, 1));
    }

    #[test]
    fn div() {
        let f1 = Fraction::new(1, 2);
        let f2 = Fraction::new(1, 3);

        assert_eq!(f1 / f2, Fraction::new(3, 2));
        assert_eq!(f2 / 3, Fraction::new(1, 9));
    }
}
