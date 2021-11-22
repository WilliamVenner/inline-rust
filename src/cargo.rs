use super::*;

pub(crate) fn try_inline(
	storage_dir: OutputDir,
	target_dir: TargetDir,
	manifest: &str,
	code: &str,
) -> Result<TokenStream, InlineRustError> {
	// cargo init
	let output = Command::new("cargo")
		.current_dir(&storage_dir)
		.arg("init")
		.output()?;

	if !output.status.success() {
		return Err(InlineRustError::CargoError(
			String::from_utf8_lossy(&output.stderr).into_owned(),
		));
	}

	// Write code
	File::create(storage_dir.join("src").join("main.rs"))?.write_all(code.as_bytes())?;

	// Write manifest
	let cargo_manifest = storage_dir.join("Cargo.toml");
	std::fs::write(
		&cargo_manifest,
		std::fs::read_to_string(&cargo_manifest)?.replace("[dependencies]", &manifest),
	)?;

	// cargo run
	let output = Command::new("cargo")
		.args(&["run", "--target-dir", &format!("{}", target_dir.display())])
		.current_dir(&storage_dir)
		.env("RUSTFLAGS", "-Ctarget-cpu=native")
		.output()?;

	if !output.status.success() {
		return Err(InlineRustError::RuntimeError(
			String::from_utf8_lossy(&output.stderr).into_owned(),
		));
	}

	Ok(TokenStream::from_str(&String::from_utf8_lossy(
		&output.stdout,
	))?)
}