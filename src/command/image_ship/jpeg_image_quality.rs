use alloc::string::String;

use crate::result::{Error, Result};

const JPEG_SUFFIX: &str = "J";
const JPEG_DEFAULT: u8 = 50;
const JPEG_MAX: u8 = 100;

/// Represents the image ship JPEG image quality.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JpegImageQuality {
    quality: u8,
}

impl JpegImageQuality {
    /// Creates a new [JpegImageQuality].
    pub const fn new() -> Self {
        Self {
            quality: JPEG_DEFAULT,
        }
    }

    /// Gets the [JpegImageQuality] quality setting.
    pub const fn quality(&self) -> u8 {
        self.quality
    }

    /// Creates a [JpegImageQuality] from a quality parameter.
    pub const fn try_from_quality(quality: u8) -> Result<Self> {
        match quality {
            s if s <= JPEG_MAX => Ok(Self { quality }),
            _ => Err(Error::InvalidValue(quality as usize)),
        }
    }

    /// Gets the ASCII serial command code for [JpegImageQuality].
    pub fn command(&self) -> String {
        let quality = self.quality;
        format!("{quality}{JPEG_SUFFIX}")
    }
}

impl Default for JpegImageQuality {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<u8> for JpegImageQuality {
    type Error = Error;

    fn try_from(val: u8) -> Result<Self> {
        Self::try_from_quality(val)
    }
}

impl TryFrom<&str> for JpegImageQuality {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let pos = val.find(JPEG_SUFFIX).ok_or(Error::InvalidVariant)?;
        let exp_start = val[..pos]
            .rfind(|c: char| c.is_ascii_uppercase() || c.is_ascii_lowercase())
            .map(|s| s + 1)
            .unwrap_or(0);

        val[exp_start..pos]
            .parse::<u8>()
            .map_err(|_| Error::InvalidVariant)
            .and_then(Self::try_from_quality)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (0..=JPEG_MAX).for_each(|quality| {
            let exp_quality = JpegImageQuality { quality };

            assert_eq!(JpegImageQuality::try_from_quality(quality), Ok(exp_quality));
            assert_eq!(exp_quality.quality(), quality);
        });
    }

    #[test]
    fn test_invalid() {
        ((JPEG_MAX + 1)..=u8::MAX).for_each(|quality| {
            let err = Error::InvalidValue(quality as usize);

            assert_eq!(JpegImageQuality::try_from_quality(quality), Err(err));
            assert_eq!(JpegImageQuality::try_from(quality), Err(err));
        });
    }
}
