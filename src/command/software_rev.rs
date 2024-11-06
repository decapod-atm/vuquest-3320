use crate::result::{Error, Result};

const SOFTWARE_REVISION: &str = "REVINF";

/// Represents the `Software Revision` serial command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SoftwareRevision;

impl SoftwareRevision {
    /// Creates a new [SoftwareRevision].
    pub const fn new() -> Self {
        Self {}
    }

    /// Gets the ASCII serial command code for [SoftwareRevision].
    pub const fn command(&self) -> &str {
        SOFTWARE_REVISION
    }
}

impl Default for SoftwareRevision {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for SoftwareRevision {
    type Error = Error;

    fn try_from(val: &str) -> Result<Self> {
        match val {
            v if v.contains(SOFTWARE_REVISION) => Ok(Self::new()),
            _ => Err(Error::InvalidVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let cmd = SoftwareRevision::new();
        let exp_ascii_cmd = SOFTWARE_REVISION;

        assert_eq!(cmd.command(), exp_ascii_cmd);
        assert_eq!(SoftwareRevision::try_from(exp_ascii_cmd), Ok(cmd));
    }
}
