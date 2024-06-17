// Re-export
pub use vek;

/// A screen size in pixels
pub type ScreenSize = vek::Extent2<u32>;

/// A delta of the mouse cursor movement over the last frame
pub type MouseMotionDelta = vek::Vec2<f64>;

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

// Macros

#[macro_export]
macro_rules! v {
	($x:expr,$y:expr) => {{
		$crate::vek::Vec2::new($x, $y)
	}};
	($x:expr,$y:expr,$z:expr) => {{
		$crate::vek::Vec3::new($x, $y, $z)
	}};
	($x:expr,$y:expr,$z:expr,$w:expr) => {{
		$crate::vek::Vec4::new($x, $y, $z, $w)
	}};
}

#[macro_export]
macro_rules! vec2 {
	($x:expr) => {{
		$crate::vek::Vec2::new($x, $x)
	}};
	($x:expr,$y:expr) => {{
		$crate::vek::Vec2::new($x, $y)
	}};
}

#[macro_export]
macro_rules! vec3 {
	($x:expr) => {{
		$crate::vek::Vec3::new($x, $x, $x)
	}};
	($x:expr,$y:expr,$z:expr) => {{
		$crate::vek::Vec3::new($x, $y, $z)
	}};
}

#[macro_export]
macro_rules! vec4 {
	($x:expr) => {{
		$crate::vek::Vec3::new($x, $x, $x, $x)
	}};
	($x:expr,$y:expr,$z:expr,$w:expr) => {{
		$crate::vek::Vec4::new($x, $y, $z, $w)
	}};
}

#[macro_export]
macro_rules! size {
	($x:expr,$y:expr) => {{
		$crate::ScreenSize::new($x, $y)
	}};
}
