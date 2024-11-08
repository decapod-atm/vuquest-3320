use crate::result::{Error, Result};

const HISTO_OFF: &str = "0H";
const HISTO_ON: &str = "1H";

/// Sets the image ship histogram stretch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistogramStretch {
    /// HistogramStretch disabled.
    Off,
    /// HistogramStretch enabled.
    On,
}

impl HistogramStretch {
    /// Creates a new [HistogramStretch].
    pub const fn new() -> Self {
        Self::Off
    }

    /// Gets the ASCII serial command code for [HistogramStretch].
    pub const fn command(&self) -> &str {
        match self {
            Self::Off => HISTO_OFF,
            Self::On => HISTO_ON,
        }
    }
}

impl Default for HistogramStretch {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for HistogramStretch {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(HISTO_OFF) => Ok(Self::Off),
            v if v.contains(HISTO_ON) => Ok(Self::On),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [HistogramStretch::Off, HistogramStretch::On]
            .into_iter()
            .zip([HISTO_OFF, HISTO_ON])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(HistogramStretch::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
