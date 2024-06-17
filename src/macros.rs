#[cfg(feature = "path")]
#[macro_export]
macro_rules! path {
	($path:expr) => {
		typed_path::Utf8TypedPath::derive($path).to_path_buf()
	};
}

#[cfg(feature = "path")]
#[macro_export]
macro_rules! native_pathbuf {
	($path:expr) => {
		TryInto::<PathBuf>::try_into($path)
	};
}

#[cfg(feature = "path")]
#[macro_export]
macro_rules! native_path {
	($path:expr) => {
		TryInto::<Path>::try_into($path)
	};
}

// TODO cleanup into their directories

#[cfg(feature = "path")]
#[macro_export]
macro_rules! src_path {
	() => {
		path!(env!("CARGO_MANIFEST_DIR")).join("src")
	};
}

#[cfg(feature = "path")]
#[macro_export]
macro_rules! rooted_path {
	($path:expr) => {
		Utf8UnixPath::new("/").join($path).normalize()
	};
}

#[macro_export]
macro_rules! path_bytes {
	($path:expr) => {
		typed_path::TypedPath::from($path)
	};
}

#[cfg(feature = "shader")]
#[macro_export]
macro_rules! include_shader {
	($file:expr) => {
		$crate::engine_3d::ShaderFile {
			file_name: $file,
			shader_source: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $file)),
		}
	};
}

// #[cfg(feature = "shader")]
// #[macro_export]
// macro_rules! build_shader_dir {
// 	($dir:expr) => {};
// }

#[cfg(feature = "shader")]
#[macro_export]
macro_rules! include_shader_dir {
	() => {{
		include!(concat!(env!("OUT_DIR"), "/shader_dir.rs"));
	}};
}

#[cfg(feature = "angle")]
#[macro_export]
macro_rules! deg {
	($value:expr) => {{
		$crate::math::Degrees::new($value).into()
	}};
}

#[cfg(feature = "angle")]
#[macro_export]
macro_rules! rad {
	($value:expr) => {{
		$crate::math::Radians::new($value).into()
	}};
}

#[cfg(feature = "speed")]
#[macro_export]
macro_rules! spd {
	($value:expr) => {{
		$crate::math::Speed::new_per_second($value).into()
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
