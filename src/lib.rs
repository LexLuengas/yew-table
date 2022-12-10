#![deny(missing_docs)]
//! # yew-table
//!
//! A simple table component for the Yew web framework.

#[doc(hidden)]
#[macro_use]
pub mod macros;
pub mod types;
pub mod component;
pub mod error;

pub use self::types::*;
pub use self::component::*;
pub use self::error::*;
