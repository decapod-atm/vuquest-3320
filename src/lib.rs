//! Serial protocol for communicating with Honeywell BCS devices.

#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub mod command;
pub mod result;
