use crate::result::{Error, Result};

const WAIT_OFF: &str = "0T";
const WAIT_ON: &str = "1T";

/// Sets the image snap wait for trigger.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaitForTrigger {
    /// No beep.
    NoWait,
    /// WaitForTriggers when image captured.
    Wait,
}

impl WaitForTrigger {
    /// Creates a new [WaitForTrigger].
    pub const fn new() -> Self {
        Self::NoWait
    }

    /// Gets the ASCII serial command code for [WaitForTrigger].
    pub const fn command(&self) -> &str {
        match self {
            Self::NoWait => WAIT_OFF,
            Self::Wait => WAIT_ON,
        }
    }
}

impl Default for WaitForTrigger {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for WaitForTrigger {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(WAIT_OFF) => Ok(Self::NoWait),
            v if v.contains(WAIT_ON) => Ok(Self::Wait),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [WaitForTrigger::NoWait, WaitForTrigger::Wait]
            .into_iter()
            .zip([WAIT_OFF, WAIT_ON])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(WaitForTrigger::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
