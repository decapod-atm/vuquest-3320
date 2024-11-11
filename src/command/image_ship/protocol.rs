use crate::result::{Error, Result};

const PROTOCOL_RAW: &str = "0P";
const PROTOCOL_USB: &str = "2P";
const PROTOCOL_HMODEM_COMP: &str = "3P";
const PROTOCOL_HMODEM: &str = "4P";

/// Sets the image ship protocol.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Protocol {
    /// Protocol none, raw data.
    Raw,
    /// Protocol none, USB default.
    Usb,
    /// Protocol HMODEM compressed, RS232 default.
    HmodemCompressed,
    /// Protocol HMODEM.
    Hmodem,
}

impl Protocol {
    /// Creates a new [Protocol].
    pub const fn new() -> Self {
        Self::Raw
    }

    /// Gets the ASCII serial command code for [Protocol].
    pub const fn command(&self) -> &str {
        match self {
            Self::Raw => PROTOCOL_RAW,
            Self::Usb => PROTOCOL_USB,
            Self::HmodemCompressed => PROTOCOL_HMODEM_COMP,
            Self::Hmodem => PROTOCOL_HMODEM,
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for Protocol {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(PROTOCOL_RAW) => Ok(Self::Raw),
            v if v.contains(PROTOCOL_USB) => Ok(Self::Usb),
            v if v.contains(PROTOCOL_HMODEM_COMP) => Ok(Self::HmodemCompressed),
            v if v.contains(PROTOCOL_HMODEM) => Ok(Self::Hmodem),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [
            Protocol::Raw,
            Protocol::Usb,
            Protocol::HmodemCompressed,
            Protocol::Hmodem,
        ]
        .into_iter()
        .zip([
            PROTOCOL_RAW,
            PROTOCOL_USB,
            PROTOCOL_HMODEM_COMP,
            PROTOCOL_HMODEM,
        ])
        .for_each(|(cmd, exp_ascii_cmd)| {
            assert_eq!(cmd.command(), exp_ascii_cmd);
            assert_eq!(Protocol::try_from(exp_ascii_cmd), Ok(cmd));
        });
    }
}
