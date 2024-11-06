use crate::result::{Error, Result};

const ALL_SYM_OFF: &str = "ALLENA0";
const ALL_SYM_ON: &str = "ALLENA1";

/// Configure all barcode `Symbology` encodings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AllSymbologies {
    Off = 0,
    On = 1,
}

impl AllSymbologies {
    /// Creates a new [AllSymbologies].
    pub const fn new() -> Self {
        Self::Off
    }

    /// Gets the ASCII serial command code for [AllSymbologies].
    pub const fn command(&self) -> &str {
        match self {
            Self::Off => ALL_SYM_OFF,
            Self::On => ALL_SYM_ON,
        }
    }
}

impl Default for AllSymbologies {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for AllSymbologies {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(ALL_SYM_OFF) => Ok(Self::Off),
            v if v.contains(ALL_SYM_ON) => Ok(Self::On),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [AllSymbologies::Off, AllSymbologies::On]
            .into_iter()
            .zip([ALL_SYM_OFF, ALL_SYM_ON])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(AllSymbologies::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
