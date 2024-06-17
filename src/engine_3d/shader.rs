use std::hash::Hash;
use std::path::PathBuf;
use std::str::FromStr;
use std::{collections::HashSet, fs::read_to_string, ops::Range};

use regex::Regex;
use wgpu::ShaderSource;
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor};

use crate::src;

use anyhow::Result;
use anyhow::{anyhow, Ok};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

// pub fn normalize_file_path(file: &str) -> String {
// 	// Normalize the file path
// 	let file_path = PathBuf::from_str(file).expect("File not a valid path");
// 	let file: String = file_path.to_string_lossy().into().replace(r#"\\"#, "/");
// 	file
// }

pub type ShaderDir = crate::lib_crates::phf::Map<&'static str, &'static str>;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShaderFile<'a> {
	pub file_name: &'a str,
	pub shader_source: &'a str,
}

impl<'a> ShaderFile<'a> {
	pub fn from_shader_dir(shader_dir: &ShaderDir, file: &str) -> Self {
		let file = &normalize_file_path(file);
		let entry = shader_dir.get_entry(file).expect("File not found");

		Self {
			file_name: *entry.0,
			shader_source: *entry.1,
		}
	}
}

#[derive(Clone, Debug)]
pub struct ShaderBuilder<'a> {
	shader_dir: &'a ShaderDir,
	source: String,
	included_files: HashSet<String>,
}

impl<'a> ShaderBuilder<'a> {
	pub fn new(shader_dir: &'a ShaderDir) -> Self {
		Self {
			shader_dir,
			source: String::default(),
			included_files: HashSet::default(),
		}
	}

	pub fn include(self, file: &str) -> Self {
		self.include_at(0, file)
	}

	pub fn include_at(self, byte_position: usize, file: &str) -> Self {
		let shader_file = ShaderFile::from_shader_dir(self.shader_dir, file);
		self.include_static_at(byte_position, shader_file)
	}

	pub fn include_static(self, shader_file: ShaderFile) -> Self {
		// Insert the source at the beginning of the current shader source
		self.include_static_at(0, shader_file)
	}

	pub fn include_static_at(mut self, byte_position: usize, shader_file: ShaderFile) -> Self {
		// If the shader already included a file previously, don't re-include it
		if self.included_files.contains(shader_file.file_name) {
			return self;
		}

		self.source.insert_str(byte_position, shader_file.shader_source);

		self.included_files.insert(shader_file.file_name.to_owned());
		self
	}

	pub fn build(mut self, device: &Device) -> Result<ShaderModule> {
		println!("{}", self.source);

		self.process_includes()?;

		println!("{}", self.source);

		let shader_module = device.create_shader_module(ShaderModuleDescriptor {
			label: None,
			source: ShaderSource::Wgsl(<std::borrow::Cow<str>>::from(self.source)),
		});

		Ok(shader_module)
	}

	fn process_includes(&mut self) -> Result<()> {
		let re = Regex::new(r#"(?m)^#include "(.+?)"$"#).unwrap();

		let mut byte_offset = 0;
		let mut includes = Vec::<(String, Range<usize>)>::new();

		// Get captures and record their range of the entire match (#include "hello.wgsl") and the capture group (hello.wgsl)
		for caps in re.captures_iter(&self.source) {
			println!("{:?}", caps);
			let range = caps.get(0).unwrap().range();
			let file = caps.get(1).unwrap().as_str().to_owned();
			includes.push((file, range));
		}

		for (file, range) in includes {
			// Offset the range by byte_offset since adding new content to the string shifts down the position of the following include statements
			let range = range.start + byte_offset..range.end + byte_offset;

			// Find the filename in the list of files that was given
			let file = &normalize_file_path(&file);

			println!("{}", file);
			println!("{:?}", self.shader_dir);

			let source = self.shader_dir.get(file).ok_or(anyhow!("File not found"))?;

			// Get the byte-size of the file to be inserted, to shift the other insertions
			let file_size = source.len();

			// Replace the whole range with the included file source
			self.source.replace_range(range, source);

			byte_offset += file_size;
		}

		Ok(())
	}
}
