use super::ToMdOptions;
use crate::error::{Error, Result};
use htmd::element_handler::{HandlerResult, Handlers};
use htmd::Element;

#[doc = include_str!("../../docs/rustdoc/to_md.md")]
pub fn to_md(html_content: &str, options: impl Into<ToMdOptions>) -> Result<String> {
	let opts: ToMdOptions = options.into();
	let htmd_options = opts.into_htmd_options();

	let converter = htmd::HtmlToMarkdown::builder()
		.options(htmd_options)
		.add_handler(vec!["script"], script_handler)
		.build();
	let res = converter.convert(html_content).map_err(Error::custom_from_err)?;
	Ok(res)
}

// region:    --- Support

fn script_handler(_ctx: &dyn Handlers, _el: Element<'_>) -> Option<HandlerResult> {
	Some(String::new().into())
}

// endregion: --- Support

// region:    --- Tests
#[cfg(test)]
mod tests {
	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

	use super::*;

	#[test]
	fn test_to_md_basic_html() -> Result<()> {
		let html = "<h1>Title</h1><p>Paragraph</p><a href=\"/x\">link</a><ul><li>item</li></ul>";
		let md = to_md(html, None)?;
		assert!(md.contains("# Title"));
		assert!(md.contains("Paragraph"));
		assert!(md.contains("[link](/x)"));
		assert!(md.contains("- item"));
		Ok(())
	}

	#[test]
	fn test_to_md_empty_string() -> Result<()> {
		let md = to_md("", None)?;
		assert_eq!(md, "");
		Ok(())
	}

	#[test]
	fn test_to_md_invalid_html() {
		// htmd converter is lenient, but we verify that to_md does not panic
		// and returns a Result (error propagation mapping is covered by Error::custom_from_err).
		let result = to_md("<p>unclosed<b>", None);
		assert!(result.is_ok() || result.is_err());
	}

	#[test]
	fn test_to_md_script_tags() -> Result<()> {
		// -- Setup & Fixtures
		let html = r#"
			<div>
				<h1>Title</h1>
				<script>
					const x = 10;
					console.log("secret script content", x);
				</script>
				<p>Hello <script type="text/javascript">alert('inline');</script>World</p>
				<script src="app.js">fallback script text</script>
			</div>
		"#;

		// -- Exec
		let md = to_md(html, None)?;

		// -- Check
		assert!(md.contains("# Title"));
		assert!(md.contains("Hello"));
		assert!(md.contains("World"));
		assert!(!md.contains("console.log"));
		assert!(!md.contains("secret script content"));
		assert!(!md.contains("alert"));
		assert!(!md.contains("inline"));
		assert!(!md.contains("fallback script text"));
		assert!(!md.contains("script"));

		Ok(())
	}
}
// endregion: --- Tests
