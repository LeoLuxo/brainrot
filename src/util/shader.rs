use std::borrow::Cow;
use std::fmt::format;
use std::hash::Hash;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::{collections::HashSet, fs::read_to_string, ops::Range};

use hashlink::{LinkedHashMap, LinkedHashSet};
use regex::Regex;
use typed_path::{Utf8Path, Utf8UnixPath, Utf8UnixPathBuf};
use wgpu::ShaderSource;
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor};

use anyhow::Result;
use anyhow::{anyhow, Ok};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

// Macros

#[macro_export]
macro_rules! build_shader_map {
	($path:expr) => {{
		use std::{
			env,
			fs::{read_to_string, File},
			io::{BufWriter, Write},
			path::PathBuf,
		};

		use brainrot::{native_pathbuf, path};

		let dir = path!($path);
		let absolute_dir = path!(env!("CARGO_MANIFEST_DIR")).join(&dir);
		let destination = path!("shader_map.rs");

		// Tell Cargo that if the directory changes, to rerun this build script.
		println!("cargo::rerun-if-changed={}", dir);

		let mut map = $crate::lib_crates::phf_codegen::Map::<String>::new();
		// Set the path that will be printed in the resulting source to the phf re-export (so that it isn't needed in the destination lib)
		map.phf_path("brainrot::lib_crates::phf");

		let shader_files = $crate::lib_crates::glob::glob(absolute_dir.join("**/*").as_str()).unwrap();

		for entry in shader_files {
			let path_buf = if let Ok(path) = entry {
				path
			} else {
				continue;
			};
			if !path_buf.is_file() {
				continue;
			}

			let source = read_to_string(&path_buf).unwrap();

			// Convert path_buf to a typed_path
			let path_buf = path!(&path_buf.to_string_lossy());

			// Make the path relative from the shader dir, and set the root
			let shader_path_relative = path_buf.strip_prefix(&absolute_dir).unwrap().with_unix_encoding();
			let shader_path_rooted = path!("/").join(shader_path_relative);
			let shader_path_str = shader_path_rooted.into_string();

			// The program source needs to be quoted, as the value of the map is printed *as-is*
			map.entry(shader_path_str, &format!("r#\"{}\"#", &source));
		}

		let out_path = path!(&env::var("OUT_DIR").unwrap()).join(destination);
		let out_file = File::create(native_pathbuf!(out_path).unwrap()).unwrap();
		let mut out_writer = BufWriter::new(out_file);

		write!(&mut out_writer, "{}", map.build()).unwrap();
	}};
}

#[macro_export]
macro_rules! include_shader_map {
	() => {{
		include!(concat!(env!("OUT_DIR"), "/shader_map.rs"))
	}};
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub type ShaderMap = crate::lib_crates::phf::Map<&'static str, &'static str>;

#[derive(Clone, Debug)]
pub struct ShaderBuilder<'a> {
	shader_map: &'a ShaderMap,
	include_directives: LinkedHashSet<Utf8UnixPathBuf>,
	// TODO: Add define directive
	// #define POO vec3f(0)
	// define_directives: LinkedHashMap<String, String>,
}

impl<'a> ShaderBuilder<'a> {
	pub fn new(shader_map: &'a ShaderMap) -> Self {
		Self {
			shader_map,
			include_directives: LinkedHashSet::default(),
		}
	}

	pub fn include<P>(mut self, file: P) -> Self
	where
		P: AsRef<Utf8UnixPath>,
	{
		self.include_directives.insert(rooted_path!(file));
		self
	}

	pub fn build(self, device: &Device) -> Result<ShaderModule> {
		let source = self.build_source()?;

		let shader_module = device.create_shader_module(ShaderModuleDescriptor {
			label: None,
			source: ShaderSource::Wgsl(<Cow<str>>::from(source)),
		});

		Ok(shader_module)
	}

	pub fn build_source(self) -> Result<String> {
		let mut blacklist = HashSet::new();
		let shader_map = self.shader_map;

		let mut source = String::new();

		for file in self.include_directives {
			let included_source = Self::build_individual_source(file, &mut blacklist, shader_map)?;
			source.push_str(&included_source);
		}

		Ok(source)
	}

	fn build_individual_source(
		file: Utf8UnixPathBuf,
		blacklist: &mut HashSet<Utf8UnixPathBuf>,
		shader_map: &ShaderMap,
	) -> Result<String> {
		// Check that the file wasn't already included
		if blacklist.contains(&file) {
			// Not an error, just includes empty source
			return Ok("".to_string());
		}

		// The path of the current shader file
		let parent_path = file.parent().map(|x| x.to_owned()).unwrap_or(rooted_path!(""));

		// Get the source from the shader map
		let source_ref = shader_map.get(file.as_str()).ok_or(anyhow!("File not found"))?;
		let mut source = (*source_ref).to_owned();

		// Blacklist the file from including it anymore
		(*blacklist).insert(file);

		let mut byte_offset: isize = 0;
		let mut includes = Vec::<(String, Range<usize>)>::new();

		// Find all `#include "path/to/shader.wgsl"` in the file
		let re = Regex::new(r#"(?m)^#include "(.+?)"$"#).unwrap();

		for caps in re.captures_iter(&source) {
			// The bytes that the `#include "path/to/shader.wgsl"` statement occupies
			let range = caps.get(0).unwrap().range();
			// The `path/to/shader.wgsl` part
			let path_str = caps.get(1).unwrap().as_str().to_owned();
			includes.push((path_str, range));
		}

		// Replace the include statements in the source with the actual source of each file
		for (path_str, range) in includes {
			// Offset the range by byte_offset
			let range = (range.start as isize + byte_offset) as usize..(range.end as isize + byte_offset) as usize;

			// Fix up the path
			let path_relative: Utf8UnixPathBuf = path!(&path_str)
				.try_into()
				.or(Err(anyhow!("Invalid file `{}`", path_str)))?;
			let path_absolute = rooted_path!(parent_path.join(path_relative));

			// Recursively build the source of the included file
			let source_to_include = Self::build_individual_source(path_absolute, blacklist, shader_map)?;

			// Get the byte-size of the file to be inserted, to shift the other insertions afterwards
			byte_offset += (source_to_include.len() as isize) - (range.len() as isize);

			// Replace the whole range with the included file source
			source.replace_range(range, &source_to_include);
		}

		Ok(source)
	}
}
