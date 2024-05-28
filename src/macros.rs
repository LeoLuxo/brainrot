#[cfg(feature = "angle")]
#[macro_export]
macro_rules! deg {
	($value:expr) => {{
		$crate::angle::Degrees::new($value).into()
	}};
}

#[cfg(feature = "angle")]
#[macro_export]
macro_rules! rad {
	($value:expr) => {{
		$crate::angle::Radians::new($value).into()
	}};
}

#[cfg(feature = "speed")]
#[macro_export]
macro_rules! spd {
	($value:expr) => {{
		$crate::math::speed::Speed::new_per_second($value).into()
	}};
}

#[macro_export]
macro_rules! v {
	($x:expr,$y:expr) => {{
		vek::Vec2::new($x, $y)
	}};
	($x:expr,$y:expr,$z:expr) => {{
		vek::Vec3::new($x, $y, $z)
	}};
	($x:expr,$y:expr,$z:expr,$w:expr) => {{
		vek::Vec4::new($x, $y, $z, $w)
	}};
}

#[macro_export]
macro_rules! vec2 {
	($x:expr) => {{
		vek::Vec2::new($x, $x)
	}};
	($x:expr,$y:expr) => {{
		vek::Vec2::new($x, $y)
	}};
}

#[macro_export]
macro_rules! vec3 {
	($x:expr) => {{
		vek::Vec3::new($x, $x, $x)
	}};
	($x:expr,$y:expr,$z:expr) => {{
		vek::Vec3::new($x, $y, $z)
	}};
}

#[macro_export]
macro_rules! vec4 {
	($x:expr) => {{
		vek::Vec3::new($x, $x, $x, $x)
	}};
	($x:expr,$y:expr,$z:expr,$w:expr) => {{
		vek::Vec4::new($x, $y, $z, $w)
	}};
}

#[macro_export]
macro_rules! size {
	($x:expr,$y:expr) => {{
		$crate::ScreenSize::new($x, $y)
	}};
}
