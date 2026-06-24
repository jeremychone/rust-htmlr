//! HtmlContent and HtmlParsed types for accepting raw or pre-parsed HTML.

use scraper::Html;

/// A type alias for `scraper::Html`, the parsed document tree.
pub type HtmlParsed = Html;

/// Represents either a raw HTML string or a pre-parsed HTML document.
pub enum HtmlContent<'a> {
	/// A raw HTML string.
	Source(&'a str),
	/// A pre-parsed HTML document.
	Parsed(&'a HtmlParsed),
}

// region:    --- Froms

impl<'a> From<&'a String> for HtmlContent<'a> {
	fn from(value: &'a String) -> Self {
		HtmlContent::Source(value)
	}
}

impl<'a> From<&'a str> for HtmlContent<'a> {
	fn from(value: &'a str) -> Self {
		HtmlContent::Source(value)
	}
}

impl<'a> From<&'a HtmlParsed> for HtmlContent<'a> {
	fn from(value: &'a HtmlParsed) -> Self {
		HtmlContent::Parsed(value)
	}
}

// endregion: --- Froms

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_html_content_from_str() {
		let content: HtmlContent = "some html".into();
		match content {
			HtmlContent::Source(s) => assert_eq!(s, "some html"),
			_ => panic!("Expected Source variant"),
		}
	}

	#[test]
	fn test_html_content_from_parsed() {
		let doc = HtmlParsed::parse_document("<p>test</p>");
		let content: HtmlContent = (&doc).into();
		match content {
			HtmlContent::Parsed(d) => {
				// check that the inner_html produces something
				let text = d.root_element().inner_html();
				assert!(text.contains("<p>"));
			}
			_ => panic!("Expected Parsed variant"),
		}
	}
}

// endregion: --- Tests
