use crate::result::{Error, Result};

const INVERT_X: &str = "1ix";
const INVERT_Y: &str = "1iy";

/// Sets the image ship invert image.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InvertImage {
    /// Invert around the X-axis.
    X,
    /// Invert around the Y-axis.
    Y,
}

impl InvertImage {
    /// Creates a new [InvertImage].
    pub const fn new() -> Self {
        Self::X
    }

    /// Gets the ASCII serial command code for [InvertImage].
    pub const fn command(&self) -> &str {
        match self {
            Self::X => INVERT_X,
            Self::Y => INVERT_Y,
        }
    }
}

impl Default for InvertImage {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for InvertImage {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(INVERT_X) => Ok(Self::X),
            v if v.contains(INVERT_Y) => Ok(Self::Y),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [InvertImage::X, InvertImage::Y]
            .into_iter()
            .zip([INVERT_X, INVERT_Y])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(InvertImage::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
