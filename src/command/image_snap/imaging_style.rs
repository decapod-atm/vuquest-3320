use crate::result::{Error, Result};

const DECODING_STYLE: &str = "0P";
const PHOTO_STYLE: &str = "1P";
const MANUAL_STYLE: &str = "2P";

/// Sets the image snap style.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImagingStyle {
    /// Allows a few frames of exposure before taking the photo.
    Decoding,
    /// Mimics a digital camera.
    Photo,
    /// Advanced style that allows freedom to set up the scanner, and manually control exposure.
    Manual,
}

impl ImagingStyle {
    /// Creates a new [ImagingStyle].
    pub const fn new() -> Self {
        Self::Photo
    }

    /// Gets the ASCII serial command code for [ImagingStyle].
    pub const fn command(&self) -> &str {
        match self {
            Self::Decoding => DECODING_STYLE,
            Self::Photo => PHOTO_STYLE,
            Self::Manual => MANUAL_STYLE,
        }
    }
}

impl Default for ImagingStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for ImagingStyle {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(DECODING_STYLE) => Ok(Self::Decoding),
            v if v.contains(PHOTO_STYLE) => Ok(Self::Photo),
            v if v.contains(MANUAL_STYLE) => Ok(Self::Manual),
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
            ImagingStyle::Decoding,
            ImagingStyle::Photo,
            ImagingStyle::Manual,
        ]
        .into_iter()
        .zip([DECODING_STYLE, PHOTO_STYLE, MANUAL_STYLE])
        .for_each(|(cmd, exp_ascii_cmd)| {
            assert_eq!(cmd.command(), exp_ascii_cmd);
            assert_eq!(ImagingStyle::try_from(exp_ascii_cmd), Ok(cmd));
        });
    }
}
