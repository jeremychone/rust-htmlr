use super::MdToHtmlOptions;
use crate::Result;
use html_escape::{encode_double_quoted_attribute, encode_text};
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};

#[doc = include_str!("../../docs/rustdoc/to_html/md_to_html.md")]
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
			custom_code_block_is_mermaid = Some(is_mermaid && opts.code_block_mermaid_as_pre);
			Event::Html(code_block_opening(&kind, is_mermaid && opts.code_block_mermaid_as_pre))
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
	let events = parser.collect::<Vec<_>>();
	let mut html_events = Vec::with_capacity(events.len());
	let mut events = events.into_iter();
	while let Some(event) = events.next() {
		if let Event::Start(Tag::Heading { level, .. }) = event {
			let mut heading_events = Vec::new();
			for event in events.by_ref() {
				if matches!(event, Event::End(TagEnd::Heading(_))) {
					break;
				}
				heading_events.push(event);
			}

			let id = heading_id(&heading_events);
			html_events.push(Event::Html(heading_opening(level, &id)));
			html_events.extend(heading_events);
			html_events.push(Event::Html(heading_closing(level)));
		} else {
			html_events.push(event);
		}
	}
	let mut html_output = String::new();
	html::push_html(&mut html_output, html_events.into_iter());

	Ok(html_output)
}

// region:    --- Support

fn code_block_opening(kind: &CodeBlockKind<'_>, is_mermaid: bool) -> pulldown_cmark::CowStr<'static> {
	if is_mermaid {
		return "<pre class=\"mermaid\">".into();
	}

	let opening = match kind {
		CodeBlockKind::Indented => "<pre>\n<code>".to_string(),
		CodeBlockKind::Fenced(info) => {
			if let Some(language) = info.split_whitespace().next() {
				let language = encode_double_quoted_attribute(language);
				format!("<pre>\n<code class=\"language-{language}\">")
			} else {
				"<pre>\n<code>".to_string()
			}
		}
	};
	opening.into()
}

fn heading_id(events: &[Event<'_>]) -> String {
	let mut id = String::new();
	let mut needs_separator = false;

	for event in events {
		let content = match event {
			Event::Text(content) | Event::Code(content) => content,
			_ => continue,
		};
		for character in content.chars() {
			if character.is_alphanumeric() {
				if needs_separator && !id.is_empty() {
					id.push('-');
				}
				for lowercase_character in character.to_lowercase() {
					if lowercase_character.is_alphanumeric() {
						id.push(lowercase_character);
					}
				}
				needs_separator = false;
			} else {
				needs_separator = true;
			}
		}
	}

	id
}

fn heading_opening(level: HeadingLevel, id: &str) -> pulldown_cmark::CowStr<'static> {
	let tag = heading_tag(level);
	let id = encode_double_quoted_attribute(id);
	format!("<{tag} id=\"{id}\">").into()
}

fn heading_closing(level: HeadingLevel) -> pulldown_cmark::CowStr<'static> {
	let tag = heading_tag(level);
	format!("</{tag}>\n").into()
}

fn heading_tag(level: HeadingLevel) -> &'static str {
	match level {
		HeadingLevel::H1 => "h1",
		HeadingLevel::H2 => "h2",
		HeadingLevel::H3 => "h3",
		HeadingLevel::H4 => "h4",
		HeadingLevel::H5 => "h5",
		HeadingLevel::H6 => "h6",
	}
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "md_to_html_impl_tests.rs"]
mod md_to_html_impl_tests;

// endregion: --- Tests
