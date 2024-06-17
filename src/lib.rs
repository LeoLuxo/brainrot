#![allow(unused_imports)]
#![allow(dead_code)]

macro_rules! reexport_feature_module {
	($module:ident) => {
		paste::paste! {
			#[macro_use]
			#[cfg(feature = "" $module "")]
			mod $module;
			#[cfg(feature = "" $module "")]
			pub use $module::*;
		}
	};
}

// Re-export certain crates needed by brainrot so that they can be used in macros
pub mod lib_crates {
	pub use paste;
	pub use phf;
}

mod util;
pub use util::*;

// bevy
#[cfg(feature = "bevy")]
pub mod bevy;
