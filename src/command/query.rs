const DEFAULT_VALUE: &str = "^";
const CURRENT_VALUE: &str = "?";
const RANGE_VALUE: &str = "*";

/// Represents special characters used to modify other serial commands to query command values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryCommand {
    /// Gets the default value for the command.
    DefaultValue,
    /// Gets the current value for the command.
    CurrentValue,
    /// Gets the range of acceptable values for the command.
    RangeValue,
}

impl QueryCommand {
    /// Creates a new [QueryCommand].
    pub const fn new() -> Self {
        Self::DefaultValue
    }

    /// Gets the ASCII-encoded serial command for the [QueryCommand].
    pub const fn command(&self) -> &str {
        match self {
            Self::DefaultValue => DEFAULT_VALUE,
            Self::CurrentValue => CURRENT_VALUE,
            Self::RangeValue => RANGE_VALUE,
        }
    }
}

impl Default for QueryCommand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        [
            QueryCommand::DefaultValue,
            QueryCommand::CurrentValue,
            QueryCommand::RangeValue,
        ]
        .into_iter()
        .zip([DEFAULT_VALUE, CURRENT_VALUE, RANGE_VALUE])
        .for_each(|(cmd, exp_ascii)| {
            assert_eq!(cmd.command(), exp_ascii);
        });
    }
}
