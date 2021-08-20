#[cfg(not(windows))]
compile_error!("win-overlay is exclusive to windows.");

#[macro_use]
pub mod utils;
pub mod win_overlay;
