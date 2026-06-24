use crate::Result;
use html_escape::encode_double_quoted_attribute;
use scraper::ElementRef;

// region:    --- Constants

pub(super) const TAGS_TO_REMOVE: &[&str] = &["script", "link", "style", "svg", "base"];

pub(super) const REMOVABLE_EMPTY_TAGS: &[&str] = &[
	"div", "span", "p", "i", "b", "em", "strong", "section", "article", "header", "footer", "nav", "aside",
];

pub(super) const META_PROPERTY_KEYWORDS: &[&str] = &["title", "url", "image", "description"];

pub(super) const ALLOWED_META_ATTRS: &[&str] = &["property", "content"];

pub(super) const ALLOWED_BODY_ATTRS: &[&str] = &["class", "aria-label", "href", "title", "id"];

#[rustfmt::skip]
pub(super) const BLOCK_LEVEL_TAGS: &[&str] = &[
	"html", "head", "body", "a", "div", "p", "section", "article", "header", "footer", "nav", "aside",
	"ul", "ol", "li", "table", "tr", "td", "th", "h1", "h2", "h3", "h4", "h5", "h6",
	"pre", "blockquote", "main", "form", "fieldset", "details", "summary", "figure", "figcaption",
	"dl", "dt", "dd", "br", "hr", "img", "input", "meta", "link", "script", "style",
];

#[rustfmt::skip]
pub(super) const VOID_ELEMENTS: &[&str] = &[
	"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta",
	"param", "source", "track", "wbr",
];

// endregion: --- Constants

pub(super) fn remove_empty_lines(content: String) -> Result<String> {
	let lines: Vec<&str> = content.lines().filter(|line| !line.trim().is_empty()).collect();
	Ok(lines.join("\n"))
}

pub(super) fn is_string_effectively_empty(s: &str) -> bool {
	s.trim().is_empty()
}

pub(super) fn should_keep_meta(element: ElementRef) -> bool {
	// Check if the element is actually a <meta> tag
	if element.value().name() != "meta" {
		return false;
	}

	if let Some(prop_value) = element.value().attr("property") {
		let value_lower = prop_value.to_lowercase();
		// Check if the property value contains any of the relevant keywords
		META_PROPERTY_KEYWORDS.iter().any(|&keyword| value_lower.contains(keyword))
	} else {
		// No 'property' attribute found
		false
	}
}

pub(super) fn filter_and_write_attributes(
	element: ElementRef,
	is_in_head_context: bool,
	output: &mut String,
) -> Result<()> {
	let tag_name = element.value().name();

	// Determine the correct list of allowed attributes based on context
	let allowed_attrs: &[&str] = if is_in_head_context {
		match tag_name {
			"meta" => ALLOWED_META_ATTRS,
			"title" => &[], // No attributes allowed on title
			_ => &[],       // Default deny for other unexpected tags in head
		}
	} else {
		// Outside head context
		ALLOWED_BODY_ATTRS
	};

	// Iterate over attributes and append allowed ones
	for (name, value) in element.value().attrs() {
		// Check against the determined allowlist
		if allowed_attrs.contains(&name) {
			output.push(' ');
			output.push_str(name);
			output.push_str("=\"");
			// Encode attribute value correctly
			output.push_str(&encode_double_quoted_attribute(value));
			output.push('"');
		}
	}

	Ok(())
}
