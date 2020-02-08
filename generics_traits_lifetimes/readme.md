## Generic Types & Traits

This playground includes the features of _trait_ and _generic type_ concepts in Rust.

### Project Structure

This project has a simple structure:
- It includes multiple binaries
- Each binary is being declared in the `Cargo.toml` in its own `[[bin]]` section
- Based on this, each binary can be run using `cargo run --bin {binary-file-name}`<br/> 
  Example: `cargo run --bin generics_struct_tagged`

Each executable (so called binary, not lib) describes its purpose.
