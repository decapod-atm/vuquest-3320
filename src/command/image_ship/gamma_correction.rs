use alloc::string::String;

use crate::result::{Error, Result};

const GAMMA_SUFFIX: &str = "K";
const GAMMA_DEFAULT: u16 = 0;
const GAMMA_MAX: u16 = 1000;

/// Represents the image ship gamma correction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GammaCorrection {
    factor: u16,
}

impl GammaCorrection {
    /// Creates a new [GammaCorrection].
    pub const fn new() -> Self {
        Self {
            factor: GAMMA_DEFAULT,
        }
    }

    /// Gets the [GammaCorrection] factor setting.
    pub const fn factor(&self) -> u16 {
        self.factor
    }

    /// Creates a [GammaCorrection] from a factor parameter.
    pub const fn try_from_factor(factor: u16) -> Result<Self> {
        match factor {
            s if s <= GAMMA_MAX => Ok(Self { factor }),
            _ => Err(Error::InvalidValue(factor as usize)),
        }
    }

    /// Gets the ASCII serial command code for [GammaCorrection].
    pub fn command(&self) -> String {
        let factor = self.factor;
        format!("{factor}{GAMMA_SUFFIX}")
    }
}

impl Default for GammaCorrection {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u16> for GammaCorrection {
    type Error = Error;

    fn try_from(val: u16) -> Result<Self> {
        Self::try_from_factor(val)
    }
}

impl TryFrom<&str> for GammaCorrection {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let pos = val.find(GAMMA_SUFFIX).ok_or(Error::InvalidVariant)?;
        let exp_start = val[..pos]
            .rfind(|c: char| c.is_ascii_uppercase() || c.is_ascii_lowercase())
            .map(|s| s + 1)
            .unwrap_or(0);

        val[exp_start..pos]
            .parse::<u16>()
            .map_err(|_| Error::InvalidVariant)
            .and_then(Self::try_from_factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (0..=GAMMA_MAX).for_each(|factor| {
            let exp_factor = GammaCorrection { factor };

            assert_eq!(GammaCorrection::try_from_factor(factor), Ok(exp_factor));
            assert_eq!(exp_factor.factor(), factor);
        });
    }

    #[test]
    fn test_invalid() {
        ((GAMMA_MAX + 1)..=u16::MAX).for_each(|factor| {
            let err = Error::InvalidValue(factor as usize);

            assert_eq!(GammaCorrection::try_from_factor(factor), Err(err));
            assert_eq!(GammaCorrection::try_from(factor), Err(err));
        });
    }
}
