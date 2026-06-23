fn main() -> Result<(), Box<dyn std::error::Error>> {
	let fx_html = r#"
	<!DOCTYPE html>
	<html>
	<head>
		<meta charset="utf-8">
		<link rel="icon" href="favicon.ico">
	</head>
	<body>
		<p>Content</p>
	</body>
	</html>
	"#;

	let slim = htmlr::slim(fx_html, htmlr::SlimOptions::default().with_indent(2))?;

	println!("Slim (formatted):\n\n{slim}");

	Ok(())
}
