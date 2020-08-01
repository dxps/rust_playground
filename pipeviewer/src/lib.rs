// The library modules, exposed for external (outside of the library) usage.
pub mod args; // for argument parsing
pub mod read; // for reading of the data
pub mod stats; // for outputting the statistics
pub mod write; // for writing of the data

/// 16 kb for the chunk of the bytes
const CHUNK_SIZE: usize = 16 * 1024;
