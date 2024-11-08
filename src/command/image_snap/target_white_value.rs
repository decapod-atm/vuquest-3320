use alloc::string::String;

use crate::result::{Error, Result};

const TWV_SUFFIX: &str = "W";
const TWV_DEFAULT: u8 = 125;

/// Represents the image snap target white value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetWhiteValue {
    value: u8,
}

impl TargetWhiteValue {
    /// Creates a new [TargetWhiteValue].
    pub const fn new() -> Self {
        Self { value: TWV_DEFAULT }
    }

    /// Gets the [TargetWhiteValue] value setting.
    pub const fn value(&self) -> u8 {
        self.value
    }

    /// Creates a [TargetWhiteValue] from a value parameter.
    pub const fn from_value(value: u8) -> Self {
        Self { value }
    }

    /// Gets the ASCII serial command code for [TargetWhiteValue].
    pub fn command(&self) -> String {
        let value = self.value;
        format!("{value}{TWV_SUFFIX}")
    }
}

impl Default for TargetWhiteValue {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for TargetWhiteValue {
    fn from(val: u8) -> Self {
        Self::from_value(val)
    }
}

impl TryFrom<&str> for TargetWhiteValue {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let pos = val.find(TWV_SUFFIX).ok_or(Error::InvalidVariant)?;
        let exp_start = val[..pos]
            .rfind(|c: char| c.is_ascii_uppercase() || c == '%')
            .map(|s| s + 1)
            .unwrap_or(0);

        val[exp_start..pos]
            .parse::<u8>()
            .map(Self::from_value)
            .map_err(|_| Error::InvalidVariant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (0..=u8::MAX).for_each(|value| {
            let exp_value = TargetWhiteValue { value };

            assert_eq!(TargetWhiteValue::from_value(value), exp_value);
            assert_eq!(exp_value.value(), value);
        });
    }
}
