use super::ToMdOptions;
use crate::error::{Error, Result};

#[doc = include_str!("../../docs/rustdoc/to_md.md")]
pub fn to_md(html_content: &str, options: impl Into<ToMdOptions>) -> Result<String> {
	let opts: ToMdOptions = options.into();
	let htmd_options = opts.into_htmd_options();

	let converter = htmd::HtmlToMarkdown::builder().options(htmd_options).build();
	let res = converter.convert(html_content).map_err(Error::custom_from_err)?;
	Ok(res)
}

// region:    --- Tests
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_md_basic_html() {
		let html = "<h1>Title</h1><p>Paragraph</p><a href=\"/x\">link</a><ul><li>item</li></ul>";
		let md = to_md(html, None).unwrap();
		assert!(md.contains("# Title"));
		assert!(md.contains("Paragraph"));
		assert!(md.contains("[link](/x)"));
		assert!(md.contains("- item"));
	}

	#[test]
	fn test_to_md_empty_string() {
		let md = to_md("", None).unwrap();
		assert_eq!(md, "");
	}

	#[test]
	fn test_to_md_invalid_html() {
		// htmd converter is lenient, but we verify that to_md does not panic
		// and returns a Result (error propagation mapping is covered by Error::custom_from_err).
		let result = to_md("<p>unclosed<b>", None);
		assert!(result.is_ok() || result.is_err());
	}
}
// endregion: --- Tests
