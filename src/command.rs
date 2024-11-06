//! Serial commands for configuring the BCS device.

use core::fmt;

use alloc::string::String;

pub mod image_snap;
mod manual_trigger;
mod mobile_phone;
mod pdf;
mod qr;
mod query;
mod serial_trigger;
mod software_rev;
mod symbologies;
mod trigger;

pub use image_snap::*;
pub use manual_trigger::*;
pub use mobile_phone::*;
pub use pdf::*;
pub use qr::*;
pub use query::*;
pub use serial_trigger::*;
pub use software_rev::*;
pub use symbologies::*;
pub use trigger::*;

/// Represents Honeywell BCS serial commands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SerialCommand {
    AllSymbologies(AllSymbologies),
    ImageSnap(ImageSnap),
    ManualTriggerMode(ManualTriggerMode),
    MobilePhoneReadMode(MobilePhoneReadMode),
    PDF417(PDF417),
    QRCode(QRCode),
    SoftwareRevision(SoftwareRevision),
    Trigger(Trigger),
}

impl SerialCommand {
    /// Creates a new [SerialCommand].
    pub const fn new() -> Self {
        Self::AllSymbologies(AllSymbologies::new())
    }

    /// Gets the ASCII-encoded [SerialCommand].
    pub fn command(&self) -> String {
        match self {
            Self::AllSymbologies(cmd) => cmd.command().into(),
            Self::ImageSnap(cmd) => cmd.command(),
            Self::ManualTriggerMode(cmd) => cmd.command().into(),
            Self::MobilePhoneReadMode(cmd) => cmd.command().into(),
            Self::PDF417(cmd) => cmd.command().into(),
            Self::QRCode(cmd) => cmd.command().into(),
            Self::SoftwareRevision(cmd) => cmd.command().into(),
            Self::Trigger(cmd) => cmd.command().into(),
        }
    }
}

impl Default for SerialCommand {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a Honeywell BCS serial command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Command {
    serial: SerialCommand,
    query: Option<QueryCommand>,
}

impl Command {
    /// Creates a new [Command].
    pub const fn new() -> Self {
        Self {
            serial: SerialCommand::new(),
            query: None,
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd = self.serial.command();
        let query = self
            .query
            .map(|q| String::from(q.command()))
            .unwrap_or_default();

        write!(f, "{cmd}{query}.")
    }
}
