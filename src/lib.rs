#![doc = include_str!("../docs/rustdoc/lib.md")]

// region:    --- Modules

mod common;
mod elem;
mod error;
mod html_content;
mod md_to_html;
mod pretty;
mod select;
mod slim;
mod to_md;

pub use common::*;
pub use elem::*;
pub use error::*;
pub use html_content::*;
pub use md_to_html::*;
pub use pretty::*;
pub use select::*;
pub use slim::*;
pub use to_md::*;

// endregion: --- Modules
