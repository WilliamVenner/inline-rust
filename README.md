# inline-rust

This is a stupid macro inspired by [`inline_python`](https://github.com/fusion-engineering/inline-python) that compiles and executes Rust and spits the output directly into your Rust code.

This really should not be used in production. It's slow, dangerous, and cursed.

However, there is a use case of using it to evaluate advanced const expressions, see the example below.

# Example

```rust
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

const CONST_FOR_LOOP: i32 = inline_rust!({
    let mut sum: i32 = 0;
    for n in 0..30 {
        sum += n;
    }
    format!("{}", sum)
});
```