use super::MdToHtmlOptions;
use crate::Result;
use pulldown_cmark::{Options, Parser, html};

pub fn md_to_html(md: &str, options: impl Into<MdToHtmlOptions>) -> Result<String> {
	let _opts = options.into();

	let mut parser_options = Options::empty();
	parser_options.insert(Options::ENABLE_TABLES);
	parser_options.insert(Options::ENABLE_FOOTNOTES);
	parser_options.insert(Options::ENABLE_STRIKETHROUGH);
	parser_options.insert(Options::ENABLE_TASKLISTS);
	parser_options.insert(Options::ENABLE_SMART_PUNCTUATION);

	let parser = Parser::new_ext(md, parser_options);
	let mut html_output = String::new();
	html::push_html(&mut html_output, parser);

	Ok(html_output)
}

// region:    --- Tests

#[cfg(test)]
#[path = "to_html_impl_tests.rs"]
mod tests;

// endregion: --- Tests
