//! Miscellaneous data structures in Rust
//!
//! Generally, behavior/functionality is meant to match the expectations of the original C++ version,
//! with some common caveats:
//!
//! - Rust doesn't allow for non-move construction, so separate copy/move constructors and functions don't make sense,
//!   and only the move version will be implemented.
//! - default constructor → [Default trait](core::default::Default).
//! - copy constructor → [Clone trait](core::clone::Clone).
//! - destructor → [Drop trait](core::ops::Drop),
//! - If some behavior can be easily and idiomatically implemented via a trait, the trait will be used instead of a function.
//!     - When this happens, the trait's main function will have a `doc::alias` attribute applied, to make it appear in search.

#![no_std]
#![warn(missing_docs)]
#![deny(
    unsafe_code,
    reason = "unsafe code should be performed in sub-modules to minimize risk"
)]
#![deny(clippy::undocumented_unsafe_blocks)]

extern crate alloc;
#[cfg(test)] // during tests, allow std for print and such
extern crate std;

pub mod my_box;
