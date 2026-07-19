#[doc = include_str!("../../docs/rustdoc/pretty/pretty_options.md")]
/// Options for the [`pretty`](crate::pretty) function.
#[derive(Clone, Copy, Debug)]
pub struct PrettyOptions {
	/// Number of spaces per indentation level.
	pub ident: u8,

	/// Maximum text-content line length, or `None` to disable wrapping.
	pub wrap: Option<u16>,
}

impl Default for PrettyOptions {
	fn default() -> Self {
		Self {
			ident: 2,
			wrap: Some(80),
		}
	}
}

// region:    --- Froms

impl From<Option<PrettyOptions>> for PrettyOptions {
	fn from(options: Option<PrettyOptions>) -> Self {
		options.unwrap_or_default()
	}
}

// endregion: --- Froms
