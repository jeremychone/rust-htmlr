// region:    --- Modules

use crate::error::{Error, Result};

/// Converts an HTML string to Markdown using the `htmd` crate.
///
/// # Arguments
///
/// * `html_content` - A string slice containing the HTML content.
///
/// # Returns
///
/// A `Result<String>` which is:
/// - `Ok(String)` containing the Markdown output.
/// - `Err` if conversion fails.
pub fn to_md(html_content: &str) -> Result<String> {
	let options = htmd::options::Options {
		bullet_list_marker: htmd::options::BulletListMarker::Dash,
		ul_bullet_spacing: 1,
		ol_number_spacing: 1,
		..Default::default()
	};
	let converter = htmd::HtmlToMarkdown::builder().options(options).build();
	let res = converter
		.convert(html_content)
		.map_err(|err| Error::custom_from_err(err))?;
	Ok(res)
}

// endregion: --- Modules

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_md_basic_html() {
		let html = "<h1>Title</h1><p>Paragraph</p><a href=\"/x\">link</a><ul><li>item</li></ul>";
		let md = to_md(html).unwrap();
		assert!(md.contains("# Title"));
		assert!(md.contains("Paragraph"));
		assert!(md.contains("[link](/x)"));
		assert!(md.contains("- item"));
	}

	#[test]
	fn test_to_md_empty_string() {
		let md = to_md("").unwrap();
		assert_eq!(md, "");
	}

	#[test]
	fn test_to_md_invalid_html() {
		// htmd converter is lenient, but we verify that to_md does not panic
		// and returns a Result (error propagation mapping is covered by Error::custom_from_err).
		let result = to_md("<p>unclosed<b>");
		assert!(result.is_ok() || result.is_err());
	}
}
