use crate::result::{Error, Result};

const DEFAULT_SETTINGS: &str = "PDFDFT";
const PDF_OFF: &str = "PDFENA0";
const PDF_ON: &str = "PDFENA1";

/// Represents the `Mobile Phone Read Mode` serial command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PDF417 {
    DefaultSettings,
    Off,
    On,
}

impl PDF417 {
    /// Creates a new [PDF417].
    pub const fn new() -> Self {
        Self::On
    }

    /// Gets the ASCII serial command code for [PDF417].
    pub const fn command(&self) -> &str {
        match self {
            Self::DefaultSettings => DEFAULT_SETTINGS,
            Self::Off => PDF_OFF,
            Self::On => PDF_ON,
        }
    }
}

impl Default for PDF417 {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for PDF417 {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(DEFAULT_SETTINGS) => Ok(Self::DefaultSettings),
            v if v.contains(PDF_ON) => Ok(Self::On),
            v if v.contains(PDF_OFF) => Ok(Self::Off),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [PDF417::DefaultSettings, PDF417::Off, PDF417::On]
            .into_iter()
            .zip([DEFAULT_SETTINGS, PDF_OFF, PDF_ON])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(PDF417::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
