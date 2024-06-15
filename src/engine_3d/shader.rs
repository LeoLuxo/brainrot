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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ShaderFile<'a> {
	pub file_name: &'a str,
	pub shader_source: &'a str,
}

#[derive(Clone, Debug, Default)]
pub struct ShaderBuilder {
	source: String,
	files: HashSet<String>,
}

impl ShaderBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn from_source(shader_file: ShaderFile) -> Self {
		Self::new().include(shader_file)
		// TODO Change this to take dynamic &str instead of static ShaderFile and make the constructor of shaderbuilder take SHADER_FILES
	}

	pub fn include(self, shader_file: ShaderFile) -> Self {
		// Insert the source at the beginning of the current shader source
		self.include_at(shader_file, 0)
	}

	pub fn include_at(mut self, shader_file: ShaderFile, byte_position: usize) -> Self {
		// If the shader already included a file previously, don't re-include it
		if self.files.contains(shader_file.file_name) {
			return self;
		}

		self.source.insert_str(byte_position, shader_file.shader_source);

		self.files.insert(shader_file.file_name.to_owned());
		self
	}

	pub fn build(mut self, device: &Device, shader_files: &[ShaderFile]) -> Result<ShaderModule> {
		self.process_includes(shader_files)?;

		let shader_module = device.create_shader_module(ShaderModuleDescriptor {
			label: None,
			source: ShaderSource::Wgsl(self.source.into()),
		});

		Ok(shader_module)
	}

	fn process_includes(&mut self, shader_files: &[ShaderFile]) -> Result<()> {
		let re = Regex::new(r#"^#include "(.*?)"$"#).unwrap();

		let mut byte_offset = 0;
		let mut includes = Vec::<(String, Range<usize>)>::new();

		// Get captures and record their range of the entire match (#include "hello.wgsl") and the capture group (hello.wgsl)
		for caps in re.captures_iter(&self.source) {
			let range = caps.get(0).unwrap().range();
			let file = caps.get(1).unwrap().as_str().to_owned();
			includes.push((file, range));
		}

		for (file, range) in includes {
			// Offset the range by byte_offset since adding new content to the string shifts down the position of the following include statements
			let range = range.start + byte_offset..range.end + byte_offset;

			// Find the filename in the list of files that was given
			let source = shader_files
				.iter()
				.find(|ShaderFile { file_name, .. }| *file_name == file)
				.ok_or(anyhow!("File not found"))?
				.shader_source;

			// Get the byte-size of the file to be inserted, to shift the other insertions
			let file_size = source.len();

			// Replace the whole range with the included file source
			self.source.replace_range(range, &source);

			byte_offset += file_size;
		}

		Ok(())
	}
}
