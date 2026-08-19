use htmd::options::{BulletListMarker, Options as HtmdOptions};

#[derive(Debug)]
pub struct ToMdOptions {
	pub htmd_options: HtmdOptions,
	pub title_as_h1: bool,
	pub shift_headings: bool,
}

impl ToMdOptions {
	pub fn with_title_as_h1(mut self, title_as_h1: bool) -> Self {
		self.title_as_h1 = title_as_h1;
		self
	}

	pub fn with_shift_headings(mut self, shift_headings: bool) -> Self {
		self.shift_headings = shift_headings;
		self
	}

	pub fn into_htmd_options(self) -> HtmdOptions {
		self.htmd_options
	}

	pub fn title_as_h1(&self) -> bool {
		self.title_as_h1
	}

	pub fn shift_headings(&self) -> bool {
		self.shift_headings
	}
}

impl Default for ToMdOptions {
	fn default() -> Self {
		let options = HtmdOptions {
			bullet_list_marker: BulletListMarker::Dash,
			ul_bullet_spacing: 1,
			ol_number_spacing: 1,
			..Default::default()
		};
		Self {
			htmd_options: options,
			title_as_h1: true,
			shift_headings: true,
		}
	}
}

// region:    --- Froms

impl From<HtmdOptions> for ToMdOptions {
	fn from(opts: HtmdOptions) -> Self {
		Self {
			htmd_options: opts,
			title_as_h1: true,
			shift_headings: true,
		}
	}
}

impl From<Option<ToMdOptions>> for ToMdOptions {
	fn from(o: Option<ToMdOptions>) -> Self {
		o.unwrap_or_default()
	}
}

// endregion: --- Froms
