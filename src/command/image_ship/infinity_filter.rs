use crate::result::{Error, Result};

const FILTER_OFF: &str = "0A";
const FILTER_ON: &str = "1A";

/// Sets the image ship infinity filter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfinityFilter {
    /// Infinity filter off.
    Off,
    /// Infinity filters on.
    On,
}

impl InfinityFilter {
    /// Creates a new [InfinityFilter].
    pub const fn new() -> Self {
        Self::Off
    }

    /// Gets the ASCII serial command code for [InfinityFilter].
    pub const fn command(&self) -> &str {
        match self {
            Self::Off => FILTER_OFF,
            Self::On => FILTER_ON,
        }
    }
}

impl Default for InfinityFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for InfinityFilter {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(FILTER_OFF) => Ok(Self::Off),
            v if v.contains(FILTER_ON) => Ok(Self::On),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [InfinityFilter::Off, InfinityFilter::On]
            .into_iter()
            .zip([FILTER_OFF, FILTER_ON])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(InfinityFilter::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
