use alloc::string::String;

use crate::result::{Error, Result};

const EDGE_SUFFIX: &str = "E";
const EDGE_DEFAULT: u8 = 0;
const EDGE_MAX: u8 = 24;

/// Represents the image ship edge sharpen.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EdgeSharpen {
    strength: u8,
}

impl EdgeSharpen {
    /// Creates a new [EdgeSharpen].
    pub const fn new() -> Self {
        Self {
            strength: EDGE_DEFAULT,
        }
    }

    /// Gets the [EdgeSharpen] strength setting.
    pub const fn strength(&self) -> u8 {
        self.strength
    }

    /// Creates a [EdgeSharpen] from a strength parameter.
    pub const fn try_from_strength(strength: u8) -> Result<Self> {
        match strength {
            s if s <= EDGE_MAX => Ok(Self { strength }),
            _ => Err(Error::InvalidValue(strength as usize)),
        }
    }

    /// Gets the ASCII serial command code for [EdgeSharpen].
    pub fn command(&self) -> String {
        let strength = self.strength;
        format!("{strength}{EDGE_SUFFIX}")
    }
}

impl Default for EdgeSharpen {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for EdgeSharpen {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        Self::try_from_strength(val)
    }
}

impl TryFrom<&str> for EdgeSharpen {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let pos = val.find(EDGE_SUFFIX).ok_or(Error::InvalidVariant)?;
        let exp_start = val[..pos]
            .rfind(|c: char| c.is_ascii_uppercase() || c.is_ascii_lowercase())
            .map(|s| s + 1)
            .unwrap_or(0);

        val[exp_start..pos]
            .parse::<u8>()
            .map_err(|_| Error::InvalidVariant)
            .and_then(Self::try_from_strength)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (0..=EDGE_MAX).for_each(|strength| {
            let exp_strength = EdgeSharpen { strength };

            assert_eq!(EdgeSharpen::try_from_strength(strength), Ok(exp_strength));
            assert_eq!(exp_strength.strength(), strength);
        });
    }

    #[test]
    fn test_invalid() {
        ((EDGE_MAX + 1)..=u8::MAX).for_each(|strength| {
            let err = Error::InvalidValue(strength as usize);

            assert_eq!(EdgeSharpen::try_from_strength(strength), Err(err));
            assert_eq!(EdgeSharpen::try_from(strength), Err(err));
        });
    }
}
