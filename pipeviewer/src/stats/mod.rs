// Declaring the submodules of this `stats` modules.
mod stats_logic;
mod timer;

// Reexporting (to be used as `stats::stats_loop()`, see main).
pub use stats_logic::stats_loop;
