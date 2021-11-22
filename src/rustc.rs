use super::*;

pub(crate) fn try_inline(storage_dir: OutputDir, code: &str) -> Result<TokenStream, InlineRustError> {
	let input_path = storage_dir.join("inline_rust.rs");
	let executable_path = storage_dir.join("inline_rust");

	File::create(&input_path)?.write_all(code.as_bytes())?;

	let output = Command::new("rustc")
		.args(&[
			"-Ctarget-cpu=native",
			"-Copt-level=0",
			&format!("-o{}", executable_path.display()),
			&format!("{}", input_path.display()),
		])
		.output()?;

	if !output.status.success() {
		return Err(InlineRustError::RustcError(
			String::from_utf8_lossy(&output.stderr).into_owned(),
		));
	}

	let output = Command::new(executable_path).output()?;
	if !output.status.success() {
		return Err(InlineRustError::RuntimeError(
			String::from_utf8_lossy(&output.stdout).into_owned(),
		));
	}

	Ok(TokenStream::from_str(&String::from_utf8_lossy(
		&output.stdout,
	))?)
}