#[derive(Debug)]
pub struct MdToHtmlOptions {
	pub code_block_html_escape_content: bool,
	pub code_block_mermaid_as_pre: bool,
}

impl Default for MdToHtmlOptions {
	fn default() -> Self {
		Self {
			code_block_html_escape_content: true,
			code_block_mermaid_as_pre: true,
		}
	}
}

// region:    --- Froms

impl From<Option<MdToHtmlOptions>> for MdToHtmlOptions {
	fn from(options: Option<MdToHtmlOptions>) -> Self {
		options.unwrap_or_default()
	}
}

// endregion: --- Froms
