use std::borrow::Cow;
use std::fmt::format;
use std::hash::Hash;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::{collections::HashSet, fs::read_to_string, ops::Range};

use regex::Regex;
use typed_path::{Utf8Path, Utf8UnixPath, Utf8UnixPathBuf};
use wgpu::ShaderSource;
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor};

use anyhow::Result;
use anyhow::{anyhow, Ok};

use crate::{path, rooted_path};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

// // pub fn normalize_file_path(file: &str) -> String {
// // 	// Normalize the file path
// // 	let file_path = PathBuf::from_str(file).expect("File not a valid path");
// // 	let file: String = file_path.to_string_lossy().into().replace(r#"\\"#, "/");
// // 	file
// // }

pub type ShaderMap = crate::lib_crates::phf::Map<&'static str, &'static str>;

#[derive(Clone, Debug)]
pub struct ShaderBuilder<'a> {
	shader_map: &'a ShaderMap,
	included_files: HashSet<Utf8UnixPathBuf>,
}

impl<'a> ShaderBuilder<'a> {
	pub fn new(shader_dir: &'a ShaderMap) -> Self {
		Self {
			shader_map: shader_dir,
			included_files: HashSet::default(),
		}
	}

	pub fn include<P>(mut self, file: P) -> Self
	where
		P: AsRef<Utf8UnixPath>,
	{
		self.included_files.insert(rooted_path!(file));
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

		for file in self.included_files {
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
		println!("blacklist: {:?}", blacklist);
		println!("file: {:?}, is included: {}", file, blacklist.contains(&file));

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
			let range = (range.start as isize + byte_offset) as usize..(range.end as isize + byte_offset) as usize;

			let path_relative: Utf8UnixPathBuf = path!(&path_str)
				.try_into()
				.or(Err(anyhow!("Invalid file `{}`", path_str)))?;
			let path_absolute = rooted_path!(parent_path.join(path_relative));

			let source_to_include = Self::build_individual_source(path_absolute, blacklist, shader_map)?;

			// Get the byte-size of the file to be inserted, to shift the other insertions
			byte_offset += (source_to_include.len() as isize) - (range.len() as isize);

			// Replace the whole range with the included file source
			source.replace_range(range, &source_to_include);

			println!("\n\n\nnot final:{}", source);
		}

		println!("\n\n\nFINAL:\n{}", source);
		Ok(source)
	}
}
