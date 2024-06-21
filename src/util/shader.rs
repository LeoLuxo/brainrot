use std::{
	borrow::Cow,
	collections::{HashMap, HashSet},
	fmt::format,
	fs::read_to_string,
	hash::{DefaultHasher, Hash, Hasher},
	mem,
	ops::{Deref, Range},
	path::PathBuf,
	str::FromStr,
};

use anyhow::{anyhow, Ok, Result};
use hashlink::{LinkedHashMap, LinkedHashSet};
use rand::{distributions, seq::IteratorRandom, Rng};
use regex::Regex;
use replace_with::{replace_with, replace_with_or_abort};
use typed_path::{
	TypedPath, TypedPathBuf, UnixPath, UnixPathBuf, Utf8Path, Utf8TypedPath, Utf8TypedPathBuf, Utf8UnixPath,
	Utf8UnixPathBuf, Utf8WindowsPath, Utf8WindowsPathBuf, WindowsPath, WindowsPathBuf,
};
use velcro::{iter, vec};
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

// Macros

#[macro_export]
macro_rules! build_shader_source_map {
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
		let destination = path!("shader_source_map.rs");

		// Tell Cargo that if the directory changes, to rerun this build script.
		println!("cargo::rerun-if-changed={}", dir);

		let mut map = $crate::lib_crates::phf_codegen::Map::<String>::new();
		// Set the path that will be printed in the resulting source to the phf
		// re-export (so that it isn't needed in the destination lib)
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

			// The program source needs to be quoted, as the value of the map is printed
			// *as-is*
			map.entry(shader_path_str, &format!("r#\"{}\"#", &source));
		}

		let out_path = path!(&env::var("OUT_DIR").unwrap()).join(destination);
		let out_file = File::create(native_pathbuf!(out_path).unwrap()).unwrap();
		let mut out_writer = BufWriter::new(out_file);

		write!(&mut out_writer, "{}", map.build()).unwrap();
	}};
}

