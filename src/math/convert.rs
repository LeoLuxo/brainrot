pub trait Converter<To> {
	fn convert(self) -> To;
}

impl Converter<egui::Vec2> for vek::Vec2<f32> {
	/// Convert from `vek::Vec2` -> `mint::Vector2` -> `egui::Vec2`
	#[inline]
	fn convert(self) -> egui::Vec2 {
		Into::<mint::Vector2<f32>>::into(self).into()
	}
}

impl<T: winit::dpi::Pixel> Converter<vek::Extent2<T>> for winit::dpi::PhysicalSize<T> {
	/// Convert from `winit::PhysicalSize` -> `vek::Extent2`
	#[inline]
	fn convert(self) -> vek::Extent2<T> {
		vek::Extent2::new(self.width, self.height)
	}
}

impl<T: winit::dpi::Pixel> Converter<winit::dpi::PhysicalSize<T>> for vek::Extent2<T> {
	/// Convert from `vek::Extent2` -> `winit::PhysicalSize`
	#[inline]
	fn convert(self) -> winit::dpi::PhysicalSize<T> {
		winit::dpi::PhysicalSize::new(self.w, self.h)
	}
}

impl<T> Converter<[T; 2]> for vek::Extent2<T> {
	/// Convert from `vek::Extent2` -> `winit::PhysicalSize`
	#[inline]
	fn convert(self) -> [T; 2] {
		[self.w, self.h]
	}
}
