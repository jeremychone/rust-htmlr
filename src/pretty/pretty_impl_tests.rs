type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

use super::*;

#[test]
fn test_pretty_options_default() -> Result<()> {
	// -- Setup & Fixtures
	let options = PrettyOptions::from(None);

	// -- Exec
	let indent = options.ident;

	// -- Check
	assert_eq!(indent, 2);

	Ok(())
}

#[test]
fn test_pretty_option_forms() -> Result<()> {
	// -- Setup & Fixtures
	let html = "<div><p>Hello</p></div>";

	// -- Exec
	let direct = pretty(html, PrettyOptions { ident: 4 });
	let optional = pretty(html, Some(PrettyOptions { ident: 4 }));
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
	let result = pretty(html, PrettyOptions { ident: 4 });

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
