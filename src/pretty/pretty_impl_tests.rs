type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

use super::*;

#[test]
fn test_pretty_options_default() -> Result<()> {
	// -- Setup & Fixtures
	let options = PrettyOptions::from(None);

	// -- Exec
	let indent = options.ident;
	let wrap = options.wrap;

	// -- Check
	assert_eq!(indent, 2);
	assert_eq!(wrap, Some(80));

	Ok(())
}

#[test]
fn test_pretty_option_forms() -> Result<()> {
	// -- Setup & Fixtures
	let html = "<div><p>Hello</p></div>";

	// -- Exec
	let direct = pretty(
		html,
		PrettyOptions {
			ident: 4,
			..Default::default()
		},
	);
	let optional = pretty(
		html,
		Some(PrettyOptions {
			ident: 4,
			..Default::default()
		}),
	);
	let defaulted = pretty(html, None);

	// -- Check
	assert_eq!(direct, optional);
	assert!(defaulted.contains("<p>Hello</p>"));

	Ok(())
}

#[test]
fn test_pretty_block_elements() -> Result<()> {
	// -- Setup & Fixtures
	let html = "<div><p>Hello <strong>there</strong></p><section><br></section></div>";

	// -- Exec
	let result = pretty(
		html,
		PrettyOptions {
			ident: 4,
			..Default::default()
		},
	);

	// -- Check
	assert_eq!(
		result,
		"<div>\n    <p>Hello <strong>there</strong></p>\n    <section><br></section>\n</div>"
	);

	Ok(())
}

#[test]
fn test_pretty_raw_element_content() -> Result<()> {
	// -- Setup & Fixtures
	let html = r#"<div data-value="a > b"><script>if (a < b) { call(">"); }</script></div>"#;

	// -- Exec
	let result = pretty(html, None);

	// -- Check
	assert_eq!(
		result,
		"<div data-value=\"a > b\">\n  <script>if (a < b) { call(\">\"); }</script>\n</div>"
	);

	Ok(())
}

#[test]
fn test_pretty_normalized_dom() -> Result<()> {
	// -- Setup & Fixtures
	let html = "<div><p>Hello<section>World</section></div>";

	// -- Exec
	let result = pretty(html, None);

	// -- Check
	assert_eq!(result, "<div>\n  <p>Hello</p>\n  <section>World</section>\n</div>");

	Ok(())
}

#[test]
fn test_pretty_custom_element() -> Result<()> {
	// -- Setup & Fixtures
	let html = "<div><my-tag><p>Hello</p></my-tag></div>";

	// -- Exec
	let result = pretty(html, None);

	// -- Check
	assert_eq!(result, "<div><my-tag>\n  <p>Hello</p></my-tag>\n</div>");

	Ok(())
}

#[test]
fn test_pretty_text_wrap_long_content() -> Result<()> {
	// -- Setup & Fixtures
	let text = vec!["word"; 25].join(" ");
	let html = format!("<p>{text}</p>");
	let options = PrettyOptions {
		ident: 2,
		wrap: Some(40),
	};

	// -- Exec
	let result = pretty(&html, options);

	// -- Check
	let lines = result.lines().collect::<Vec<_>>();
	assert_eq!(lines.first(), Some(&"<p>"));
	assert_eq!(lines.last(), Some(&"</p>"));
	assert!(lines[1..lines.len() - 1].iter().all(|line| line.trim_start().chars().count() <= 40));

	Ok(())
}

#[test]
fn test_pretty_text_wrap_inline_children() -> Result<()> {
	// -- Setup & Fixtures
	let html = "<p>First words followed by <strong>important inline content that remains marked up</strong> and more words.</p>";
	let options = PrettyOptions {
		ident: 2,
		wrap: Some(30),
	};

	// -- Exec
	let result = pretty(html, options);

	// -- Check
	assert!(result.starts_with("<p>\n  "));
	assert!(result.contains("<strong>"));
	assert!(result.contains("</strong>"));
	assert!(result.ends_with("\n</p>"));

	Ok(())
}

#[test]
fn test_pretty_text_wrap_disabled() -> Result<()> {
	// -- Setup & Fixtures
	let html = "<p>This text is deliberately longer than a small wrapping width but wrapping is disabled.</p>";
	let options = PrettyOptions {
		ident: 2,
		wrap: None,
	};

	// -- Exec
	let result = pretty(html, options);

	// -- Check
	assert_eq!(result, html);

	Ok(())
}

#[test]
fn test_pretty_text_wrap_skips_block_children() -> Result<()> {
	// -- Setup & Fixtures
	let text = vec!["word"; 25].join(" ");
	let html = format!("<blockquote><div>{text}</div></blockquote>");
	let options = PrettyOptions {
		ident: 2,
		wrap: Some(20),
	};

	// -- Exec
	let result = pretty(&html, options);

	// -- Check
	assert_eq!(result, format!("<blockquote>\n  <div>{text}</div>\n</blockquote>"));

	Ok(())
}

#[test]
fn test_pretty_head_child_elements() -> Result<()> {
	// -- Setup & Fixtures
	let html = r#"<!doctype html><html><head><meta charset="utf-8"><title>Example</title><link rel="stylesheet" href="site.css"></head><body></body></html>"#;

	// -- Exec
	let result = pretty(html, None);

	// -- Check
	assert!(result.contains("\n  <head>\n    <meta "));
	assert!(result.contains("\n    <title>Example</title>\n    <link "));
	assert!(result.contains("\n  </head>\n"));

	Ok(())
}

#[test]
fn test_pretty_blank_line_between_head_and_body() -> Result<()> {
	// -- Setup & Fixtures
	let html =
		"<!doctype html><html><head><title>Example</title></head><body><main>Content</main></body></html>";

	// -- Exec
	let result = pretty(html, None);

	// -- Check
	assert_eq!(
		result,
		"<!DOCTYPE html>\n<html>\n  <head>\n    <title>Example</title>\n  </head>\n\n  <body>\n    <main>Content</main>\n  </body>\n</html>"
	);

	Ok(())
}

#[test]
fn test_pretty_pre_code_starts_on_new_line() -> Result<()> {
	// -- Setup & Fixtures
	let html = r#"<div><pre><code class="language-mermaid">graph TD
    Start --> Review
    Review --> Complete
</code></pre></div>"#;

	// -- Exec
	let result = pretty(html, None);

	// -- Check
	assert_eq!(
		result,
		r#"<div>
  <pre>
<code class="language-mermaid">graph TD
    Start --&gt; Review
    Review --&gt; Complete
</code>
  </pre>
</div>"#
	);

	Ok(())
}

#[test]
fn test_pretty_preserves_escaped_html_text() -> Result<()> {
	// -- Setup & Fixtures
	let html = "<div>&lt;strong&gt;Safe &amp; sound&lt;/strong&gt;</div>";

	// -- Exec
	let result = pretty(html, None);

	// -- Check
	assert_eq!(result, html);

	Ok(())
}

#[test]
fn test_pretty_preserves_escaped_html_text_when_wrapping() -> Result<()> {
	// -- Setup & Fixtures
	let html = "<p>&lt;strong&gt;This escaped HTML remains text while its long content is wrapped safely&lt;/strong&gt;</p>";
	let options = PrettyOptions {
		ident: 2,
		wrap: Some(30),
	};

	// -- Exec
	let result = pretty(html, options);

	// -- Check
	assert!(result.contains("&lt;strong&gt;"));
	assert!(result.contains("&lt;/strong&gt;"));
	assert!(!result.contains("<strong>"));

	Ok(())
}
