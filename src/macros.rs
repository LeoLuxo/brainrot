#[macro_export]
macro_rules! src {
	($value:expr) => {
		concat!(env!("CARGO_MANIFEST_DIR"), "/src/", $value)
	};
}

#[cfg(feature = "angle")]
#[macro_export]
macro_rules! deg {
	($value:expr) => {{
		$crate::math::angle::Degrees::new($value).into()
	}};
}

#[cfg(feature = "angle")]
#[macro_export]
macro_rules! rad {
	($value:expr) => {{
		$crate::math::angle::Radians::new($value).into()
	}};
}

#[cfg(feature = "speed")]
#[macro_export]
macro_rules! spd {
	($value:expr) => {{
		$crate::math::speed::Speed::new_per_second($value).into()
	}};
}

#[cfg(feature = "vek")]
#[macro_export]
macro_rules! v {
	($x:expr,$y:expr) => {{
		brainrot::vek::Vec2::new($x, $y)
	}};
	($x:expr,$y:expr,$z:expr) => {{
		brainrot::vek::Vec3::new($x, $y, $z)
	}};
	($x:expr,$y:expr,$z:expr,$w:expr) => {{
		brainrot::vek::Vec4::new($x, $y, $z, $w)
	}};
}

#[cfg(feature = "vek")]
#[macro_export]
macro_rules! vec2 {
	($x:expr) => {{
		brainrot::vek::Vec2::new($x, $x)
	}};
	($x:expr,$y:expr) => {{
		brainrot::vek::Vec2::new($x, $y)
	}};
}

#[cfg(feature = "vek")]
#[macro_export]
macro_rules! vec3 {
	($x:expr) => {{
		brainrot::vek::Vec3::new($x, $x, $x)
	}};
	($x:expr,$y:expr,$z:expr) => {{
		brainrot::vek::Vec3::new($x, $y, $z)
	}};
}

#[cfg(feature = "vek")]
#[macro_export]
macro_rules! vec4 {
	($x:expr) => {{
		brainrot::vek::Vec3::new($x, $x, $x, $x)
	}};
	($x:expr,$y:expr,$z:expr,$w:expr) => {{
		brainrot::vek::Vec4::new($x, $y, $z, $w)
	}};
}

#[cfg(feature = "vek")]
#[macro_export]
macro_rules! size {
	($x:expr,$y:expr) => {{
		$crate::ScreenSize::new($x, $y)
	}};
}
