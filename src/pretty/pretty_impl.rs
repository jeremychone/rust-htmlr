use ego_tree::NodeRef;
use html_escape::encode_double_quoted_attribute;
use scraper::{Html, node::Node};

// region:    --- Types

/// Options for the [`pretty`] function.
#[derive(Clone, Copy, Debug)]
pub struct PrettyOptions {
	/// Number of spaces per indentation level.
	pub ident: u8,
}

// endregion: --- Types

pub fn pretty(html: &str, indent: impl Into<PrettyOptions>) -> String {
	let options = indent.into();
	let document = if should_parse_document(html) {
		Html::parse_document(html)
	} else {
		Html::parse_fragment(html)
	};
	let indent = " ".repeat(options.ident.into());
	let mut output = String::new();

	serialize_node(document.tree.root(), 0, &indent, &mut output);

	output
}

fn serialize_node(node: NodeRef<Node>, depth: usize, indent: &str, output: &mut String) {
	match node.value() {
		Node::Document | Node::Fragment => {
			for child in node.children() {
				serialize_node(child, depth, indent, output);
			}
		}
		Node::Doctype(doctype) => {
			start_block(output, depth, indent);
			output.push_str("<!DOCTYPE ");
			output.push_str(&doctype.name);

			if !doctype.public_id.is_empty() {
				output.push_str(" PUBLIC \"");
				output.push_str(&doctype.public_id);
				output.push('"');
			}

			if !doctype.system_id.is_empty() {
				if doctype.public_id.is_empty() {
					output.push_str(" SYSTEM");
				}
				output.push_str(" \"");
				output.push_str(&doctype.system_id);
				output.push('"');
			}

			output.push('>');
		}
		Node::Comment(comment) => {
			output.push_str("<!--");
			output.push_str(comment);
			output.push_str("-->");
		}
		Node::Text(text) => {
			if !text.trim().is_empty() {
				output.push_str(text);
			}
		}
		Node::Element(element) => {
			let tag_name = element.name();
			let is_formatting = !indent.is_empty();
			let is_block = is_formatting && is_block_element(tag_name);
			let is_void = is_void_element(tag_name);

			if is_block {
				start_block(output, depth, indent);
			}

			output.push('<');
			output.push_str(tag_name);
			for (name, value) in element.attrs() {
				output.push(' ');
				output.push_str(name);
				output.push_str("=\"");
				output.push_str(&encode_double_quoted_attribute(value));
				output.push('"');
			}
			output.push('>');

			if is_void {
				return;
			}

			let children_start = output.len();
			let child_depth = if is_block { depth + 1 } else { depth };
			for child in node.children() {
				serialize_node(child, child_depth, indent, output);
			}

			if is_block && output[children_start..].contains('\n') {
				start_block(output, depth, indent);
			}

			output.push_str("</");
			output.push_str(tag_name);
			output.push('>');
		}
		Node::ProcessingInstruction(instruction) => {
			output.push_str("<?");
			output.push_str(&instruction.target);
			if !instruction.is_empty() {
				output.push(' ');
				output.push_str(instruction);
			}
			output.push_str("?>");
		}
	}
}

fn start_block(output: &mut String, depth: usize, indent: &str) {
	if !output.is_empty() {
		output.push('\n');
	}
	output.push_str(&indent.repeat(depth));
}

fn should_parse_document(html: &str) -> bool {
	let html = html.trim_start().to_ascii_lowercase();

	html.starts_with("<!doctype") || html.starts_with("<html") || html.starts_with("<head") || html.starts_with("<body")
}

fn is_block_element(name: &str) -> bool {
	matches!(
		name.to_ascii_lowercase().as_str(),
		"address"
			| "article"
			| "aside" | "blockquote"
			| "body" | "caption"
			| "col" | "colgroup"
			| "dd" | "details"
			| "dialog"
			| "div" | "dl"
			| "dt" | "fieldset"
			| "figcaption"
			| "figure"
			| "footer"
			| "form" | "h1"
			| "h2" | "h3"
			| "h4" | "h5"
			| "h6" | "head"
			| "header"
			| "hgroup"
			| "hr" | "html"
			| "li" | "main"
			| "menu" | "nav"
			| "ol" | "p"
			| "pre" | "script"
			| "section"
			| "style" | "summary"
			| "table" | "tbody"
			| "td" | "tfoot"
			| "th" | "thead"
			| "tr" | "ul"
	)
}

fn is_void_element(name: &str) -> bool {
	matches!(
		name.to_ascii_lowercase().as_str(),
		"area"
			| "base" | "br"
			| "col" | "embed"
			| "hr" | "img"
			| "input" | "link"
			| "meta" | "param"
			| "source"
			| "track" | "wbr"
	)
}

impl Default for PrettyOptions {
	fn default() -> Self {
		Self { ident: 2 }
	}
}

// region:    --- Froms

impl From<Option<PrettyOptions>> for PrettyOptions {
	fn from(options: Option<PrettyOptions>) -> Self {
		options.unwrap_or_default()
	}
}

// endregion: --- Froms

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pretty_defaults_to_two_spaces() {
		let options = PrettyOptions::from(None);

		assert_eq!(options.ident, 2);
	}

	#[test]
	fn pretty_accepts_all_option_forms() {
		let html = "<div><p>Hello</p></div>";
		let direct = pretty(html, PrettyOptions { ident: 4 });
		let optional = pretty(html, Some(PrettyOptions { ident: 4 }));
		let defaulted = pretty(html, None);

		assert_eq!(direct, optional);
		assert!(defaulted.contains("<p>Hello</p>"));
	}

	#[test]
	fn pretty_indents_block_elements_without_slimming() {
		let html = "<div><p>Hello <strong>there</strong></p><section><br></section></div>";

		assert_eq!(
			pretty(html, PrettyOptions { ident: 4 }),
			"<div>\n    <p>Hello <strong>there</strong></p>\n    <section>\n        <br>\n    </section>\n</div>"
		);
	}

	#[test]
	fn pretty_preserves_tag_attributes_and_raw_element_content() {
		let html = r#"<div data-value="a > b"><script>if (a < b) { call(">"); }</script></div>"#;

		assert_eq!(
			pretty(html, None),
			"<div data-value=\"a > b\">\n  <script>if (a < b) { call(\">\"); }</script>\n</div>"
		);
	}

	#[test]
	fn pretty_formats_the_normalized_dom() {
		let html = "<div><p>Hello<section>World</section></div>";

		assert_eq!(
			pretty(html, None),
			"<div>\n  <p>Hello</p>\n  <section>World</section>\n</div>"
		);
	}

	#[test]
	fn pretty_closes_custom_elements() {
		let html = "<div><my-tag><p>Hello</p></my-tag></div>";

		assert_eq!(pretty(html, None), "<div><my-tag>\n  <p>Hello</p></my-tag></div>");
	}
}

// endregion: --- Tests
