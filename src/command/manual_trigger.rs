use crate::result::{Error, Result};

const NORMAL: &str = "PAPHHF";
const ENHANCED: &str = "PAPHHS";

/// Represents the `Manual Trigger Mode` serial command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManualTriggerMode {
    Normal,
    Enhanced,
}

impl ManualTriggerMode {
    /// Creates a new [ManualTriggerMode].
    pub const fn new() -> Self {
        Self::Normal
    }

    /// Gets the ASCII serial command code for [ManualTriggerMode].
    pub const fn command(&self) -> &str {
        match self {
            Self::Normal => NORMAL,
            Self::Enhanced => ENHANCED,
        }
    }
}

impl Default for ManualTriggerMode {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for ManualTriggerMode {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(NORMAL) => Ok(Self::Normal),
            v if v.contains(ENHANCED) => Ok(Self::Enhanced),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [ManualTriggerMode::Normal, ManualTriggerMode::Enhanced]
            .into_iter()
            .zip([NORMAL, ENHANCED])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(ManualTriggerMode::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
