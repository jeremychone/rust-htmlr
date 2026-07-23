use super::MdToHtmlOptions;
use crate::Result;
use html_escape::{encode_double_quoted_attribute, encode_text};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd, html};

pub fn md_to_html(md: &str, options: impl Into<MdToHtmlOptions>) -> Result<String> {
	let opts = options.into();

	let mut parser_options = Options::empty();
	parser_options.insert(Options::ENABLE_TABLES);
	parser_options.insert(Options::ENABLE_FOOTNOTES);
	parser_options.insert(Options::ENABLE_STRIKETHROUGH);
	parser_options.insert(Options::ENABLE_TASKLISTS);
	parser_options.insert(Options::ENABLE_SMART_PUNCTUATION);

	let parser = Parser::new_ext(md, parser_options);
	let mut custom_code_block_is_mermaid = None;
	let parser = parser.map(move |event| match event {
		Event::Start(Tag::CodeBlock(kind)) => {
			let is_mermaid = matches!(
				&kind,
				CodeBlockKind::Fenced(info)
					if info.split_whitespace().next() == Some("mermaid")
			);
			let customize =
				!opts.code_block_html_escape_content
					|| (is_mermaid && opts.code_block_mermaid_as_pre);

			if customize {
				custom_code_block_is_mermaid =
					Some(is_mermaid && opts.code_block_mermaid_as_pre);
				Event::Html(code_block_opening(&kind, is_mermaid && opts.code_block_mermaid_as_pre))
			} else {
				Event::Start(Tag::CodeBlock(kind))
			}
		}
		Event::Text(content) if custom_code_block_is_mermaid.is_some() => {
			let content = if opts.code_block_html_escape_content {
				encode_text(&content).into_owned()
			} else {
				content.into_string()
			};
			Event::Html(content.into())
		}
		Event::End(TagEnd::CodeBlock) => {
			if let Some(is_mermaid) = custom_code_block_is_mermaid.take() {
				Event::Html(if is_mermaid {
					"</pre>\n".into()
				} else {
					"</code></pre>\n".into()
				})
			} else {
				Event::End(TagEnd::CodeBlock)
			}
		}
		event => event,
	});
	let mut html_output = String::new();
	html::push_html(&mut html_output, parser);

	Ok(html_output)
}

// region:    --- Support

fn code_block_opening(kind: &CodeBlockKind<'_>, is_mermaid: bool) -> pulldown_cmark::CowStr<'static> {
	if is_mermaid {
		return "<pre class=\"mermaid\">\n".into();
	}

	let opening = match kind {
		CodeBlockKind::Indented => "<pre><code>".to_string(),
		CodeBlockKind::Fenced(info) => {
			if let Some(language) = info.split_whitespace().next() {
				let language = encode_double_quoted_attribute(language);
				format!("<pre><code class=\"language-{language}\">")
			} else {
				"<pre><code>".to_string()
			}
		}
	};
	opening.into()
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "md_to_html_impl_tests.rs"]
mod md_to_html_impl_tests;

// endregion: --- Tests
