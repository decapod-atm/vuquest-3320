use crate::result::{Error, Result};

const NOISE_OFF: &str = "0if";
const NOISE_ON: &str = "1if";

/// Sets the image ship noise reduction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NoiseReduction {
    /// Noise reduction off.
    Off,
    /// Salt and pepper noise reduction on.
    On,
}

impl NoiseReduction {
    /// Creates a new [NoiseReduction].
    pub const fn new() -> Self {
        Self::Off
    }

    /// Gets the ASCII serial command code for [NoiseReduction].
    pub const fn command(&self) -> &str {
        match self {
            Self::Off => NOISE_OFF,
            Self::On => NOISE_ON,
        }
    }
}

impl Default for NoiseReduction {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for NoiseReduction {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(NOISE_OFF) => Ok(Self::Off),
            v if v.contains(NOISE_ON) => Ok(Self::On),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [NoiseReduction::Off, NoiseReduction::On]
            .into_iter()
            .zip([NOISE_OFF, NOISE_ON])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(NoiseReduction::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
