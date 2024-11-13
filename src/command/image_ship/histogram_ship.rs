use crate::result::{Error, Result};

const HISTO_NOSHIP: &str = "0W";
const HISTO_SHIP: &str = "1W";

/// Sets the image ship histogram ship.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HistogramShip {
    /// Histogram don't ship.
    Off,
    /// Histogram ship.
    On,
}

impl HistogramShip {
    /// Creates a new [HistogramShip].
    pub const fn new() -> Self {
        Self::Off
    }

    /// Gets the ASCII serial command code for [HistogramShip].
    pub const fn command(&self) -> &str {
        match self {
            Self::Off => HISTO_NOSHIP,
            Self::On => HISTO_SHIP,
        }
    }
}

impl Default for HistogramShip {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for HistogramShip {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(HISTO_NOSHIP) => Ok(Self::Off),
            v if v.contains(HISTO_SHIP) => Ok(Self::On),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [HistogramShip::Off, HistogramShip::On]
            .into_iter()
            .zip([HISTO_NOSHIP, HISTO_SHIP])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(HistogramShip::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
