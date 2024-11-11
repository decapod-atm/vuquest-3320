use crate::result::{Error, Result};

const PIXEL_SHIP1: &str = "1S";
const PIXEL_SHIP2: &str = "2S";
const PIXEL_SHIP3: &str = "3S";

/// Sets the image ship pixel ship.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PixelShip {
    /// Pixel Ship skip 1 pixel evey 1 line.
    Skip1,
    /// Pixel Ship skip 2 pixel evey 2 line.
    Skip2,
    /// Pixel Ship skip 3 pixel evey 3 line.
    Skip3,
}

impl PixelShip {
    /// Creates a new [PixelShip].
    pub const fn new() -> Self {
        Self::Skip1
    }

    /// Gets the ASCII serial command code for [PixelShip].
    pub const fn command(&self) -> &str {
        match self {
            Self::Skip1 => PIXEL_SHIP1,
            Self::Skip2 => PIXEL_SHIP2,
            Self::Skip3 => PIXEL_SHIP3,
        }
    }
}

impl Default for PixelShip {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for PixelShip {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(PIXEL_SHIP1) => Ok(Self::Skip1),
            v if v.contains(PIXEL_SHIP2) => Ok(Self::Skip2),
            v if v.contains(PIXEL_SHIP3) => Ok(Self::Skip3),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [PixelShip::Skip1, PixelShip::Skip2, PixelShip::Skip3]
            .into_iter()
            .zip([PIXEL_SHIP1, PIXEL_SHIP2, PIXEL_SHIP3])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(PixelShip::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
