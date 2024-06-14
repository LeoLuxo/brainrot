#![allow(unused_imports)]

pub mod engine_3d;
pub mod math;

// bevy
#[cfg(feature = "bevy")]
pub mod bevy;

// Import macros and re-export under the brainrot::... namespace
#[cfg(feature = "macros")]
mod macros;
#[cfg(feature = "macros")]
pub use crate::macros::*;

// Re-export vek under brainrot::vek
#[cfg(feature = "vek")]
pub use vek;
