[![crates.io](https://img.shields.io/crates/v/inline-rust.svg)](https://crates.io/crates/inline-rust)
[![docs.rs](https://docs.rs/inline-rust/badge.svg)](https://docs.rs/inline-rust/)
[![license](https://img.shields.io/crates/l/inline-rust)](https://github.com/WilliamVenner/inline-rust/blob/master/LICENSE)

# inline-rust

This is a stupid macro inspired by [`inline-python`](https://github.com/fusion-engineering/inline-python) that compiles and executes Rust and spits the output directly into your Rust code.

There is a use case of using it to evaluate advanced "const" expressions, see the example below... if you dare.

# Usage

```toml
[dependencies]
inline-rust = "*"
```

# Example

```rust
// Compiles using cargo
const CONST_HASH: &'static str = inline_rust!(
    r#"
        [dependencies]
        sha2 = "0.9.8"
    "#,
    {
        use sha2::Digest;

        let mut sum: i32 = 0;
        for n in 0..30 {
            sum += n;
        }

        format!("\"{:x}\"", sha2::Sha256::digest(&sum.to_ne_bytes()))
    }
);

// Compiles using rustc
const CONST_FOR_LOOP: i32 = inline_rust!({
    let mut sum: i32 = 0;
    for n in 0..30 {
        sum += n;
    }
    format!("{}", sum)
});
```