#[macro_export]
macro_rules! include_shader_source_map {
	() => {{
		include!(concat!(env!("OUT_DIR"), "/shader_source_map.rs"))
	}};
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub type ShaderSourceMap = crate::lib_crates::phf::Map<&'static str, &'static str>;

trait ShaderPath {}
impl ShaderPath for TypedPath<'_> {}
impl ShaderPath for TypedPathBuf {}
impl ShaderPath for Utf8TypedPath<'_> {}
impl ShaderPath for Utf8TypedPathBuf {}
impl ShaderPath for UnixPath {}
impl ShaderPath for UnixPathBuf {}
impl ShaderPath for Utf8UnixPath {}
impl ShaderPath for Utf8UnixPathBuf {}
impl ShaderPath for WindowsPath {}
impl ShaderPath for WindowsPathBuf {}
impl ShaderPath for Utf8WindowsPath {}
impl ShaderPath for Utf8WindowsPathBuf {}

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub enum Shader {
	Source(String),
	Path(Utf8UnixPathBuf),
	Builder(ShaderBuilder),
}

impl Shader {
	pub fn get_parent(&self) -> Utf8UnixPathBuf {
		match self {
			Shader::Source(_) => root!(),
			Shader::Path(path) => path.parent().map(|x| x.to_owned()).unwrap_or(root!()),
			Shader::Builder(_) => root!(),
		}
	}

	pub fn build(self, shader_source_map: &ShaderSourceMap) -> Result<String> {
		match self {
			Shader::Source(source) => Ok(source),
			Shader::Path(path) => Self::get_path_source(path, shader_source_map),
			Shader::Builder(mut builder) => builder.build_source(shader_source_map),
		}
	}

	pub fn obfuscate_fn(&mut self, func_name: &str) -> String {
		// Generate the obfuscated function name
		let obfuscated = iter![..('a'..='z'), ..('A'..='Z')]
			.choose_multiple(&mut rand::thread_rng(), 16)
			.into_iter()
			.collect::<String>();

		let from = format!("{}(", func_name);
		let to = format!("{}(", obfuscated);

		replace_with_or_abort(self, |self_| match self_ {
			Shader::Source(source) => source.replace(&from, &to).into(),
			Shader::Path(path) => ShaderBuilder::new().include(path).define(from, to).into(),
			Shader::Builder(mut builder) => builder.define(from, to).into(),
		});

		obfuscated
	}

	fn process_source(self, blacklist: &mut HashSet<Shader>, shader_source_map: &ShaderSourceMap) -> Result<String> {
		// Check that the file wasn't already included
		if blacklist.contains(&self) {
			// Not an error, just includes empty source
			return Ok("".to_string());
		}

		// Blacklist the shader from including it anymore
		(*blacklist).insert(self.clone());

		// The path of the current shader file
		let parent_path = self.get_parent();

		// Get the source from the shader
		let mut source = self.build(shader_source_map)?;

		let mut byte_offset: isize = 0;
		let mut includes = Vec::<(String, Range<usize>)>::new();

		// Find all `#include "path/to/shader.wgsl"` in the source
		let re = Regex::new(r#"(?m)^#include "(.+?)"$"#).unwrap();

		for caps in re.captures_iter(&source) {
			// The bytes that the `#include "path/to/shader.wgsl"` statement occupies
			let range = caps.get(0).unwrap().range();
			// The `path/to/shader.wgsl` part
			let path_str = caps.get(1).unwrap().as_str().to_owned();
			includes.push((path_str, range));
		}

		// Replace the include statements in the source with the actual source of each
		// file
		for (path_str, range) in includes {
			// Offset the range by byte_offset
			let range = (range.start as isize + byte_offset) as usize..(range.end as isize + byte_offset) as usize;

			// Fix up the path
			let path_relative: Utf8UnixPathBuf = path!(&path_str)
				.try_into()
				.or(Err(anyhow!("Invalid file `{}`", path_str)))?;
			let path_absolute = rooted_path!(parent_path.join(path_relative));

			// Recursively build the source of the included file
			let source_to_include = Self::process_source(path_absolute.into(), blacklist, shader_source_map)?;

			// Get the byte-size of the file to be inserted, to shift the other insertions
			// afterwards
			byte_offset += (source_to_include.len() as isize) - (range.len() as isize);

			// Replace the whole range with the included file source
			source.replace_range(range, &source_to_include);
		}

		Ok(source)
	}

	fn get_path_source(path: Utf8UnixPathBuf, shader_source_map: &ShaderSourceMap) -> Result<String> {
		let path = rooted_path!(path);

		// Get the source from the shader map
		let source_ref = shader_source_map
			.get(path.as_str())
			.ok_or(anyhow!("File not found: {}", path.as_str()))?;
		let source = (*source_ref).to_owned();

		Ok(source)
	}
}

impl From<String> for Shader {
	fn from(value: String) -> Self {
		Self::Source(value)
	}
}

impl From<&str> for Shader {
	fn from(value: &str) -> Self {
		Self::Source(value.to_owned())
	}
}

impl<P> From<P> for Shader
where
	P: TryInto<Utf8UnixPathBuf> + ShaderPath,
{
	fn from(value: P) -> Self {
		// The case where a path is valid as Windows path but not as Unix is so rare
		// that it's okay to unwrap here instead of delegating the error to
		// ShaderBuilder.build
		Self::Path(value.try_into().or(Err(anyhow!("Invalid shader path"))).unwrap())
	}
}

impl From<ShaderBuilder> for Shader {
	fn from(value: ShaderBuilder) -> Self {
		Self::Builder(value)
	}
}

impl From<&mut ShaderBuilder> for Shader {
	fn from(value: &mut ShaderBuilder) -> Self {
		Self::Builder(mem::take(value))
	}
}

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct ShaderBuilder {
	include_directives: LinkedHashSet<Shader>,
	define_directives: LinkedHashMap<String, String>,
}

impl ShaderBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn include<S>(&mut self, shader: S) -> &mut Self
	where
		S: Into<Shader>,
	{
		self.include_directives.insert(shader.into());
		self
	}

	pub fn include_path<P>(&mut self, path: P) -> &mut Self
	where
		P: Into<Utf8UnixPathBuf>,
	{
		self.include(Shader::Path(path.into()))
	}

	pub fn define<K, V>(&mut self, key: K, value: V) -> &mut Self
	where
		K: Into<String>,
		V: Into<String>,
	{
		self.define_directives.insert(key.into(), value.into());
		self
	}

	pub fn build(&mut self, shader_source_map: &ShaderSourceMap, device: &Device) -> Result<ShaderModule> {
		let source = self.build_source(shader_source_map)?;

		let shader_module = device.create_shader_module(ShaderModuleDescriptor {
			label: None,
			source: ShaderSource::Wgsl(<Cow<str>>::from(source)),
		});

		Ok(shader_module)
	}

	pub fn build_source(&mut self, shader_source_map: &ShaderSourceMap) -> Result<String> {
		let mut builder = mem::take(self);

		let mut include_blacklist = HashSet::new();

		let mut source = String::new();

		for shader in builder.include_directives.drain() {
			let included_source = shader.process_source(&mut include_blacklist, shader_source_map)?;
			source.push_str(&included_source);
		}

		builder
			.define_directives
			.extend(Self::process_define_directives(&mut source));
		source = builder.apply_define_directives(source);

		Ok(source)
	}

	fn process_define_directives(source: &mut String) -> LinkedHashMap<String, String> {
		let mut define_directives = LinkedHashMap::<String, String>::new();

		// Find all `#define KEY value` in the source
		let re = Regex::new(r#"(?m)^#define (.+?) (.+?)$"#).unwrap();

		let mut ranges = Vec::<Range<usize>>::new();
		for caps in re.captures_iter(source) {
			// The bytes that the `#define ...` statement occupies
			let range = caps.get(0).unwrap().range();
			ranges.push(range);

			let key = caps.get(1).unwrap().as_str().to_owned();
			let value = caps.get(2).unwrap().as_str().to_owned();
			define_directives.insert(key, value);
		}

		// Delete the directives from the source string
		let mut offset: isize = 0;
		for range in ranges {
			let range = (range.start as isize + offset) as usize..(range.end as isize + offset) as usize;

			// Decrease offset since we're deleting sections of text
			offset -= range.len() as isize;

			source.replace_range(range, "");
		}

		define_directives
	}

	fn apply_define_directives(&mut self, mut source: String) -> String {
		let mut directives = self.define_directives.iter().collect::<Vec<_>>();
		// Sort by reverse size, so from biggest key to smallest key
		directives.sort_by(|(key1, _), (key2, _)| key2.cmp(key1));

		for (key, value) in directives {
			source = source.replace(key, value);
		}
		source
	}
}
