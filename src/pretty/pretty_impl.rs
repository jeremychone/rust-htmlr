use ego_tree::NodeRef;
use html_escape::{encode_double_quoted_attribute, encode_text};
use scraper::{Html, node::Node};

use super::PrettyOptions;

// region:    --- Types

#[derive(Debug)]
enum InlinePart {
	Open(String),
	Close(String),
	Markup(String),
	Text(String),
	Whitespace,
}

// endregion: --- Types

#[doc = include_str!("../../docs/rustdoc/pretty/pretty.md")]
pub fn pretty(html: &str, indent: impl Into<PrettyOptions>) -> String {
	let options = indent.into();
	let is_document = should_parse_document(html);
	let document = if is_document {
		Html::parse_document(html)
	} else {
		Html::parse_fragment(html)
	};
	let indent = " ".repeat(options.ident.into());
	let wrap = options.wrap.map(|width| usize::from(width).max(1));
	let mut output = String::new();

	serialize_node(document.tree.root(), 0, &indent, wrap, is_document, &mut output);

	output
}

fn serialize_node(
	node: NodeRef<Node>,
	depth: usize,
	indent: &str,
	wrap: Option<usize>,
	is_document: bool,
	output: &mut String,
) {
	match node.value() {
		Node::Document | Node::Fragment => {
			for child in node.children() {
				serialize_node(child, depth, indent, wrap, is_document, output);
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
				if is_raw_text_parent(node) {
					output.push_str(text);
				} else {
					output.push_str(&encode_text(text as &str));
				}
			}
		}
		Node::Element(element) => {
			let tag_name = element.name();
			if !is_document && matches!(tag_name, "html" | "head" | "body") {
				for child in node.children() {
					serialize_node(child, depth, indent, wrap, is_document, output);
				}
				return;
			}

			let is_formatting = !indent.is_empty();
			let is_block = is_formatting && (is_block_element(tag_name) || is_head_child(node));
			let is_void = is_void_element(tag_name);

			if is_block {
				if tag_name.eq_ignore_ascii_case("body") && follows_head_element(node) {
					output.push('\n');
				}
				start_block(output, depth, indent);
			}

			output.push('<');
			output.push_str(tag_name);
			for (name, value) in element.attrs() {
				output.push(' ');
				output.push_str(name);
				output.push_str("=\"");
				output.push_str(&encode_attribute_value(value));
				output.push('"');
			}
			output.push('>');

			if is_void {
				return;
			}

			if tag_name.eq_ignore_ascii_case("pre") && has_direct_code_child(node) {
				output.push('\n');
			}

			if let Some(wrap) = options_wrap(indent, wrap, tag_name, node) {
				let mut parts = Vec::new();
				for child in node.children() {
					collect_inline_parts(child, &mut parts);
				}

				if normalized_text_len(&parts) > wrap {
					for line in wrap_inline_parts(parts, wrap) {
						start_block(output, depth + 1, indent);
						output.push_str(&line);
					}
					start_block(output, depth, indent);
					output.push_str("</");
					output.push_str(tag_name);
					output.push('>');
					return;
				}
			}

			let children_start = output.len();
			let child_depth = if is_block { depth + 1 } else { depth };
			for child in node.children() {
				serialize_node(child, child_depth, indent, wrap, is_document, output);
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

fn options_wrap(indent: &str, wrap: Option<usize>, tag_name: &str, node: NodeRef<Node>) -> Option<usize> {
	if indent.is_empty() || !is_text_wrap_element(tag_name) || contains_block_element(node) {
		return None;
	}

	wrap
}

fn collect_inline_parts(node: NodeRef<Node>, parts: &mut Vec<InlinePart>) {
	match node.value() {
		Node::Document | Node::Fragment => {
			for child in node.children() {
				collect_inline_parts(child, parts);
			}
		}
		Node::Doctype(doctype) => {
			parts.push(InlinePart::Markup(format!("<!DOCTYPE {}>", doctype.name)));
		}
		Node::Comment(comment) => {
			let mut value = String::from("<!--");
			value.push_str(comment);
			value.push_str("-->");
			parts.push(InlinePart::Markup(value));
		}
		Node::Text(text) => {
			if is_raw_text_parent(node) {
				parts.push(InlinePart::Markup(text.to_string()));
			} else {
				push_text_parts(text, parts);
			}
		}
		Node::Element(element) => {
			let mut opening = String::from("<");
			opening.push_str(element.name());
			for (name, value) in element.attrs() {
				opening.push(' ');
				opening.push_str(name);
				opening.push_str("=\"");
				opening.push_str(&encode_attribute_value(value));
				opening.push('"');
			}
			opening.push('>');
			parts.push(InlinePart::Open(opening));

			if !is_void_element(element.name()) {
				for child in node.children() {
					collect_inline_parts(child, parts);
				}
				parts.push(InlinePart::Close(format!("</{}>", element.name())));
			}
		}
		Node::ProcessingInstruction(instruction) => {
			let mut value = format!("<?{}", instruction.target);
			if !instruction.is_empty() {
				value.push(' ');
				value.push_str(instruction);
			}
			value.push_str("?>");
			parts.push(InlinePart::Markup(value));
		}
	}
}

fn push_text_parts(text: &str, parts: &mut Vec<InlinePart>) {
	let mut value = String::new();
	let mut in_whitespace = false;

	for character in text.chars() {
		if character.is_whitespace() {
			if !value.is_empty() {
				parts.push(InlinePart::Text(core::mem::take(&mut value)));
			}
			if !in_whitespace {
				parts.push(InlinePart::Whitespace);
				in_whitespace = true;
			}
		} else {
			value.push(character);
			in_whitespace = false;
		}
	}

	if !value.is_empty() {
		parts.push(InlinePart::Text(value));
	}
}

fn normalized_text_len(parts: &[InlinePart]) -> usize {
	let mut length = 0;
	let mut has_text = false;
	let mut pending_space = false;

	for part in parts {
		match part {
			InlinePart::Text(value) => {
				if has_text && pending_space {
					length += 1;
				}
				length += value.chars().count();
				has_text = true;
				pending_space = false;
			}
			InlinePart::Whitespace if has_text => pending_space = true,
			_ => {}
		}
	}

	length
}

fn wrap_inline_parts(parts: Vec<InlinePart>, width: usize) -> Vec<String> {
	let mut lines = Vec::new();
	let mut line = String::new();
	let mut line_width = 0;
	let mut pending_space = false;
	let mut pending_markup = String::new();

	for part in parts {
		match part {
			InlinePart::Whitespace => {
				if line_width > 0 {
					pending_space = true;
				}
			}
			InlinePart::Open(value) if pending_space => pending_markup.push_str(&value),
			InlinePart::Open(value) | InlinePart::Close(value) | InlinePart::Markup(value) => {
				line.push_str(&pending_markup);
				pending_markup.clear();
				line.push_str(&value);
			}
			InlinePart::Text(value) => {
				let value_width = value.chars().count();
				if pending_space && line_width > 0 {
					if line_width + 1 + value_width > width {
						lines.push(core::mem::take(&mut line));
						line_width = 0;
					} else {
						line.push(' ');
						line_width += 1;
					}
				}
				pending_space = false;
				line.push_str(&pending_markup);
				pending_markup.clear();

				for character in value.chars() {
					if line_width == width {
						lines.push(core::mem::take(&mut line));
						line_width = 0;
					}
					line.push_str(&encode_text(character.encode_utf8(&mut [0; 4])));
					line_width += 1;
				}
			}
		}
	}

	line.push_str(&pending_markup);
	if !line.is_empty() {
		lines.push(line);
	}

	lines
}

fn contains_block_element(node: NodeRef<Node>) -> bool {
	node.children().any(|child| match child.value() {
		Node::Element(element) => is_block_element(element.name()) || contains_block_element(child),
		_ => contains_block_element(child),
	})
}

fn has_direct_code_child(node: NodeRef<Node>) -> bool {
	node.children()
		.any(|child| matches!(child.value(), Node::Element(element) if element.name().eq_ignore_ascii_case("code")))
}

fn is_head_child(node: NodeRef<Node>) -> bool {
	matches!(
		node.parent().map(|parent| parent.value()),
		Some(Node::Element(element)) if element.name().eq_ignore_ascii_case("head")
	)
}

fn follows_head_element(node: NodeRef<Node>) -> bool {
	let mut sibling = node.prev_sibling();

	while let Some(previous) = sibling {
		match previous.value() {
			Node::Element(element) => return element.name().eq_ignore_ascii_case("head"),
			Node::Text(text) if text.trim().is_empty() => {}
			_ => return false,
		}
		sibling = previous.prev_sibling();
	}

	false
}

fn is_raw_text_parent(node: NodeRef<Node>) -> bool {
	matches!(
		node.parent().map(|parent| parent.value()),
		Some(Node::Element(element))
			if matches!(
				element.name().to_ascii_lowercase().as_str(),
				"script" | "style" | "xmp" | "iframe" | "noembed" | "noframes" | "plaintext"
			)
	)
}

fn encode_attribute_value(value: &str) -> String {
	encode_double_quoted_attribute(value).replace("&gt;", ">")
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

fn is_text_wrap_element(name: &str) -> bool {
	matches!(
		name.to_ascii_lowercase().as_str(),
		"p" | "li"
			| "dt" | "dd"
			| "figcaption"
			| "caption"
			| "summary"
			| "blockquote"
			| "h1" | "h2"
			| "h3" | "h4"
			| "h5" | "h6"
	)
}

// region:    --- Tests

#[cfg(test)]
#[path = "pretty_impl_tests.rs"]
mod tests;

// endregion: --- Tests
