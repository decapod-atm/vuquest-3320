use crate::result::{Error, Result};

const DEPTH_8BIT: &str = "8D";
const DEPTH_1BIT: &str = "1D";

/// Sets the image ship pixel depth.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PixelDepth {
    /// 8 bits per pixel, grayscale.
    Bit8,
    /// 1 bit per pixel, black and white.
    Bit1,
}

impl PixelDepth {
    /// Creates a new [PixelDepth].
    pub const fn new() -> Self {
        Self::Bit8
    }

    /// Gets the ASCII serial command code for [PixelDepth].
    pub const fn command(&self) -> &str {
        match self {
            Self::Bit8 => DEPTH_8BIT,
            Self::Bit1 => DEPTH_1BIT,
        }
    }
}

impl Default for PixelDepth {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for PixelDepth {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(DEPTH_8BIT) => Ok(Self::Bit8),
            v if v.contains(DEPTH_1BIT) => Ok(Self::Bit1),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [PixelDepth::Bit8, PixelDepth::Bit1]
            .into_iter()
            .zip([DEPTH_8BIT, DEPTH_1BIT])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(PixelDepth::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
