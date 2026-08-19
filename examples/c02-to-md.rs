fn main() -> Result<(), Box<dyn std::error::Error>> {
	use htmlr::to_md;
	use std::fs;

	let url = "https://en.wikipedia.org/wiki/Moon";
	println!("Fetching {url}...");

	let client = reqwest::blocking::Client::builder()
		.user_agent("htmlr-example/0.1 (https://github.com/jeremychone/rust-htmlr)")
		.build()?;

	let html = client.get(url).send()?.text()?;

	let out_dir = "examples/.out";
	let html_path = "examples/.out/c02-to-md.html";
	let md_path = "examples/.out/c02-to-md.md";
	fs::create_dir_all(out_dir)?;

	fs::write(html_path, &html)?;
	let html_len = html.len();
	println!("Saved html to {html_path} ({html_len} bytes)");

	let md = to_md(&html, None)?;
	fs::write(md_path, &md)?;

	let md_len = md.len();
	println!("Saved markdown to {md_path} ({md_len} bytes)");

	Ok(())
}
