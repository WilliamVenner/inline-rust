use std::path::{Path, PathBuf};

pub(crate) struct TargetDir {
	inner: PathBuf,
	exec_id: String,
}
impl Drop for TargetDir {
	fn drop(&mut self) {
		let _ = self.inner.join("debug").read_dir().map(|read_dir| {
			read_dir
				.filter_map(|entry| entry.ok())
				.filter(|entry| {
					entry
						.file_type()
						.ok()
						.map(|file_type| file_type.is_file())
						.unwrap_or(false)
				})
				.filter(|entry| {
					entry
						.path()
						.file_stem()
						.map(|file_stem| file_stem.to_string_lossy() == self.exec_id)
						.unwrap_or(false)
				})
				.for_each(|entry| {
					let _ = std::fs::remove_file(entry.path());
				})
		});
	}
}
impl AsRef<Path> for TargetDir {
	fn as_ref(&self) -> &Path {
		self.inner.as_ref()
	}
}
impl std::ops::Deref for TargetDir {
	type Target = PathBuf;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
impl std::ops::DerefMut for TargetDir {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

pub(crate) struct OutputDir(PathBuf);
impl Drop for OutputDir {
	fn drop(&mut self) {
		let _ = std::fs::remove_dir_all(&self.0);
	}
}
impl AsRef<Path> for OutputDir {
	fn as_ref(&self) -> &Path {
		self.0.as_ref()
	}
}
impl std::ops::Deref for OutputDir {
	type Target = PathBuf;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl std::ops::DerefMut for OutputDir {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

pub(crate) fn create_storage_dir(exec_id: &str) -> Result<(OutputDir, TargetDir), std::io::Error> {
	let mut storage_dir = std::env::temp_dir();
	storage_dir.push("inline_rust");

	let target_dir = storage_dir.join("target");
	std::fs::create_dir_all(&target_dir)?;

	storage_dir.push(exec_id);
	std::fs::create_dir_all(&storage_dir)?;

	Ok((
		OutputDir(storage_dir),
		TargetDir {
			inner: target_dir,
			exec_id: exec_id.to_string(),
		},
	))
}