use crate::result::{Error, Result};

const ROTATE_0: &str = "0ir";
const ROTATE_90: &str = "1ir";
const ROTATE_180: &str = "2ir";
const ROTATE_270: &str = "3ir";

/// Sets the image ship image rotate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageRotate {
    /// Rotate 0 degrees (image snapped).
    Degrees0,
    /// Rotate 90 degrees to the right.
    Degrees90,
    /// Rotate 180 degrees.
    Degrees180,
    /// Rotate 270 degrees to the right (90 degrees left).
    Degrees270,
}

impl ImageRotate {
    /// Creates a new [ImageRotate].
    pub const fn new() -> Self {
        Self::Degrees0
    }

    /// Gets the ASCII serial command code for [ImageRotate].
    pub const fn command(&self) -> &str {
        match self {
            Self::Degrees0 => ROTATE_0,
            Self::Degrees90 => ROTATE_90,
            Self::Degrees180 => ROTATE_180,
            Self::Degrees270 => ROTATE_270,
        }
    }
}

impl Default for ImageRotate {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for ImageRotate {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(ROTATE_0) => Ok(Self::Degrees0),
            v if v.contains(ROTATE_90) => Ok(Self::Degrees90),
            v if v.contains(ROTATE_180) => Ok(Self::Degrees180),
            v if v.contains(ROTATE_270) => Ok(Self::Degrees270),
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
            ImageRotate::Degrees0,
            ImageRotate::Degrees90,
            ImageRotate::Degrees180,
            ImageRotate::Degrees270,
        ]
        .into_iter()
        .zip([ROTATE_0, ROTATE_90, ROTATE_180, ROTATE_270])
        .for_each(|(cmd, exp_ascii_cmd)| {
            assert_eq!(cmd.command(), exp_ascii_cmd);
            assert_eq!(ImageRotate::try_from(exp_ascii_cmd), Ok(cmd));
        });
    }
}
