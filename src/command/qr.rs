use crate::result::{Error, Result};

const DEFAULT_SETTINGS: &str = "QRCDFT";
const QR_OFF: &str = "QRCENA0";
const QR_ON: &str = "QRCENA1";

/// Represents the `QR Code` serial command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QRCode {
    DefaultSettings,
    Off,
    On,
}

impl QRCode {
    /// Creates a new [QRCode].
    pub const fn new() -> Self {
        Self::On
    }

    /// Gets the ASCII serial command code for [QRCode].
    pub const fn command(&self) -> &str {
        match self {
            Self::DefaultSettings => DEFAULT_SETTINGS,
            Self::Off => QR_OFF,
            Self::On => QR_ON,
        }
    }
}

impl Default for QRCode {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for QRCode {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(DEFAULT_SETTINGS) => Ok(Self::DefaultSettings),
            v if v.contains(QR_ON) => Ok(Self::On),
            v if v.contains(QR_OFF) => Ok(Self::Off),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [QRCode::DefaultSettings, QRCode::Off, QRCode::On]
            .into_iter()
            .zip([DEFAULT_SETTINGS, QR_OFF, QR_ON])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(QRCode::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
