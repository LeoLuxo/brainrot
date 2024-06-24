#![allow(dead_code)]

use std::mem;

use bevy_ecs::{
	component::Component,
	schedule::{IntoSystemConfigs, IntoSystemSetConfigs, Schedule, ScheduleBuildSettings, ScheduleLabel, Schedules},
	system::Resource,
	world::{FromWorld, World},
};

use super::{plugin::Plugin, *};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

/// A container of app logic and data.
/// This is a personal partial re-implementation of [`bevy_app::app::App`] from
/// `bevy_app`
///
/// Bundles together the necessary elements like [`World`] and [`Schedule`] to
/// create a bevy ECS-based application. It also stores a pointer to a [runner
/// function](Self::set_runner). The runner is responsible for managing the
/// application's event loop and applying the [`Schedule`] to the [`World`] to
/// drive application logic.
pub struct App {
	/// The main ECS [`World`] of the [`App`].
	/// This stores and provides access to all the main data of the app.
	/// The systems of the [`App`] will run using this [`World`].
	pub world: World,

	/// The [runner function](Self::set_runner) is primarily responsible for
	/// managing the app's event loop and advancing the [`Schedule`].
	// Send bound is required to make App Send
	runner: Option<Box<dyn FnOnce(App) + Send>>,

	/// The vector where plugins are stored for running functions on them
	plugin_registry: Vec<Box<dyn Plugin>>,

	/// A private counter to prevent incorrect calls to [`App::run()`] from
	/// [`Plugin::build()`]
	building_plugin_depth: usize,

	/// The state of the Plugin initialization process
	plugins_state: PluginsState,
}

/// Plugins state in the app
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PluginsState {
	/// Plugins are being added.
	Adding,
	/// All plugins already added are ready.
	Ready,
	/// Finish has been executed for all plugins added.
	Finished,
	/// Cleanup has been executed for all plugins added.
	Cleaned,
}

/// Dummy plugin used to temporary hold the place in the plugin registry
struct PlaceholderPlugin;
impl Plugin for PlaceholderPlugin {
	fn build(&self, _app: &mut App) {}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

impl Default for App {
	fn default() -> Self {
		let mut world = World::new();
		world.init_resource::<Schedules>();
		Self {
			world,
			runner: None,
			plugin_registry: Vec::new(),
			building_plugin_depth: 0,
			plugins_state: PluginsState::Adding,
		}
	}
}

impl App {
	/// Construct a new default [`App`] instance.
	/// Has no runner by default.
	pub fn new() -> Self {
		Self::default()
	}

	/// Starts the app by calling the app's [runner function](Self::set_runner).
	///
	/// # `run()` might not return
	///
	/// Calls to [`App::run()`] might never return.
	/// This is completely dependant on what the [runner
	/// function](Self::set_runner) does. If it works like a windowed app using
	/// an *event loop* and then terminates the app when a certain event is
	/// fired, [`App::run()`] will probably not return properly.
	///
	/// # Panics
	///
	/// Panics if called from `Plugin::build()`, because it would prevent other
	/// plugins to properly build.
	pub fn run(&mut self) {
		// The way I understand this, we're taking ownership of self so that we can pass
		// it to the runner as fully owned
		let mut app = mem::take(self);

		assert!(
			app.building_plugin_depth == 0,
			"App::run() was called from within Plugin::build(), which is not allowed."
		);

		// Also make sure that the runner cannot call the runner again (well, "itself" I
		// guess)
		let runner = app
			.runner
			.take()
			.expect("No runner was set before App::run() was called.");

		// Call the runner, which will now own the instance of App
		(runner)(app);
	}

	/// Check the state of all plugins already added to this app.
	#[inline]
	pub fn plugins_state(&self) -> PluginsState {
		match self.plugins_state {
			PluginsState::Adding => {
				for plugin in &self.plugin_registry {
					if !plugin.ready(self) {
						return PluginsState::Adding;
					}
				}
				PluginsState::Ready
			}
			state => state,
		}
	}

	/// Run [`Plugin::finish`] for each plugin. This should usually be called by
	/// the event loop / runner once all plugins are ready.
	pub fn finish(&mut self) {
		// temporarily remove the plugin registry to run each plugin's setup function on
		// app.
		let plugin_registry = mem::take(&mut self.plugin_registry);
		for plugin in &plugin_registry {
			plugin.finish(self);
		}
		self.plugin_registry = plugin_registry;
		self.plugins_state = PluginsState::Finished;
	}

