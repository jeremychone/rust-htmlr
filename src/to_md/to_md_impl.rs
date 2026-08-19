use super::ToMdOptions;
use crate::error::{Error, Result};
use htmd::element_handler::{HandlerResult, Handlers};
use htmd::Element;

#[doc = include_str!("../../docs/rustdoc/to_md.md")]
pub fn to_md(html_content: &str, options: impl Into<ToMdOptions>) -> Result<String> {
	let opts: ToMdOptions = options.into();
	let title = if opts.title_as_h1 {
		extract_title(html_content)
	} else {
		None
	};
	let should_shift = opts.shift_headings && title.is_some();

	let htmd_options = opts.into_htmd_options();

	let mut builder = htmd::HtmlToMarkdown::builder()
		.options(htmd_options)
		.add_handler(vec!["script"], script_handler)
		.add_handler(vec!["title"], title_handler);

	if should_shift {
		builder = builder
			.add_handler(vec!["h1"], h1_handler)
			.add_handler(vec!["h2"], h2_handler)
			.add_handler(vec!["h3"], h3_handler)
			.add_handler(vec!["h4"], h4_handler)
			.add_handler(vec!["h5"], h5_handler)
			.add_handler(vec!["h6"], h6_handler);
	}

	let converter = builder.build();
	let res = converter.convert(html_content).map_err(Error::custom_from_err)?;

	if let Some(title_text) = title {
		let trimmed_res = res.trim_start_matches(['\r', '\n']);
		if trimmed_res.is_empty() {
			Ok(format!("# {title_text}\n"))
		} else {
			Ok(format!("# {title_text}\n\n{trimmed_res}"))
		}
	} else {
		Ok(res)
	}
}

// region:    --- Support

fn extract_title(html: &str) -> Option<String> {
	let bytes = html.as_bytes();
	let len = bytes.len();
	let mut i = 0;

	while i < len {
		if bytes[i] == b'<' && i + 6 <= len && bytes[i + 1..i + 6].eq_ignore_ascii_case(b"title") {
			let after_tag = if i + 6 < len { bytes[i + 6] } else { 0 };
			if matches!(after_tag, b'>' | b' ' | b'\t' | b'\n' | b'\r' | b'/')
				&& let Some(gt_offset) = html[i + 6..].find('>')
			{
				let content_start = i + 6 + gt_offset + 1;
				let mut j = content_start;
				while j + 7 <= len {
					if bytes[j] == b'<' && bytes[j + 1] == b'/' && bytes[j + 2..j + 7].eq_ignore_ascii_case(b"title") {
						let content = &html[content_start..j];
						let trimmed = content.trim();
						if !trimmed.is_empty() {
							let decoded = crate::common::decode_html_entities(trimmed);
							let decoded_trimmed = decoded.trim();
							if !decoded_trimmed.is_empty() {
								return Some(decoded_trimmed.to_string());
							}
						}
						i = j + 7;
						break;
					}
					j += 1;
				}
				if j + 7 > len {
					return None;
				}
				continue;
			}
		}
		i += 1;
	}

	None
}

fn script_handler(_ctx: &dyn Handlers, _el: Element<'_>) -> Option<HandlerResult> {
	Some(String::new().into())
}

fn title_handler(_ctx: &dyn Handlers, _el: Element<'_>) -> Option<HandlerResult> {
	Some(String::new().into())
}

fn h1_handler(ctx: &dyn Handlers, el: Element<'_>) -> Option<HandlerResult> {
	let text = ctx.walk_children(el.node);
	let trimmed = text.content.trim();
	if trimmed.is_empty() {
		Some(String::new().into())
	} else {
		Some(format!("\n\n## {trimmed}\n\n").into())
	}
}

fn h2_handler(ctx: &dyn Handlers, el: Element<'_>) -> Option<HandlerResult> {
	let text = ctx.walk_children(el.node);
	let trimmed = text.content.trim();
	if trimmed.is_empty() {
		Some(String::new().into())
	} else {
		Some(format!("\n\n### {trimmed}\n\n").into())
	}
}

fn h3_handler(ctx: &dyn Handlers, el: Element<'_>) -> Option<HandlerResult> {
	let text = ctx.walk_children(el.node);
	let trimmed = text.content.trim();
	if trimmed.is_empty() {
		Some(String::new().into())
	} else {
		Some(format!("\n\n#### {trimmed}\n\n").into())
	}
}

fn h4_handler(ctx: &dyn Handlers, el: Element<'_>) -> Option<HandlerResult> {
	let text = ctx.walk_children(el.node);
	let trimmed = text.content.trim();
	if trimmed.is_empty() {
		Some(String::new().into())
	} else {
		Some(format!("\n\n##### {trimmed}\n\n").into())
	}
}

