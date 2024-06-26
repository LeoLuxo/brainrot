#![allow(dead_code)]

use anyhow::{Ok, Result};
use image::GenericImageView;
use vek::Extent2;
use wgpu::{
	naga::back::msl::sampler::Filter, AddressMode, CompareFunction, Device, Extent3d, FilterMode, ImageCopyTexture,
	ImageDataLayout, Origin3d, Queue, Sampler, SamplerBorderColor, SamplerDescriptor, TextureAspect, TextureDescriptor,
	TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
};

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub struct TextureAsset {
	pub texture: wgpu::Texture,
	pub view: TextureView,
	pub sampler: Sampler,
}

impl TextureAsset {
	pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

	pub fn from_bytes(
		device: &Device,
		queue: &Queue,
		bytes: &[u8],
		filter_mode: FilterMode,
		label: Option<&str>,
	) -> Result<Self> {
		let img = image::load_from_memory(bytes)?;
		Self::from_image(device, queue, &img, filter_mode, label)
	}

	pub fn from_image(
		device: &Device,
		queue: &Queue,
		img: &image::DynamicImage,
		filter_mode: FilterMode,
		label: Option<&str>,
	) -> Result<Self> {
		let rgba = img.to_rgba8();
		let dimensions = img.dimensions();

		let size = Extent3d {
			width: dimensions.0,
			height: dimensions.1,
			depth_or_array_layers: 1,
		};
		let texture = device.create_texture(&TextureDescriptor {
			label,
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: TextureDimension::D2,
			format: TextureFormat::Rgba8UnormSrgb,
			usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
			view_formats: &[],
		});

		queue.write_texture(
			ImageCopyTexture {
				aspect: TextureAspect::All,
				texture: &texture,
				mip_level: 0,
				origin: Origin3d::ZERO,
			},
			&rgba,
			ImageDataLayout {
				offset: 0,
				bytes_per_row: Some(4 * dimensions.0),
				rows_per_image: Some(dimensions.1),
			},
			size,
		);

		let view = texture.create_view(&TextureViewDescriptor::default());
		let sampler = device.create_sampler(&SamplerDescriptor {
			address_mode_u: AddressMode::Repeat,
			address_mode_v: AddressMode::Repeat,
			address_mode_w: AddressMode::Repeat,
			mag_filter: filter_mode,
			min_filter: filter_mode,
			mipmap_filter: filter_mode,
			..Default::default()
		});

		Ok(Self { texture, view, sampler })
	}

	pub fn create_depth_texture(device: &Device, size: Extent2<u32>, label: Option<&str>) -> Self {
		let size = Extent3d {
			width: size.w,
			height: size.h,
			depth_or_array_layers: 1,
		};

		let desc = TextureDescriptor {
			label,
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: TextureDimension::D2,
			format: Self::DEPTH_FORMAT,
			usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
			view_formats: &[],
		};

		let texture = device.create_texture(&desc);

		let view = texture.create_view(&TextureViewDescriptor::default());

		let sampler = device.create_sampler(&SamplerDescriptor {
			address_mode_u: AddressMode::ClampToEdge,
			address_mode_v: AddressMode::ClampToEdge,
			address_mode_w: AddressMode::ClampToEdge,
			mag_filter: FilterMode::Linear,
			min_filter: FilterMode::Linear,
			mipmap_filter: FilterMode::Nearest,
			compare: Some(CompareFunction::LessEqual),
			lod_min_clamp: 0.0,
			lod_max_clamp: 100.0,
			..Default::default()
		});

		Self { texture, view, sampler }
	}

	pub fn create_storage_sampler_texture(
		device: &Device,
		size: Extent2<u32>,
		filter_mode: FilterMode,
		format: TextureFormat,
		label: Option<&str>,
	) -> Self {
		let size = Extent3d {
			width: size.w,
			height: size.h,
			depth_or_array_layers: 1,
		};

		let desc = TextureDescriptor {
			label,
			size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: TextureDimension::D2,
			format,
			usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC,
			view_formats: &[],
		};

		let texture = device.create_texture(&desc);

		let view = texture.create_view(&TextureViewDescriptor::default());

		let sampler = device.create_sampler(&SamplerDescriptor {
			address_mode_u: AddressMode::ClampToBorder,
			address_mode_v: AddressMode::ClampToBorder,
			address_mode_w: AddressMode::ClampToBorder,
			border_color: Some(SamplerBorderColor::OpaqueBlack),
			mag_filter: filter_mode,
			min_filter: filter_mode,
			mipmap_filter: filter_mode,
			..Default::default()
		});

		Self { texture, view, sampler }
	}
}

/*
--------------------------------------------------------------------------------
||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||
--------------------------------------------------------------------------------
*/

pub struct TextureArray {
	pub textures: Vec<TextureAsset>,
}

impl TextureArray {
	pub fn from_bytes(
		device: &Device,
		queue: &Queue,
		array: Vec<(&[u8], Option<&str>)>,
		filter_mode: FilterMode,
	) -> Result<Self> {
		let mut textures = Vec::new();

		for (bytes, label) in array {
			textures.push(TextureAsset::from_bytes(device, queue, bytes, filter_mode, label)?);
		}

		Ok(TextureArray { textures })
	}

	#[allow(dead_code)]
	pub fn from_images(
		device: &Device,
		queue: &Queue,
		filter_mode: FilterMode,
		array: Vec<(&image::DynamicImage, Option<&str>)>,
	) -> Result<Self> {
		let mut textures = Vec::new();

		for (image, label) in array {
			textures.push(TextureAsset::from_image(device, queue, image, filter_mode, label)?);
		}

		Ok(TextureArray { textures })
	}

	pub fn get_samplers(&self) -> Vec<&Sampler> {
		self.textures.iter().map(|t| &t.sampler).collect()
	}

	pub fn get_views(&self) -> Vec<&TextureView> {
		self.textures.iter().map(|t| &t.view).collect()
	}

	pub fn len(&self) -> usize {
		self.textures.len()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}
