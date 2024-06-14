#![allow(unused_imports)]

mod math;
pub use crate::math::*;

mod engine_3d;
pub use crate::engine_3d::*;

// bevy
#[cfg(feature = "bevy")]
mod bevy;
#[cfg(feature = "bevy")]
pub use crate::bevy::*;

// Import macros and re-export under the brainrot::... namespace
#[cfg(feature = "macros")]
mod macros;
#[cfg(feature = "macros")]
pub use crate::macros::*;

// Re-export vek under brainrot::vek
#[cfg(feature = "vek")]
pub use vek;
