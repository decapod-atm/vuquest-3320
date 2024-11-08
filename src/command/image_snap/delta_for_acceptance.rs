use alloc::string::String;

use crate::result::{Error, Result};

const DFA_SUFFIX: &str = "D";
const DFA_DEFAULT_GEN6: u8 = 25;
const DFA_DEFAULT_GEN7: u8 = 30;

/// Represents the image snap delta for acceptance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeltaForAcceptance {
    delta: u8,
}

impl DeltaForAcceptance {
    /// Creates a new [DeltaForAcceptance].
    pub const fn new() -> Self {
        Self {
            delta: DFA_DEFAULT_GEN6,
        }
    }

    /// Creates a new [DeltaForAcceptance] (GEN 7).
    pub const fn new_gen7() -> Self {
        Self {
            delta: DFA_DEFAULT_GEN7,
        }
    }

    /// Gets the [DeltaForAcceptance] delta setting.
    pub const fn delta(&self) -> u8 {
        self.delta
    }

    /// Creates a [DeltaForAcceptance] from a delta parameter.
    pub const fn from_delta(delta: u8) -> Self {
        Self { delta }
    }

    /// Gets the ASCII serial command code for [DeltaForAcceptance].
    pub fn command(&self) -> String {
        let delta = self.delta;
        format!("{delta}{DFA_SUFFIX}")
    }
}

impl Default for DeltaForAcceptance {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for DeltaForAcceptance {
    fn from(val: u8) -> Self {
        Self::from_delta(val)
    }
}

impl TryFrom<&str> for DeltaForAcceptance {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let pos = val.find(DFA_SUFFIX).ok_or(Error::InvalidVariant)?;
        let exp_start = val[..pos]
            .rfind(|c: char| c.is_ascii_uppercase() || c == '%')
            .map(|s| s + 1)
            .unwrap_or(0);

        val[exp_start..pos]
            .parse::<u8>()
            .map(Self::from_delta)
            .map_err(|_| Error::InvalidVariant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (0..=u8::MAX).for_each(|delta| {
            let exp_delta = DeltaForAcceptance { delta };

            assert_eq!(DeltaForAcceptance::from_delta(delta), exp_delta);
            assert_eq!(exp_delta.delta(), delta);
        });
    }
}
