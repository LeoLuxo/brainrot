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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShaderFile<'a> {
	pub file: Utf8UnixPathBuf,
	pub source: &'a str,
}

impl<'a> ShaderFile<'a> {
	pub fn new<P>(file: P, source: &'a str) -> Self
	where
		P: AsRef<Utf8UnixPath>,
	{
		let file = rooted_path!(file);

		Self { file, source }
	}

	pub fn from_shader_dir(shader_dir: &ShaderMap, file: &str) -> Self {
		let entry = shader_dir.get_entry(file).expect("File not found");

		Self::new(*entry.0, *entry.1)
	}
}

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

	// fn include_raw(&mut self, byte_offset: &mut usize, shader_file: ShaderFile) {
	// 	// If the shader already included a file previously, don't re-include it
	// 	if self.included_files.contains(&shader_file.file) {
	// 		return;
	// 	}

	// 	self.source.insert_str(*byte_offset, shader_file.source);
	// 	self.included_files.insert(shader_file.file);
	// 	*byte_offset += shader_file.source.len();
	// }

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

		println!("{}", source);
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

		let mut byte_offset = 0;
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
			let range = range.start + byte_offset..range.end + byte_offset;

			let path_relative: Utf8UnixPathBuf = path!(&path_str)
				.try_into()
				.or(Err(anyhow!("Invalid file `{}`", path_str)))?;
			// println!("{}", path_relative);
			let path_absolute = rooted_path!(parent_path.join(path_relative));
			// println!("{}", path_absolute);

			let source_to_include = Self::build_individual_source(path_absolute, blacklist, shader_map)?;

			// Replace the whole range with the included file source
			source.replace_range(range, &source_to_include);

			// Get the byte-size of the file to be inserted, to shift the other insertions
			byte_offset += source_to_include.len();
		}

		Ok(source)
	}

	// fn process_includes(&mut self) -> Result<()> {
	// 	let re = Regex::new(r#"(?m)^#include "(.+?)"$"#).unwrap();

	// 	let mut byte_offset = 0;
	// 	let mut includes = Vec::<(String, Range<usize>)>::new();

	// 	// Get captures and record their range of the entire match (#include "hello.wgsl") and the capture group (hello.wgsl)
	// 	for caps in re.captures_iter(&self.source) {
	// 		println!("{:?}", caps);
	// 		let range = caps.get(0).unwrap().range();
	// 		let file = caps.get(1).unwrap().as_str().to_owned();
	// 		includes.push((file, range));
	// 	}

	// 	for (file, range) in includes {
	// 		// Offset the range by byte_offset since adding new content to the string shifts down the position of the following include statements
	// 		let range = range.start + byte_offset..range.end + byte_offset;

	// 		// Find the filename in the list of files that was given
	// 		// let file = &normalize_file_path(&file);

	// 		println!("{}", file);
	// 		println!("{:?}", self.shader_dir);

	// 		let source = self.shader_dir.get(&file).ok_or(anyhow!("File not found"))?;

	// 		// Get the byte-size of the file to be inserted, to shift the other insertions
	// 		let file_size = source.len();

	// 		// Replace the whole range with the included file source
	// 		self.source.replace_range(range, source);

	// 		byte_offset += file_size;
	// 	}

	// 	Ok(())
	// }
}
