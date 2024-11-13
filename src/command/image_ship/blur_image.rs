use crate::result::{Error, Result};

const BLUR_OFF: &str = "0V";
const BLUR_ON: &str = "1V";

/// Sets the image ship blur image.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlurImage {
    /// Blur image off.
    Off,
    /// Blur image on.
    On,
}

impl BlurImage {
    /// Creates a new [BlurImage].
    pub const fn new() -> Self {
        Self::Off
    }

    /// Gets the ASCII serial command code for [BlurImage].
    pub const fn command(&self) -> &str {
        match self {
            Self::Off => BLUR_OFF,
            Self::On => BLUR_ON,
        }
    }
}

impl Default for BlurImage {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for BlurImage {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(BLUR_OFF) => Ok(Self::Off),
            v if v.contains(BLUR_ON) => Ok(Self::On),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [BlurImage::Off, BlurImage::On]
            .into_iter()
            .zip([BLUR_OFF, BLUR_ON])
            .for_each(|(cmd, exp_ascii_cmd)| {
                assert_eq!(cmd.command(), exp_ascii_cmd);
                assert_eq!(BlurImage::try_from(exp_ascii_cmd), Ok(cmd));
            });
    }
}