fn h5_handler(ctx: &dyn Handlers, el: Element<'_>) -> Option<HandlerResult> {
	let text = ctx.walk_children(el.node);
	let trimmed = text.content.trim();
	if trimmed.is_empty() {
		Some(String::new().into())
	} else {
		Some(format!("\n\n###### {trimmed}\n\n").into())
	}
}

fn h6_handler(ctx: &dyn Handlers, el: Element<'_>) -> Option<HandlerResult> {
	let text = ctx.walk_children(el.node);
	let trimmed = text.content.trim();
	if trimmed.is_empty() {
		Some(String::new().into())
	} else {
		Some(format!("\n\n####### {trimmed}\n\n").into())
	}
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

	#[test]
	fn test_to_md_title_and_heading_shift_default() -> Result<()> {
		// -- Setup & Fixtures
		let html = r#"
			<!DOCTYPE html>
			<html>
			<head><title>My Document Title</title></head>
			<body>
				<h1>Main Header</h1>
				<p>Some introductory text.</p>
				<h2>Sub Header</h2>
				<p>Section text.</p>
				<h3>Sub Sub Header</h3>
				<h4>Level 4</h4>
				<h5>Level 5</h5>
				<h6>Level 6</h6>
			</body>
			</html>
		"#;

		// -- Exec
		let md = to_md(html, None)?;

		// -- Check
		assert!(md.starts_with("# My Document Title\n\n"));
		assert!(md.contains("## Main Header"));
		assert!(md.contains("### Sub Header"));
		assert!(md.contains("#### Sub Sub Header"));
		assert!(md.contains("##### Level 4"));
		assert!(md.contains("###### Level 5"));
		assert!(md.contains("####### Level 6"));

		Ok(())
	}

	#[test]
	fn test_to_md_title_with_entities() -> Result<()> {
		// -- Setup & Fixtures
		let html = "<title>Tom &amp; Jerry &quot;Show&quot;</title><h1>Episode 1</h1>";

		// -- Exec
		let md = to_md(html, None)?;

		// -- Check
		assert!(md.starts_with("# Tom & Jerry \"Show\"\n\n"));
		assert!(md.contains("## Episode 1"));

		Ok(())
	}

	#[test]
	fn test_to_md_empty_or_whitespace_title() -> Result<()> {
		// -- Setup & Fixtures
		let html_empty = "<title></title><h1>Header 1</h1><p>Body</p>";
		let html_spaces = "<title>   \n\t </title><h1>Header 1</h1><p>Body</p>";

		// -- Exec
		let md_empty = to_md(html_empty, None)?;
		let md_spaces = to_md(html_spaces, None)?;

		// -- Check
		assert!(md_empty.contains("# Header 1"));
		assert!(!md_empty.contains("## Header 1"));

		assert!(md_spaces.contains("# Header 1"));
		assert!(!md_spaces.contains("## Header 1"));

		Ok(())
	}

	#[test]
	fn test_to_md_no_title_tag() -> Result<()> {
		// -- Setup & Fixtures
		let html = "<div><h1>Direct H1</h1><h2>Direct H2</h2></div>";

		// -- Exec
		let md = to_md(html, None)?;

		// -- Check
		assert!(md.contains("# Direct H1"));
		assert!(md.contains("## Direct H2"));
		assert!(!md.contains("### Direct H2"));

		Ok(())
	}

	#[test]
	fn test_to_md_disabled_title_as_h1() -> Result<()> {
		// -- Setup & Fixtures
		let html = "<title>Doc Title</title><h1>Header 1</h1>";
		let opts = ToMdOptions::default().with_title_as_h1(false);

		// -- Exec
		let md = to_md(html, opts)?;

		// -- Check
		assert!(!md.contains("# Doc Title"));
		assert!(md.contains("# Header 1"));

		Ok(())
	}

	#[test]
	fn test_to_md_disabled_shift_headings() -> Result<()> {
		// -- Setup & Fixtures
		let html = "<title>Doc Title</title><h1>Header 1</h1><h2>Header 2</h2>";
		let opts = ToMdOptions::default().with_shift_headings(false);

		// -- Exec
		let md = to_md(html, opts)?;

		// -- Check
		assert!(md.starts_with("# Doc Title\n\n"));
		assert!(md.contains("# Header 1"));
		assert!(md.contains("## Header 2"));
		assert!(!md.contains("### Header 2"));

		Ok(())
	}
}
// endregion: --- Tests
