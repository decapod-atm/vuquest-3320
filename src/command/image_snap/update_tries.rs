use alloc::string::String;

use crate::result::{Error, Result};

const TRIES_SUFFIX: &str = "U";
const TRIES_DEFAULT: u8 = 6;
const TRIES_MAX: u8 = 10;

/// Represents the image snap update tries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UpdateTries {
    tries: u8,
}

impl UpdateTries {
    /// Creates a new [UpdateTries].
    pub const fn new() -> Self {
        Self {
            tries: TRIES_DEFAULT,
        }
    }

    /// Gets the [UpdateTries] tries setting.
    pub const fn tries(&self) -> u8 {
        self.tries
    }

    /// Creates a [UpdateTries] from a tries parameter.
    pub const fn try_from_tries(tries: u8) -> Result<Self> {
        if tries <= TRIES_MAX {
            Ok(Self { tries })
        } else {
            Err(Error::InvalidVariant)
        }
    }

    /// Gets the ASCII serial command code for [UpdateTries].
    pub fn command(&self) -> String {
        let tries = self.tries;
        format!("{tries}{TRIES_SUFFIX}")
    }
}

impl Default for UpdateTries {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for UpdateTries {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        Self::try_from_tries(val)
    }
}

impl TryFrom<&str> for UpdateTries {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let pos = val.find(TRIES_SUFFIX).ok_or(Error::InvalidVariant)?;
        let exp_start = val[..pos]
            .rfind(|c: char| c.is_ascii_uppercase() || c == '%')
            .map(|s| s + 1)
            .unwrap_or(0);

        val[exp_start..pos]
            .parse::<u8>()
            .map_err(|_| Error::InvalidVariant)
            .and_then(Self::try_from_tries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (0..=TRIES_MAX).for_each(|tries| {
            let exp_tries = UpdateTries { tries };

            assert_eq!(UpdateTries::try_from_tries(tries), Ok(exp_tries));
            assert_eq!(exp_tries.tries(), tries);
        });
    }

    #[test]
    fn test_invalid() {
        ((TRIES_MAX + 1)..=u8::MAX).for_each(|tries| {
            assert_eq!(
                UpdateTries::try_from_tries(tries),
                Err(Error::InvalidVariant)
            );
        });
    }
}
