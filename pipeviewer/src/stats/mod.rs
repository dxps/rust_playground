//! The stats module contains the logic that measures and shows the statistics.
//!
//! ## stats_loop
//! This is the only element that is exposed as API to be used by the lib users.

// Declaring the submodules of this `stats` modules.
mod stats_logic;
mod timer;

// Reexporting (to be used as `stats::stats_loop()`, see main).
pub use stats_logic::stats_loop;
