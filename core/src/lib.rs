//! Hardware-independent logic for `canary`.

#![no_std]

// Host-only, so the tests can use `String`, `ToString` and friends.
#[cfg(test)]
extern crate std;

/// Error handler implementation.
pub mod error;

/// Helper functions.
pub mod help;

/// Widget renderer.
pub mod ui;

/// Common types that can be glob-imported for convenience.
pub mod prelude;
