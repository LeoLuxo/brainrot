#![allow(dead_code)]

use std::f32::consts;

use derive_more::{Deref, Display, From, Into};
use vek::{Extent2, Mat4, Vec3};

use crate::math::Angle;

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[derive(Deref, From, Into, Display, Copy, Clone, Debug, Default, PartialEq)]
pub struct ScreenSize(pub Extent2<u32>);

#[derive(Deref, From, Into, Display, Copy, Clone, Debug, Default, PartialEq)]
pub struct Position(pub Vec3<f32>);

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Direction {
	pub yaw: Angle,
	pub pitch: Angle,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Frustum {
	pub y_fov: f32,
	pub z_near: f32,
	pub z_far: f32,
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub const SAFE_FRAC_PI_2: f32 = consts::FRAC_PI_2 - 0.0001;

pub fn calc_forward_vector(Direction { yaw, pitch }: Direction) -> Vec3<f32> {
	let (sin_yaw, cos_yaw) = yaw.sin_cos();
	let (sin_pitch, cos_pitch) = pitch.sin_cos();
	vek::Vec3::new(sin_yaw * cos_pitch, sin_pitch, cos_yaw * cos_pitch)
}

pub fn calc_forward_horizontal_vector(Direction { yaw, .. }: Direction) -> Vec3<f32> {
	let (sin_yaw, cos_yaw) = yaw.sin_cos();
	vek::Vec3::new(sin_yaw, 0., cos_yaw)
}

pub fn calc_right_vector(Direction { yaw, .. }: Direction) -> Vec3<f32> {
	let (sin_yaw, cos_yaw) = yaw.sin_cos();
	vek::Vec3::new(cos_yaw, 0., -sin_yaw)
}

#[allow(dead_code)]
pub fn calc_up_vector(Direction { yaw, pitch }: Direction) -> Vec3<f32> {
	let (sin_yaw, cos_yaw) = yaw.sin_cos();
	let (sin_pitch, cos_pitch) = pitch.sin_cos();
	vek::Vec3::new(sin_yaw * -sin_pitch, cos_pitch, cos_yaw * -sin_pitch)
}

pub fn calc_view_matrix(Position(position): Position, direction: Direction) -> Mat4<f32> {
	Mat4::look_at_lh(position, position + calc_forward_vector(direction), Vec3::unit_y())
}

pub fn calc_projection_matrix(
	Frustum { y_fov, z_near, z_far }: Frustum,
	ScreenSize(Extent2 { w, h }): ScreenSize,
) -> Mat4<f32> {
	Mat4::perspective_fov_lh_zo(y_fov, w as f32, h as f32, z_near, z_far)
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

#[cfg(feature = "bevy")]
mod bevy {
	use bevy_ecs::component::{Component, TableStorage};

	use super::{Direction, Frustum, Position, ScreenSize};

	#[rustfmt::skip] impl Component for ScreenSize {type Storage = TableStorage;}
	#[rustfmt::skip] impl Component for Position   {type Storage = TableStorage;}
	#[rustfmt::skip] impl Component for Direction  {type Storage = TableStorage;}
	#[rustfmt::skip] impl Component for Frustum    {type Storage = TableStorage;}
}

#[cfg(feature = "bevy")]
pub use bevy::*;

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/
#[cfg(test)]
mod tests {

	use super::*;
	use crate::math::{Angle, Degrees};
	use approx::assert_relative_eq;
	use rstest::rstest;
	use std::f32::consts::FRAC_1_SQRT_2;
	use vek::{num_traits::Float, Vec3};

	#[rstest]
	#[case(Degrees::new(0.).into(),   Vec3::new(0., 0., 1.))]
	#[case(Degrees::new(90.).into(),  Vec3::new(1., 0., 0.))]
	#[case(Degrees::new(180.).into(), Vec3::new(0., 0., -1.))]
	#[case(Degrees::new(-90.).into(), Vec3::new(-1., 0., 0.))]
	fn camera_forward_horizonal(#[case] yaw: Angle, #[case] expected: Vec3<f32>) {
		assert_relative_eq!(
			calc_forward_horizontal_vector(Direction {
				yaw,
				pitch: Degrees::new(0.).into()
			}),
			expected
		);
	}

	#[rstest]
	#[case(Degrees::new(0.).into(),   Degrees::new(0.).into(),   Vec3::new(0., 0., 1.))]
	#[case(Degrees::new(90.).into(),  Degrees::new(0.).into(),   Vec3::new(1., 0., 0.))]
	#[case(Degrees::new(180.).into(), Degrees::new(0.).into(),   Vec3::new(0., 0., -1.))]
	#[case(Degrees::new(270.).into(), Degrees::new(0.).into(),   Vec3::new(-1., 0., 0.))]
	#[case(Degrees::new(0.).into(),   Degrees::new(90.).into(),  Vec3::new(0., 1., 0.))]
	#[case(Degrees::new(0.).into(),   Degrees::new(-90.).into(), Vec3::new(0., -1., 0.))]
	#[case(Degrees::new(180.).into(), Degrees::new(90.).into(),  Vec3::new(0., 1., 0.))]
	#[case(Degrees::new(180.).into(), Degrees::new(-90.).into(), Vec3::new(0., -1., 0.))]
	#[case(Degrees::new(45.).into(),  Degrees::new(45.).into(),  Vec3::new(0.5, FRAC_1_SQRT_2, 0.5))]
	fn camera_forward(#[case] yaw: Angle, #[case] pitch: Angle, #[case] expected: Vec3<f32>) {
		assert_relative_eq!(calc_forward_vector(Direction { yaw, pitch }), expected);
	}

	#[rstest]
	#[case(Degrees::new(0.).into(),   Vec3::new(1., 0., 0.))]
	#[case(Degrees::new(90.).into(),  Vec3::new(0., 0., -1.))]
	#[case(Degrees::new(180.).into(), Vec3::new(-1., 0., 0.))]
	#[case(Degrees::new(270.).into(), Vec3::new(0., 0., 1.))]
	fn camera_right(#[case] yaw: Angle, #[case] expected: Vec3<f32>) {
		assert_relative_eq!(
			calc_right_vector(Direction {
				yaw,
				pitch: Degrees::new(0.).into()
			}),
			expected
		);
	}

	#[rstest]
	#[case(Degrees::new(0.).into(),   Degrees::new(0.).into(),   Vec3::new(0., 1., 0.))]
	#[case(Degrees::new(90.).into(),  Degrees::new(0.).into(),   Vec3::new(0., 1., 0.))]
	#[case(Degrees::new(180.).into(), Degrees::new(0.).into(),   Vec3::new(0., 1., 0.))]
	#[case(Degrees::new(-90.).into(), Degrees::new(0.).into(),   Vec3::new(0., 1., 0.))]
	#[case(Degrees::new(0.).into(),   Degrees::new(90.).into(),  Vec3::new(0., 0., -1.))]
	#[case(Degrees::new(0.).into(),   Degrees::new(-90.).into(), Vec3::new(0., 0., 1.))]
	#[case(Degrees::new(180.).into(), Degrees::new(90.).into(),  Vec3::new(0., 0., 1.))]
	#[case(Degrees::new(180.).into(), Degrees::new(-90.).into(), Vec3::new(0., 0., -1.))]
	#[case(Degrees::new(45.).into(),  Degrees::new(45.).into(),  Vec3::new(-0.5, FRAC_1_SQRT_2, -0.5))]
	fn camera_up(#[case] yaw: Angle, #[case] pitch: Angle, #[case] expected: Vec3<f32>) {
		assert_relative_eq!(calc_up_vector(Direction { yaw, pitch }), expected);
	}
}
