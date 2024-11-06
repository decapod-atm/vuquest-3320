use crate::result::{Error, Result};

const HANDHELD_SCANNING: &str = "PAPHHC";
const STREAMING_PRESENTATION: &str = "PAPSPC";

/// Represents the `Mobile Phone Read Mode` serial command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MobilePhoneReadMode {
    HandheldScanning,
    StreamingPresentation,
}

impl MobilePhoneReadMode {
    /// Creates a new [MobilePhoneReadMode].
    pub const fn new() -> Self {
        Self::HandheldScanning
    }

    /// Gets the ASCII serial command code for [MobilePhoneReadMode].
    pub const fn command(&self) -> &str {
        match self {
            Self::HandheldScanning => HANDHELD_SCANNING,
            Self::StreamingPresentation => STREAMING_PRESENTATION,
        }
    }
}

impl Default for MobilePhoneReadMode {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for MobilePhoneReadMode {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(HANDHELD_SCANNING) => Ok(Self::HandheldScanning),
            v if v.contains(STREAMING_PRESENTATION) => Ok(Self::StreamingPresentation),
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
            MobilePhoneReadMode::HandheldScanning,
            MobilePhoneReadMode::StreamingPresentation,
        ]
        .into_iter()
        .zip([HANDHELD_SCANNING, STREAMING_PRESENTATION])
        .for_each(|(cmd, exp_ascii_cmd)| {
            assert_eq!(cmd.command(), exp_ascii_cmd);
            assert_eq!(MobilePhoneReadMode::try_from(exp_ascii_cmd), Ok(cmd));
        });
    }
}
