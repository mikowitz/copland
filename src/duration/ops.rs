use super::Duration;
use std::cmp::{PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl Add<Self> for Duration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Sub<Self> for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Mul<Self> for Duration {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Mul<i32> for Duration {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self::new(self.numerator * rhs, self.denominator)
    }
}

impl Div<Self> for Duration {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl Div<i32> for Duration {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Self::new(self.numerator, self.denominator * rhs)
    }
}

impl Neg for Duration {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.numerator, self.denominator)
    }
}

impl PartialEq<f32> for Duration {
    fn eq(&self, other: &f32) -> bool {
        self.as_float().eq(other)
    }
}

impl PartialOrd<f32> for Duration {
    fn partial_cmp(&self, other: &f32) -> Option<std::cmp::Ordering> {
        self.as_float().partial_cmp(other)
    }
}
