use super::*;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

#[test]
fn test_md_to_html_basic() -> Result<()> {
	// -- Setup & Fixtures
	let md = "# Title\n\nHello **world** and ~~removed~~.";

	// -- Exec
	let html = md_to_html(md, None)?;

	// -- Check
	assert_eq!(
		html,
		"<h1>Title</h1>\n<p>Hello <strong>world</strong> and <del>removed</del>.</p>\n"
	);

	Ok(())
}
