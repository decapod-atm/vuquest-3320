use alloc::string::String;

use crate::result::{Error, Result};

const DOCUMENT_SUFFIX: &str = "U";
const DOCUMENT_DEFAULT: u8 = 0;

/// Represents the image ship document filter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DocumentFilter {
    threshold: u8,
}

impl DocumentFilter {
    /// Creates a new [DocumentFilter].
    pub const fn new() -> Self {
        Self {
            threshold: DOCUMENT_DEFAULT,
        }
    }

    /// Gets the [DocumentFilter] threshold setting.
    pub const fn threshold(&self) -> u8 {
        self.threshold
    }

    /// Creates a [DocumentFilter] from a threshold parameter.
    pub const fn from_threshold(threshold: u8) -> Self {
        Self { threshold }
    }

    /// Gets the ASCII serial command code for [DocumentFilter].
    pub fn command(&self) -> String {
        let threshold = self.threshold;
        format!("{threshold}{DOCUMENT_SUFFIX}")
    }
}

impl Default for DocumentFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for DocumentFilter {
    fn from(val: u8) -> Self {
        Self::from_threshold(val)
    }
}

impl TryFrom<&str> for DocumentFilter {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        let pos = val.find(DOCUMENT_SUFFIX).ok_or(Error::InvalidVariant)?;
        let exp_start = val[..pos]
            .rfind(|c: char| c.is_ascii_uppercase() || c.is_ascii_lowercase())
            .map(|s| s + 1)
            .unwrap_or(0);

        val[exp_start..pos]
            .parse::<u8>()
            .map(Self::from_threshold)
            .map_err(|_| Error::InvalidVariant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (0..=u8::MAX).for_each(|threshold| {
            let exp_threshold = DocumentFilter { threshold };

            assert_eq!(DocumentFilter::from_threshold(threshold), exp_threshold);
            assert_eq!(DocumentFilter::from(threshold), exp_threshold);
            assert_eq!(exp_threshold.threshold(), threshold);
        });
    }
}
