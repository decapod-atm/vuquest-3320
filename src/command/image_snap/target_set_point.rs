use alloc::string::String;

use crate::result::{Error, Result};

const TARGET_SUFFIX: &str = "%";
const TARGET_DEFAULT: u8 = 50;
const TARGET_MIN: u8 = 1;
const TARGET_MAX: u8 = 99;

/// Represents the image snap update percentage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetSetPoint {
    percentage: u8,
}

impl TargetSetPoint {
    /// Creates a new [TargetSetPoint].
    pub const fn new() -> Self {
        Self {
            percentage: TARGET_DEFAULT,
        }
    }

    /// Gets the [TargetSetPoint] percentage setting.
    pub const fn percentage(&self) -> u8 {
        self.percentage
    }

    /// Creates a [TargetSetPoint] from a percentage parameter.
    pub const fn try_from_percentage(percentage: u8) -> Result<Self> {
        match percentage {
            p if p >= TARGET_MIN && p <= TARGET_MAX => Ok(Self { percentage }),
            _ => Err(Error::InvalidVariant),
        }
    }

    /// Gets the ASCII serial command code for [TargetSetPoint].
    pub fn command(&self) -> String {
        let percentage = self.percentage;
        format!("{percentage}{TARGET_SUFFIX}")
    }
}

impl Default for TargetSetPoint {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for TargetSetPoint {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        Self::try_from_percentage(val)
    }
}

impl TryFrom<&str> for TargetSetPoint {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let pos = val.find(TARGET_SUFFIX).ok_or(Error::InvalidVariant)?;
        let exp_start = val[..pos]
            .rfind(|c: char| c.is_ascii_uppercase())
            .map(|s| s + 1)
            .unwrap_or(0);

        val[exp_start..pos]
            .parse::<u8>()
            .map_err(|_| Error::InvalidVariant)
            .and_then(|u| Self::try_from_percentage(u))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (TARGET_MIN..=TARGET_MAX).for_each(|percentage| {
            let exp_percentage = TargetSetPoint { percentage };

            assert_eq!(
                TargetSetPoint::try_from_percentage(percentage),
                Ok(exp_percentage)
            );
            assert_eq!(exp_percentage.percentage(), percentage);
        });
    }

    #[test]
    fn test_invalid() {
        [0].into_iter()
            .chain((TARGET_MAX + 1)..=u8::MAX)
            .for_each(|percentage| {
                assert_eq!(
                    TargetSetPoint::try_from_percentage(percentage),
                    Err(Error::InvalidVariant)
                );
            });
    }
}
