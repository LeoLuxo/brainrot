use std::any::Any;

use downcast_rs::{impl_downcast, Downcast};

use super::app::App;

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// A collection of app logic and configuration.
/// This is a personal partial re-implementation of [`bevy_app::plugin::Plugin`] from `bevy_app`
///
/// Plugins configure a [`App`]. When a [`App`] registers a plugin,
/// the plugin's [`Plugin::build`] function is run. By default, a plugin
/// should only be added once to a [`App`].
/// (Since I removed the unique-ness mechanisms from the original plugins)
///
/// ## Lifecycle of a plugin
///
/// When adding a plugin to a [`App`]:
/// * the app calls [`Plugin::build`] immediately, and register the plugin
/// * once the app started, it will wait for all registered [`Plugin::ready`] to return `true`
/// * it will then call all registered [`Plugin::finish`]
/// * and call all registered [`Plugin::cleanup`]
pub trait Plugin: Downcast + Any + Send + Sync {
	/// Configures the [`App`] to which this plugin is added.
	/// Can add systems, add schedules, change the runner, etc.
	fn build(&self, app: &mut App);

	/// Has the plugin finished it's setup? This can be useful for plugins that needs something
	/// asynchronous to happen before they can finish their setup, like renderer initialization.
	/// Once the plugin is ready, [`finish`](Plugin::finish) should be called.
	fn ready(&self, _app: &App) -> bool {
		true
	}

	/// Finish adding this plugin to the [`App`], once all plugins registered are ready. This can
	/// be useful for plugins that depends on another plugin asynchronous setup, like the renderer.
	fn finish(&self, _app: &mut App) {
		// do nothing by default
	}

	/// Runs after all plugins are built and finished, but before the app schedule is executed.
	/// This can be useful if you have some resource that other plugins need during their build step,
	/// but after build you want to remove it and send it to another thread.
	fn cleanup(&self, _app: &mut App) {
		// do nothing by default
	}
}

impl_downcast!(Plugin);
