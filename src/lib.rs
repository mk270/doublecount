//! A double entry accounting system.
//! 
//! The doublecount package has the following optional cargo features:
//! 
//! + `serde-support`
//!   + Optional
//!   + Enable support for serialization/de-serialization via `serde`
//!   + Enable support for json serialization/de-serialization via `serde_json`

extern crate chrono;
extern crate nanoid;
extern crate rust_decimal;
extern crate commodity;
extern crate thiserror;

#[cfg(feature = "serde-support")]
extern crate serde;
#[cfg(feature = "serde-support")]
extern crate serde_json;

pub mod accounting;
pub mod exchange_rate;

#[doc(no_inline)] pub use accounting::*;
