# `md_to_html`

Converts Markdown into HTML using `pulldown-cmark`.

```rust
pub fn md_to_html(
    md: &str,
    options: impl Into<MdToHtmlOptions>,
) -> Result<String>
```

The conversion enables tables, footnotes, strikethrough, task lists, and smart punctuation.

Pass an `MdToHtmlOptions` value to configure code block rendering. Passing `None` uses [`MdToHtmlOptions::default`].

## Code blocks

Ordinary fenced code blocks are rendered as `<pre><code>` elements. When a fence specifies a language, the `<code>` element receives a `language-<language>` CSS class.

By default, Mermaid fenced code blocks are rendered as:

```html
<pre class="mermaid">
...
</pre>
```

Set [`MdToHtmlOptions::code_block_mermaid_as_pre`] to `false` to render Mermaid blocks as ordinary code blocks instead.

Code block contents are HTML-escaped by default. Set [`MdToHtmlOptions::code_block_html_escape_content`] to `false` when the code block content must be emitted without escaping.

## Errors

The current conversion implementation returns `Ok` for all inputs. The [`Result`] return type leaves room for fallible conversion behavior in future releases.

[`MdToHtmlOptions::default`]: crate::MdToHtmlOptions::default
[`MdToHtmlOptions::code_block_mermaid_as_pre`]: crate::MdToHtmlOptions::code_block_mermaid_as_pre
[`MdToHtmlOptions::code_block_html_escape_content`]: crate::MdToHtmlOptions::code_block_html_escape_content
[`Result`]: crate::Result
