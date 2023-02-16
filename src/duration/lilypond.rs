use super::Duration;
use crate::error::Error;
use crate::to_lilypond::ToLilypond;

#[allow(clippy::cast_possible_truncation)]
fn dots_count(duration: Duration) -> u32 {
    format!("{:b}", duration.numerator)
        .chars()
        .filter(|&c| c == '1')
        .count() as u32
        - 1
}

fn dots(duration: Duration) -> String {
    ".".repeat(dots_count(duration) as usize)
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn base_duration_string(duration: Duration) -> String {
    match duration.as_float() {
        f if f >= 8. => "\\maxima".to_string(),
        f if f >= 4. => "\\longa".to_string(),
        f if f >= 2. => "\\breve".to_string(),
        _ => {
            format!(
                "{}",
                2_i32.pow(f64::from(duration.denominator).log2() as u32 - dots_count(duration))
            )
        }
    }
}

impl ToLilypond for Duration {
    fn to_lilypond(&self) -> Result<String, Error> {
        if self.is_printable() {
            Ok(format!("{}{}", base_duration_string(*self), dots(*self)))
        } else {
            Err(Error::UnprintableDuration(*self))
        }
    }
}
