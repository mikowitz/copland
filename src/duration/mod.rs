use num::Integer;

#[must_use]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Duration {
    numerator: i32,
    denominator: i32,
}

impl Duration {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        let (numerator, denominator) = Self::reduce(numerator, denominator);

        Self {
            numerator,
            denominator,
        }
    }

    #[must_use]
    pub const fn as_tuple(self) -> (i32, i32) {
        (self.numerator, self.denominator)
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_float(self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    pub fn abs(self) -> Self {
        Self::new(self.numerator.abs(), self.denominator)
    }

    #[must_use]
    pub fn is_negative(self) -> bool {
        self.as_float().is_sign_negative()
    }

    fn reduce(a: i32, b: i32) -> (i32, i32) {
        let g = a.gcd(&b);
        match (a / g, b / g) {
            (a, b) if b < 0 => (-a, -b),
            (a, b) => (a, b),
        }
    }

    pub fn equal_or_shorter_printable_duration(self) -> Self {
        if self.is_printable() {
            self
        } else {
            Self::new(self.numerator - 1, self.denominator)
        }
    }

    pub fn equal_or_greater_printable_duration(self) -> Self {
        if self.is_printable() {
            self
        } else {
            Self::new(self.numerator + 1, self.denominator)
        }
    }

    #[must_use]
    pub fn to_printable_duration_list(self) -> Vec<Self> {
        if self.is_printable() {
            vec![self]
        } else if self < 0. {
            vec![]
        } else {
            let first = self.equal_or_shorter_printable_duration();
            let mut tail = (self - first).to_printable_duration_list();
            tail.insert(0, first);
            tail
        }
    }

    #[must_use]
    pub fn is_printable(self) -> bool {
        self.is_printable_length() && self.has_printable_denominator() && !self.is_tied()
    }

    fn is_printable_length(self) -> bool {
        let f = self.as_float();

        0. < f && f < 16.
    }

    const fn has_printable_denominator(self) -> bool {
        self.denominator & (self.denominator - 1) == 0
    }

    fn is_tied(self) -> bool {
        format!("{:b}", self.numerator).contains("01")
    }
}

mod lilypond;
mod ops;

#[cfg(test)]
mod tests;
