## Rust Experiments

This project is a collection of various rust experiments such as showcases of nice and useful crates, guidelines, idiomatic ways of writing Rust code.

### Running the binaries

Each binary is declared in `Cargo.toml` in a section like this:
```toml
[[bin]]
name = "some_name"
path = "src/bin/some_name.rs"
```
And running it is done using `cargo run --bin some_name`.
