mod app;
pub use app::*;

mod plugin;
pub use bevy_ecs::{
	component::Component,
	schedule::SystemSet,
	system::{Resource, SystemParam},
};
pub use plugin::*;
