use num::Integer;

#[derive(Debug, PartialEq)]
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

    pub fn as_tuple(&self) -> (i32, i32) {
        (self.numerator, self.denominator)
    }

    pub fn as_float(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    fn reduce(a: i32, b: i32) -> (i32, i32) {
        let g = a.gcd(&b);
        (a / g, b / g)
    }

    pub fn is_printable(&self) -> bool {
        self.is_printable_length() && self.has_printable_denominator() && !self.is_tied()
    }

    fn is_printable_length(&self) -> bool {
        let f = self.as_float();

        0. < f && f < 16.
    }

    fn has_printable_denominator(&self) -> bool {
        self.denominator & (self.denominator - 1) == 0
    }

    fn is_tied(&self) -> bool {
        format!("{:b}", self.numerator).contains("01")
    }
}

mod ops;

#[cfg(test)]
mod tests;
