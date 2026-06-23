#![doc = include_str!("../docs/rustdoc/lib.md")]

// region:    --- Modules

pub mod elem;
pub mod error;
pub mod md;
pub mod selector;
pub mod slimr;

pub use elem::*;
pub use error::{Error, Result};
pub use md::to_md;
pub use selector::*;
pub use slimr::*;

// endregion: --- Modules
