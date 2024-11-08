use crate::result::{Error, Result};

const COMPENSATION_DISABLED: &str = "0C";
const COMPENSATION_ENABLED: &str = "1C";

/// Sets the image ship compensation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Compensation {
    /// Compensation disabled.
    Disabled,
    /// Compensation enabled.
    Enabled,
}

impl Compensation {
    /// Creates a new [Compensation].
    pub const fn new() -> Self {
        Self::Disabled
    }

    /// Gets the ASCII serial command code for [Compensation].
    pub const fn command(&self) -> &str {
        match self {
            Self::Disabled => COMPENSATION_DISABLED,
            Self::Enabled => COMPENSATION_ENABLED,
        }
    }
}

impl Default for Compensation {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for Compensation {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(COMPENSATION_DISABLED) => Ok(Self::Disabled),
            v if v.contains(COMPENSATION_ENABLED) => Ok(Self::Enabled),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [Compensation::Disabled, Compensation::Enabled]
            .into_iter()
            .zip([COMPENSATION_DISABLED, COMPENSATION_ENABLED])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(Compensation::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
