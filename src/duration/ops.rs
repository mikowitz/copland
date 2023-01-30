use super::Duration;
use std::ops::Add;

impl Add<Duration> for Duration {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self {
        Self::new(
            self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            self.denominator * rhs.denominator,
        )
    }
}
