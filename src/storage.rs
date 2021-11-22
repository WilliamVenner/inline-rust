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

pub(crate) struct StorageDir(PathBuf);
impl StorageDir {
	pub(crate) fn create(exec_id: &str) -> Result<StorageDir, std::io::Error> {
		let mut storage_dir = std::env::temp_dir();
		storage_dir.push("inline_rust");
		storage_dir.push(exec_id);

		std::fs::create_dir_all(&storage_dir)?;

		Ok(StorageDir(storage_dir))
	}

	pub(crate) fn target_dir(&self, exec_id: String) -> Result<TargetDir, std::io::Error> {
		let target_dir = self.parent().unwrap().join("target");

		std::fs::create_dir_all(&target_dir)?;

		Ok(TargetDir {
			inner: target_dir,
			exec_id
		})
	}
}
impl Drop for StorageDir {
	fn drop(&mut self) {
		let _ = std::fs::remove_dir_all(&self.0);
	}
}
impl AsRef<Path> for StorageDir {
	fn as_ref(&self) -> &Path {
		self.0.as_ref()
	}
}
impl std::ops::Deref for StorageDir {
	type Target = PathBuf;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl std::ops::DerefMut for StorageDir {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}