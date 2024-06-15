#![allow(unused_imports)]
#![allow(dead_code)]

macro_rules! reexport_feature_module {
	($module:ident) => {
		// use $crate::paste::paste;
		paste::paste! {
			#[cfg(feature = "" $module "")]
			mod $module;
			#[cfg(feature = "" $module "")]
			pub use $module::*;
		}
	};
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub mod engine_3d;
pub mod math;

// bevy
#[cfg(feature = "bevy")]
pub mod bevy;

// Import macros and re-export under the brainrot::... namespace
reexport_feature_module!(macros);

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

// Re-export vek under brainrot::vek
#[cfg(feature = "vek")]
mod vek_temp {
	pub use vek;

	/// A screen size in pixels
	pub type ScreenSize = vek::Extent2<u32>;

	/// A delta of the mouse cursor movement over the last frame
	pub type MouseMotionDelta = vek::Vec2<f64>;
}

#[cfg(feature = "vek")]
pub use vek_temp::*;
