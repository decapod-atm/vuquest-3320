use crate::result::{Error, Result};

// <SYN>T<CR>
const TRIGGER_ACTIVATE: &str = "\x16T\x0d";
// <SYN>U<CR>
const TRIGGER_DEACTIVATE: &str = "\x16U\x0d";

/// Represents the `Mobile Phone Read Mode` serial command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Trigger {
    Activate,
    Deactivate,
}

impl Trigger {
    /// Creates a new [Trigger].
    pub const fn new() -> Self {
        Self::Activate
    }

    /// Gets the ASCII serial command code for [Trigger].
    pub const fn command(&self) -> &str {
        match self {
            Self::Activate => TRIGGER_ACTIVATE,
            Self::Deactivate => TRIGGER_DEACTIVATE,
        }
    }
}

impl Default for Trigger {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for Trigger {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(TRIGGER_ACTIVATE) => Ok(Self::Activate),
            v if v.contains(TRIGGER_DEACTIVATE) => Ok(Self::Deactivate),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [Trigger::Activate, Trigger::Deactivate]
            .into_iter()
            .zip([TRIGGER_ACTIVATE, TRIGGER_DEACTIVATE])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(Trigger::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
