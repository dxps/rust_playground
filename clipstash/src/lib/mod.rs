pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// Reexporting some types to have them accessible from here
// instead of having to go deeper into the domain model.
pub use domain::clip::field::ShortCode;
pub use domain::clip::ClipError;
pub use domain::time::Time;
