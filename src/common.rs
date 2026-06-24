/// Decodes HTML entities (e.g., `&lt;` becomes `<`).
/// Re-exporting from the original slimmer or using html-escape directly.
pub fn decode_html_entities(content: &str) -> String {
	html_escape::decode_html_entities(content).to_string()
}
