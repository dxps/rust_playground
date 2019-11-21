## Traits & Generic Types

This playground includes the features of _trait_ and _generic type_ concepts in Rust.

### Project Structure

This project has a simple structure:
- It includes multiple binaries
- Each binary being declared in the `Cargo.toml` in a `[[bin]]` section
- Each binary can be run using `cargo run --bin {binary-file-name}`<br/> 
  Example: `cargo run --bin tagged_gen_type`

### Binaries & Usage

- `tagged_gen_type`
    - has a `struct` that includes a generic type (`T`)
    - run: `cargo run --bin tagged_gen_type`
- `dyn_dispatch_show1`
    - shows how a vector (homogeneous type structure) can contain different data types, based on a common trait.
    - `&dyn` is a language feature allowing a dynamic (at runtime, versus static at compile time) dispatch
    - run: `cargo run --bin dyn_dispatch_show1`

