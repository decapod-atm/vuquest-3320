use crate::result::{Error, Result};

const LED_OFF: &str = "0L";
const LED_ON: &str = "1L";

/// Sets the image snap beeper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LED {
    /// LEDs off.
    Off,
    /// LEDs on.
    On,
}

impl LED {
    /// Creates a new [LED].
    pub const fn new() -> Self {
        Self::Off
    }

    /// Gets the ASCII serial command code for [LED].
    pub const fn command(&self) -> &str {
        match self {
            Self::Off => LED_OFF,
            Self::On => LED_ON,
        }
    }
}

impl Default for LED {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for LED {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(LED_OFF) => Ok(Self::Off),
            v if v.contains(LED_ON) => Ok(Self::On),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [LED::Off, LED::On]
            .into_iter()
            .zip([LED_OFF, LED_ON])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(LED::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
