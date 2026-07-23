use super::*;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

#[test]
fn test_md_to_html_basic() -> Result<()> {
	// -- Setup & Fixtures
	let md = r#"# Title

Hello **world** and ~~removed~~."#;

	// -- Exec
	let html = md_to_html(md, None)?;

	// -- Check
	assert_eq!(
		html,
		r#"<h1>Title</h1>
<p>Hello <strong>world</strong> and <del>removed</del>.</p>
"#
	);

	Ok(())
}

#[test]
fn test_md_to_html_code_block_escapes_content_by_default() -> Result<()> {
	// -- Setup & Fixtures
	let md = r#"```html
<div data-value="a&b">content</div>
```"#;

	// -- Exec
	let html = md_to_html(md, None)?;

	// -- Check
	assert_eq!(
		html,
		r#"<pre>
<code class="language-html">
&lt;div data-value="a&amp;b"&gt;content&lt;/div&gt;
</code>
</pre>
"#
	);

	Ok(())
}

#[test]
fn test_md_to_html_code_block_escapes_content_when_enabled() -> Result<()> {
	// -- Setup & Fixtures
	let md = r#"```html
<strong>Hello</strong> World

```"#;

	// -- Exec
	let html = md_to_html(md, None)?;

	// -- Check
	assert_eq!(
		html,
		r#"<pre>
<code class="language-html">
&lt;strong&gt;Hello&lt;/strong&gt; World

</code>
</pre>
"#
	);

	Ok(())
}

#[test]
fn test_md_to_html_code_block_allows_unescaped_content() -> Result<()> {
	// -- Setup & Fixtures
	let md = r#"```html
<div>content</div>
```"#;
	let options = MdToHtmlOptions {
		code_block_html_escape_content: false,
		..Default::default()
	};

	// -- Exec
	let html = md_to_html(md, options)?;

	// -- Check
	assert_eq!(
		html,
		r#"<pre>
<code class="language-html">
<div>content</div>
</code>
</pre>
"#
	);

	Ok(())
}

#[test]
fn test_md_to_html_mermaid_as_pre_follows_content_escape_option() -> Result<()> {
	// -- Setup & Fixtures
	let md = r#"```mermaid
graph TD
A[One] --> B[Two & Three]
```"#;

	// -- Exec
	let escaped_html = md_to_html(md, None)?;
	let unescaped_html = md_to_html(
		md,
		MdToHtmlOptions {
			code_block_html_escape_content: false,
			..Default::default()
		},
	)?;

	// -- Check
	assert_eq!(
		escaped_html,
		r#"<pre class="mermaid">
graph TD
A[One] --&gt; B[Two &amp; Three]
</pre>
"#
	);
	assert_eq!(
		unescaped_html,
		r#"<pre class="mermaid">
graph TD
A[One] --> B[Two & Three]
</pre>
"#
	);

	Ok(())
}

#[test]
fn test_md_to_html_unescaped_code_block_is_wrapped_in_pre_and_code() -> Result<()> {
	// -- Setup & Fixtures
	let md = r#"```html
<strong>Hello</strong> World

```"#;
	let options = MdToHtmlOptions {
		code_block_html_escape_content: false,
		..Default::default()
	};

	// -- Exec
	let html = md_to_html(md, options)?;

	// -- Check
	assert_eq!(
		html,
		r#"<pre>
<code class="language-html">
<strong>Hello</strong> World

</code>
</pre>
"#
	);

	Ok(())
}
