use alloc::string::String;

use crate::result::{Error, Result};

const SERIAL_TRIGGER: &str = "TRGSTO";
/// Maximum timeout (in milliseconds) for [SerialTriggerMode].
pub const MAX_SERIAL_TRIGGER: u32 = 300_000;

/// Represents the `Serial Trigger Mode` serial command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerialTriggerMode {
    ms: u32,
}

impl SerialTriggerMode {
    /// Creates a new [SerialTriggerMode].
    pub const fn new() -> Self {
        Self { ms: 0 }
    }

    /// Gets the ASCII serial command code for [SerialTriggerMode].
    pub fn command(&self) -> String {
        format!("{SERIAL_TRIGGER}{}", self.ms)
    }

    /// Attempts to convert a [`u32`] number of timeout milliseconds into a [SerialTriggerMode].
    ///
    /// **NOTE**: `ms` must be below [MAX_SERIAL_TRIGGER] number of milliseconds.
    pub const fn try_from_ms(ms: u32) -> Result<Self> {
        if ms <= MAX_SERIAL_TRIGGER {
            Ok(Self { ms })
        } else {
            Err(Error::InvalidValue(ms as usize))
        }
    }

    /// Converts a [SerialTriggerMode] into a [`u32`] number of milliseconds.
    pub const fn into_ms(self) -> u32 {
        self.ms
    }
}

impl Default for SerialTriggerMode {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for SerialTriggerMode {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let i = val.find(SERIAL_TRIGGER).ok_or(Error::InvalidVariant)?;

        val.get(i + SERIAL_TRIGGER.len()..)
            .ok_or(Error::InvalidVariant)?
            .parse::<u32>()
            .map(|ms| Self { ms })
            .map_err(|_| Error::InvalidVariant)
    }
}

impl TryFrom<String> for SerialTriggerMode {
    type Error = Error;

    fn try_from(val: String) -> Result<Self> {
        val.as_str().try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (0..=MAX_SERIAL_TRIGGER).for_each(|ms| {
            let exp_cmd = SerialTriggerMode { ms };
            let exp_ascii_cmd = format!("{SERIAL_TRIGGER}{ms}");

            assert_eq!(SerialTriggerMode::try_from_ms(ms), Ok(exp_cmd));
            assert_eq!(exp_cmd.command(), exp_ascii_cmd);
            assert_eq!(SerialTriggerMode::try_from(exp_ascii_cmd), Ok(exp_cmd));
        });
    }
}
