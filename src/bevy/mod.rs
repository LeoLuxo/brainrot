mod app;
pub use app::*;

mod plugin;

// Re-export
pub use bevy_ecs::{
	bundle::Bundle,
	component::Component,
	schedule::SystemSet,
	system::{Resource, SystemParam},
};
pub use plugin::*;
