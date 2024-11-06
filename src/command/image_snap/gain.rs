use crate::result::{Error, Result};

const GAIN_OFF: &str = "1G";
const GAIN_MEDIUM: &str = "2G";
const GAIN_HEAVY: &str = "4G";
const GAIN_MAX: &str = "8G";

/// Sets the image snap gain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gain {
    /// No gain.
    Off,
    /// Medium gain.
    Medium,
    /// Heavy gain.
    Heavy,
    /// Maximum gain.
    Max,
}

impl Gain {
    /// Creates a new [Gain].
    pub const fn new() -> Self {
        Self::Off
    }

    /// Gets the ASCII serial command code for [Gain].
    pub const fn command(&self) -> &str {
        match self {
            Self::Off => GAIN_OFF,
            Self::Medium => GAIN_MEDIUM,
            Self::Heavy => GAIN_HEAVY,
            Self::Max => GAIN_MAX,
        }
    }
}

impl Default for Gain {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for Gain {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(GAIN_OFF) => Ok(Self::Off),
            v if v.contains(GAIN_MEDIUM) => Ok(Self::Medium),
            v if v.contains(GAIN_HEAVY) => Ok(Self::Heavy),
            v if v.contains(GAIN_MAX) => Ok(Self::Max),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [Gain::Off, Gain::Medium, Gain::Heavy, Gain::Max]
            .into_iter()
            .zip([GAIN_OFF, GAIN_MEDIUM, GAIN_HEAVY, GAIN_MAX])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(Gain::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
