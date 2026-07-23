#[derive(Debug, Default)]
pub struct MdToHtmlOptions;

// region:    --- Froms

impl From<Option<MdToHtmlOptions>> for MdToHtmlOptions {
	fn from(options: Option<MdToHtmlOptions>) -> Self {
		options.unwrap_or_default()
	}
}

// endregion: --- Froms