	/// Run [`Plugin::cleanup`] for each plugin. This should usually be called
	/// by the event loop / runner after [`App::finish`].
	pub fn cleanup(&mut self) {
		// temporarily remove the plugin registry to run each plugin's setup function on
		// app.
		let plugin_registry = mem::take(&mut self.plugin_registry);
		for plugin in &plugin_registry {
			plugin.cleanup(self);
		}
		self.plugin_registry = plugin_registry;
		self.plugins_state = PluginsState::Cleaned;
	}

	/// Adds systems to the given schedule in this app's [`Schedules`].
	pub fn add_systems<M>(&mut self, schedule: impl ScheduleLabel, systems: impl IntoSystemConfigs<M>) -> &mut Self {
		let schedule = schedule.intern();
		let mut schedules = self.world.resource_mut::<Schedules>();

		if let Some(schedule) = schedules.get_mut(schedule) {
			schedule.add_systems(systems);
		} else {
			let mut new_schedule = Schedule::new(schedule);
			new_schedule.add_systems(systems);
			schedules.insert(new_schedule);
		}

		self
	}

	/// Configures a collection of system sets in the default schedule, adding
	/// any sets that do not exist.
	#[track_caller]
	pub fn configure_sets(&mut self, schedule: impl ScheduleLabel, sets: impl IntoSystemSetConfigs) -> &mut Self {
		let schedule = schedule.intern();
		let mut schedules = self.world.resource_mut::<Schedules>();

		if let Some(schedule) = schedules.get_mut(schedule) {
			schedule.configure_sets(sets);
		} else {
			let mut new_schedule = Schedule::new(schedule);
			new_schedule.configure_sets(sets);
			schedules.insert(new_schedule);
		}

		self
	}

	/// Inserts a [`Resource`] to the current [`App`] and overwrites any
	/// [`Resource`] previously added of the same type.
	///
	/// A [`Resource`] in Bevy represents globally unique data. [`Resource`]s
	/// must be added to the Bevy world before using them. This happens with
	/// [`insert_resource`](Self::insert_resource).
	///
	/// See [`init_resource`](Self::init_resource) for [`Resource`]s that
	/// implement [`Default`] or [`FromWorld`].
	pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
		self.world.insert_resource(resource);
		self
	}

	/// Initialize a [`Resource`] with standard starting values by adding it to
	/// the [`World`].
	///
	/// If the [`Resource`] already exists, nothing happens.
	///
	/// The [`Resource`] must implement the [`FromWorld`] trait.
	/// If the [`Default`] trait is implemented, the [`FromWorld`] trait will
	/// use the [`Default::default`] method to initialize the [`Resource`].
	pub fn init_resource<R: Resource + FromWorld>(&mut self) -> &mut Self {
		self.world.init_resource::<R>();
		self
	}

	/// Sets the function that will be called when the app is run.
	///
	/// The runner function `run_fn` is called only once by [`App::run`]. If the
	/// presence of a main loop in the app is desired, it is the responsibility
	/// of the runner function to provide it.
	pub fn set_runner(&mut self, run_fn: impl FnOnce(App) + 'static + Send) -> &mut Self {
		self.runner = Some(Box::new(run_fn));
		self
	}

	/// Checks if a [`Plugin`] has already been added.
	///
	/// This can be used by plugins to check if a plugin they depend upon has
	/// already been added.
	pub fn is_plugin_added<T>(&self) -> bool
	where
		T: Plugin,
	{
		self.plugin_registry.iter().any(|p| p.downcast_ref::<T>().is_some())
	}

	/// Returns a vector of references to any plugins of type `T` that have been
	/// added.
	///
	/// This can be used to read the settings of any already added plugins.
	/// This vector will be length zero if no plugins of that type have been
	/// added. If multiple copies of the same plugin are added to the [`App`],
	/// they will be listed in insertion order in this vector.
	pub fn get_added_plugins<T>(&self) -> Vec<&T>
	where
		T: Plugin,
	{
		self.plugin_registry.iter().filter_map(|p| p.downcast_ref()).collect()
	}

