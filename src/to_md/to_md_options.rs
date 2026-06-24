use htmd::options::{BulletListMarker, Options as HtmdOptions};

#[derive(Debug)]
pub struct ToMdOptions(HtmdOptions);

impl ToMdOptions {
	pub fn into_htmd_options(self) -> HtmdOptions {
		self.0
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
		ToMdOptions(options)
	}
}

// region:    --- Froms

impl From<HtmdOptions> for ToMdOptions {
	fn from(opts: HtmdOptions) -> Self {
		ToMdOptions(opts)
	}
}

impl From<Option<ToMdOptions>> for ToMdOptions {
	fn from(o: Option<ToMdOptions>) -> Self {
		o.unwrap_or_default()
	}
}

// endregion: --- Froms
