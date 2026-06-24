// region:    --- Types

/// Options for the `slim` function (indentation, etc.).
#[derive(Clone, Copy, Debug, Default)]
pub struct SlimOptions {
	/// Whether to use tabs instead of spaces for indentation.
	pub indent_with_tabs: bool,
	/// Number of spaces per indentation level, or `None` for flat output.
	pub indent: Option<u8>,
}

// endregion: --- Types

// region:    --- Fluid Constructors & Chainables

/// Constructors
impl SlimOptions {
	/// Create a new `SlimOptions` with the given indent and no tabs.
	pub fn from_indent(indent: u8) -> Self {
		Self {
			indent: Some(indent),
			..Default::default()
		}
	}
}

/// Chainables
impl SlimOptions {
	/// Set the number of spaces for indentation (enables formatting).
	pub fn with_indent(mut self, spaces: u8) -> Self {
		self.indent = Some(spaces);
		self
	}

	/// Use tabs instead of spaces for indentation.
	pub fn with_indent_with_tabs(mut self, tabs: bool) -> Self {
		self.indent_with_tabs = tabs;
		self
	}
}

// endregion: --- Fluid Constructors & Chainables

// region:    --- Froms

impl From<Option<SlimOptions>> for SlimOptions {
	fn from(o: Option<SlimOptions>) -> Self {
		o.unwrap_or_default()
	}
}

// endregion: --- Froms
