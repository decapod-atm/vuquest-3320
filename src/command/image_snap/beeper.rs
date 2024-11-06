use crate::result::{Error, Result};

const BEEP_OFF: &str = "0B";
const BEEP_ON: &str = "1B";

/// Sets the image snap beeper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Beeper {
    /// No beep.
    Off,
    /// Beepers when image captured.
    On,
}

impl Beeper {
    /// Creates a new [Beeper].
    pub const fn new() -> Self {
        Self::Off
    }

    /// Gets the ASCII serial command code for [Beeper].
    pub const fn command(&self) -> &str {
        match self {
            Self::Off => BEEP_OFF,
            Self::On => BEEP_ON,
        }
    }
}

impl Default for Beeper {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for Beeper {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(BEEP_OFF) => Ok(Self::Off),
            v if v.contains(BEEP_ON) => Ok(Self::On),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [Beeper::Off, Beeper::On]
            .into_iter()
            .zip([BEEP_OFF, BEEP_ON])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(Beeper::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
