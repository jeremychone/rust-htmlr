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
	let markdown = to_md(&slimmed, None)?;
	println!("\n=== Markdown `to_md(...)`:\n{markdown}");

	// decoding a html text
	let txt = "good&nbsp;link";
	let decoded = decode_html_entities(txt);
	println!("\n=== decode  `decode_html_entities(\"{txt}\")`:\n{decoded}");

	Ok(())
}
