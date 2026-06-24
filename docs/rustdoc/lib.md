# htmlr – High-level HTML helpers

A collection of utilities for cleaning, transforming, and converting HTML content.

## Quick Overview

(examples/c01-simple.rs)
```rust

fn main() -> Result<(), Box<dyn std::error::Error>> {
	use htmlr::{Elem, SlimOptions, decode_html_entities, select, slim, to_md};

	let html = r#"<html><head></head><body><p>Hello &amp; welcome!</p>
    <a class="blink" style="background: red" href="https://example.com">good&nbsp;link</a><script>some_stuff()</script></body></html>"#;

	// clean and slim
	let slimmed = slim(html, SlimOptions::from_indent(2))?;
	println!("=== Slimmed version `slim(...)`:\n{slimmed}");

	// select all of the `<a>` tags
	let elems: Vec<Elem> = select(&slimmed, ["a"])?;
	println!("\n=== <a> count `select(...)`: {}", elems.len());

	// into markdown
	let markdown = to_md(&slimmed)?;
	println!("\n=== Markdown `to_md(...)`:\n{markdown}");

	// decoding a html text
	let txt = "good&nbsp;link";
	let decoded = decode_html_entities(txt);
	println!("\n=== decode  `decode_html_entities(\"{txt}\")`:\n{decoded}");

	Ok(())
}
```

Will output: 

```
=== Slimmed version `slim(...)`:
<body>
  <p>Hello & welcome!</p>
  <a class="blink" href="https://example.com">good link</a>
</body>

=== <a> count `select(...)`: 1

=== Markdown `to_md(...)`:
Hello & welcome!

[good link](https://example.com)

=== decode  `decode_html_entities("good&nbsp;link")`:
good link
```

## Key Features

- **HTML cleaning** – `slim` strips non-content elements and filters attributes.
- **CSS selection** – `select` extracts elements by CSS selector.
- **Markdown conversion** – `to_md` turns HTML into Markdown.
- **HTML entity decoding** – `decode_html_entities` unescapes HTML entities.
- **Error handling** – custom `Error` type and `Result` alias.

## Crate Status

Early development. Follows SemVer. See the [repository](https://github.com/jeremychone/rust-htmlr) for updates.
