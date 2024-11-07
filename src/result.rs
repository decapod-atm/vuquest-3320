//! Result and error types for the library.

use core::fmt;

/// Convenience alias for the library [Result](core::result::Result) type.
pub type Result<T> = core::result::Result<T, Error>;

/// Represents error conditions for the library.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidVariant,
    InvalidValue(usize),
}

impl Error {
    /// Creates a new [Error].
    pub const fn new() -> Self {
        Self::InvalidVariant
    }
}

impl Default for Error {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidVariant => write!(f, "invalid variant"),
            Self::InvalidValue(err) => write!(f, "invalid value: {err}"),
        }
    }
}

impl core::error::Error for Error {}
