## Traits & Generic Types

This playground includes the features of _trait_ and _generic type_ concepts in Rust.

### Project Structure

This project has a simple structure:
- It includes multiple binaries
- Each binary being declared in the `Cargo.toml` in a `[[bin]]` section
- Each binary can be run using `cargo run --bin {binary-file-name}`<br/> 
  Example: `cargo run --bin tagged_gen_type`

### Binaries & Usage

- `tagged_gen_type` - has a `struct` that includes a generic type (`T`)

