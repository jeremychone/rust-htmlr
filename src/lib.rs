#![doc = include_str!("../docs/rustdoc/lib.md")]

// region:    --- Modules

mod common;
mod elem;
mod error;
mod html_content;
mod select;
mod slim;
mod to_md;

pub use common::*;
pub use elem::*;

// -- Explicit export for clarity
pub use error::{Error, Result};
pub use html_content::{HtmlContent, HtmlParsed};
pub use select::select;
pub use slim::{SlimOptions, slim};
pub use to_md::to_md;

// endregion: --- Modules
