//! Miscellaneous data structures in Rust

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