	/// Adds one or more [`Plugin`]s.
	///
	/// # Panics
	///
	/// Panics if the app has already started running.
	#[track_caller]
	pub fn add_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
		if matches!(self.plugins_state(), PluginsState::Cleaned | PluginsState::Finished) {
			panic!("Plugins cannot be added after App::cleanup() or App::finish() has been called.");
		}

		// Reserve that position in the plugin registry. if a plugin adds plugins, they
		// will be correctly ordered
		let plugin_position_in_registry = self.plugin_registry.len();
		self.plugin_registry.push(Box::new(PlaceholderPlugin));

		self.building_plugin_depth += 1;

		// Immediately call build on the plugin to start its initialization
		plugin.build(self);

		self.building_plugin_depth -= 1;
		self.plugin_registry[plugin_position_in_registry] = Box::new(plugin);

		self
	}

	/// Adds a new `schedule` to the [`App`] under the provided `label`.
	///
	/// # Warning
	/// This method will overwrite any existing schedule at that label.
	/// To avoid this behavior, use the `init_schedule` method instead.
	pub fn add_schedule(&mut self, schedule: Schedule) -> &mut Self {
		let mut schedules = self.world.resource_mut::<Schedules>();
		schedules.insert(schedule);

		self
	}

	/// Initializes a new empty `schedule` to the [`App`] under the provided
	/// `label` if it does not exists.
	///
	/// See [`App::add_schedule`] to pass in a pre-constructed schedule.
	pub fn init_schedule(&mut self, label: impl ScheduleLabel) -> &mut Self {
		let label = label.intern();
		let mut schedules = self.world.resource_mut::<Schedules>();
		if !schedules.contains(label) {
			schedules.insert(Schedule::new(label));
		}
		self
	}

	/// Gets read-only access to the [`Schedule`] with the provided `label` if
	/// it exists.
	pub fn get_schedule(&self, label: impl ScheduleLabel) -> Option<&Schedule> {
		let schedules = self.world.get_resource::<Schedules>()?;
		schedules.get(label)
	}

	/// Gets read-write access to a [`Schedule`] with the provided `label` if it
	/// exists.
	pub fn get_schedule_mut(&mut self, label: impl ScheduleLabel) -> Option<&mut Schedule> {
		let schedules = self.world.get_resource_mut::<Schedules>()?;
		// We need to call .into_inner here to satisfy the borrow checker:
		// it can reason about reborrows using ordinary references but not the `Mut`
		// smart pointer.
		schedules.into_inner().get_mut(label)
	}

	/// Applies the function to the [`Schedule`] associated with `label`.
	///
	/// **Note:** This will create the schedule if it does not already exist.
	pub fn edit_schedule(&mut self, label: impl ScheduleLabel, f: impl FnOnce(&mut Schedule)) -> &mut Self {
		let label = label.intern();
		let mut schedules = self.world.resource_mut::<Schedules>();

		if schedules.get(label).is_none() {
			schedules.insert(Schedule::new(label));
		}

		let schedule = schedules.get_mut(label).unwrap();
		// Call the function f, passing in the schedule retrieved
		f(schedule);

		self
	}

	/// Applies the provided [`ScheduleBuildSettings`] to all schedules.
	pub fn configure_schedules(&mut self, schedule_build_settings: ScheduleBuildSettings) -> &mut Self {
		self.world
			.resource_mut::<Schedules>()
			.configure_schedules(schedule_build_settings);
		self
	}

	/// When doing [ambiguity
	/// checking](bevy_ecs::schedule::ScheduleBuildSettings) this
	/// ignores systems that are ambiguous on [`Component`] T.
	///
	/// This settings only applies to the main world. To apply this to other
	/// worlds call the [corresponding
	/// method](World::allow_ambiguous_component) on World
	pub fn allow_ambiguous_component<T: Component>(&mut self) -> &mut Self {
		self.world.allow_ambiguous_component::<T>();
		self
	}

	/// When doing [ambiguity
	/// checking](bevy_ecs::schedule::ScheduleBuildSettings) this
	/// ignores systems that are ambiguous on [`Resource`] T.
	///
	/// This settings only applies to the main world. To apply this to other
	/// worlds call the [corresponding method](World::allow_ambiguous_resource)
	/// on World
	pub fn allow_ambiguous_resource<T: Resource>(&mut self) -> &mut Self {
		self.world.allow_ambiguous_resource::<T>();
		self
	}
}
