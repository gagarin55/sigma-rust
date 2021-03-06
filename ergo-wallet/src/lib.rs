//! Ergo wallet

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]

mod ergo_state_context;
mod secret_key;

pub use ergo_state_context::*;
pub use secret_key::*;
