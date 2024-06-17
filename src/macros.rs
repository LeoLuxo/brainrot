#[macro_export]
macro_rules! src {
	($file:expr) => {
		concat!(env!("CARGO_MANIFEST_DIR"), "/src/", $file)
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

#[cfg(feature = "shader")]
#[macro_export]
macro_rules! build_shader_dir {
	($dir:expr) => {{
		use std::env;
		use std::fs::read_to_string;
		use std::fs::File;
		use std::io::{BufWriter, Write};
		use std::path::Path;

		// Tell Cargo that if the directory changes, to rerun this build script.
		println!(concat!("cargo::rerun-if-changed=", $dir));

		let path = Path::new(&env::var("OUT_DIR").unwrap()).join("shader_dir.rs");
		let mut out_file = BufWriter::new(File::create(&path).unwrap());

		let shader_files = glob::glob(concat!(env!("CARGO_MANIFEST_DIR"), "/", $dir, "**/*")).unwrap();
		let mut map = phf_codegen::Map::<String>::new();

		for entry in shader_files {
			let path_buf = if let Ok(path) = entry { path } else { continue };

			let source = read_to_string(&path_buf).unwrap();
			let path = path_buf.to_str().unwrap().to_owned();

			map.entry(path, "\"" + &source + "\"");
		}

		write!(&mut out_file, "{};", map.build()).unwrap();
	}};
}

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
