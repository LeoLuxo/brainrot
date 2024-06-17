// Macros

#[macro_export]
macro_rules! path {
	($path:expr) => {
		$crate::lib_crates::typed_path::Utf8TypedPath::derive($path).to_path_buf()
	};
}

#[macro_export]
macro_rules! native_pathbuf {
	($path:expr) => {
		TryInto::<PathBuf>::try_into($path)
	};
}

#[macro_export]
macro_rules! native_path {
	($path:expr) => {
		TryInto::<Path>::try_into($path)
	};
}

#[macro_export]
macro_rules! src_path {
	() => {
		path!(env!("CARGO_MANIFEST_DIR")).join("src")
	};
}

#[macro_export]
macro_rules! rooted_path {
	($path:expr) => {
		$crate::lib_crates::typed_path::Utf8UnixPath::new("/")
			.join($path)
			.normalize()
	};
}
