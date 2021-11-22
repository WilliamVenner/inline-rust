use std::{
	sync::atomic::AtomicUsize,
	time::SystemTime,
	fs::File,
	io::Write,
	process::Command,
	str::FromStr
};

use syn::{
	parse::Parser,
	punctuated::Punctuated,
	spanned::Spanned,
	Token
};

use proc_macro::TokenStream;
use quote::ToTokens;
use sha2::Digest;

mod storage;
use storage::{OutputDir, TargetDir};

mod cargo;
mod rustc;

enum InlineRustError {
    CargoError(String),
    RustcError(String),
    RuntimeError(String),
    Other(Box<dyn std::error::Error>),
}
impl<E: std::error::Error + 'static> From<E> for InlineRustError {
    fn from(err: E) -> Self {
        InlineRustError::Other(Box::new(err))
    }
}
impl Into<TokenStream> for InlineRustError {
    fn into(self) -> TokenStream {
        let str = match self {
            InlineRustError::RuntimeError(str)
            | InlineRustError::CargoError(str)
            | InlineRustError::RustcError(str) => str,
            InlineRustError::Other(err) => err.to_string(),
        };

        syn::Error::new(str.span(), str).to_compile_error().into()
    }
}

fn exec_id(code: &str) -> String {
    static INVOKE_ID: AtomicUsize = AtomicUsize::new(0);

    let invoke_id = INVOKE_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let mut sha256 = sha2::Sha256::new();

    sha256.update(&invoke_id.to_ne_bytes());

    if let Ok(systime) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        sha256.update(&systime.as_nanos().to_ne_bytes());
    }

    sha256.update(code.as_bytes());

    format!("inline_rust_{:x}", sha256.finalize())[0..32].to_string()
}

#[proc_macro]
/// Inline the output of Rust code into your code.
///
/// # Examples
///
/// ```no_run
/// #[macro_use] extern crate inline_rust;
///
/// const CONST_HASH: &'static str = inline_rust!(
///     r#"
///         [dependencies]
///         sha2 = "0.9.8"
///     "#,
///     {
///         use sha2::Digest;
///
///         let mut sum: i32 = 0;
///         for n in 0..30 {
///             sum += n;
///         }
///
///         format!("\"{:x}\"", sha2::Sha256::digest(&sum.to_ne_bytes()))
///     }
/// );
///
/// const CONST_FOR_LOOP: i32 = inline_rust!({
/// 	let mut sum: i32 = 0;
/// 	for n in 0..30 {
/// 		sum += n;
/// 	}
/// 	format!("{}", sum)
/// });
pub fn inline_rust(tokens: TokenStream) -> TokenStream {
    let parser = Punctuated::<syn::Expr, Token![,]>::parse_separated_nonempty;
    let mut parsed = match parser.parse(tokens) {
        Ok(parsed) => parsed,
        Err(error) => return error.into_compile_error().into(),
    };

    let code = match parsed.pop() {
        Some(code) => code.into_value().into_token_stream().to_string(),
        None => return TokenStream::default(),
    };

    let manifest = match parsed.pop().map(|pair| pair.into_value()) {
        Some(manifest) => loop {
            if let syn::Expr::Lit(ref str) = manifest {
                if let syn::Lit::Str(ref str) = str.lit {
                    break Some(str.value());
                }
            }
            return syn::Error::new(
                manifest.span(),
                "Expected string literal for Cargo manifest",
            )
            .to_compile_error()
            .into();
        },
        None => None,
    };

    let code = format!("fn inline_rust() -> impl std::fmt::Display {{\n{}\n}} fn main() {{println!(\"{{}}\", inline_rust())}}", code.trim());

    let exec_id = exec_id(&code);
    let (storage_dir, target_dir) = storage::create_storage_dir(&exec_id).unwrap();

    let result = if let Some(manifest) = manifest {
        cargo::try_inline(storage_dir, target_dir, manifest.trim(), &code)
    } else {
        rustc::try_inline(storage_dir, &code)
    };

    match result {
        Ok(tokens) => tokens,
        Err(err) => err.into(),
    }
}
