# htmlr - Public API Reference


## Functions

### pretty

```rust
pub fn pretty(html: &str, options: impl Into<PrettyOptions>) -> String
```

- Parses and serializes HTML with block-level indentation.
- Places direct child elements of `head` on separate indented lines.
- Wraps long text in configured text-content elements when wrapping is enabled.
- Accepts `None`, `Some(PrettyOptions)`, or a direct `PrettyOptions` value.

### slim

```rust
pub fn slim(html_content: &str, options: impl Into<SlimOptions>) -> Result<String>
```

- Removes `<script>`, `<link>`, `<style>`, `<svg>`, `<base>`, HTML comments, empty text nodes, and elements that become empty after processing children (e.g., `<div>`, `<span>`, `<p>`).
- Drops empty `<head>`. Keeps `<title>` and `<meta>` whose `property` contains "title", "url", "image", or "description".
- Filters attributes: outside `<head>`, keeps `class`, `aria-label`, `href`, `title`, `id`; inside `<head>`, keeps only `property`/`content` on meta.
- Returns cleaned HTML `String`.

### select

```rust
pub fn select<'a, S>(html_content: impl Into<HtmlContent<'a>>, selectors: S) -> Result<Vec<Elem>>
where
    S: IntoIterator,
    S::Item: AsRef<str>,
```

- Accepts `&str`, `String`, or `HtmlParsed` as HTML source.
- Joins selectors with `OR`. Empty selectors ignored; returns empty `Vec` if none valid.
- Returns matched `Elem` items in document order.

### to_md

```rust
pub fn to_md(html_content: &str) -> String
```

- Converts HTML to Markdown via the `htmd` crate.
- No error return.

### decode_html_entities

```rust
pub fn decode_html_entities(content: &str) -> String
```

- Decodes HTML entities (`&amp;`, `&lt;`, `&gt;`, `&#...;`, etc.) to characters.

## Types

### Elem

```rust
pub struct Elem {
    pub tag: String,
    pub attrs: Option<HashMap<String, String>>,
    pub text: Option<String>,
    pub inner_html: Option<String>,
}
```

- `tag`: lowercase tag name.
- `attrs`: attribute map, `None` if no attributes.
- `text`: aggregated visible text from descendants, `None` if empty.
- `inner_html`: raw inner HTML, `None` if empty.

### Error

```rust
pub enum Error {
    Custom(String),
    SelectorParse { selector: String, cause: String },
}
```

- `Custom`: generic processing error.
- `SelectorParse`: invalid CSS selector; `selector` field holds the failing selector, `cause` the reason.

### Result

```rust
pub type Result<T> = Result<T, Error>;
```

Type alias for `std::result::Result` with crate's `Error`.

### SlimOptions

```rust
pub struct SlimOptions { /* fields */ }
```

- Builder for `slim` options.
- `SlimOptions::default()` — returns default options.
- `SlimOptions::from_indent(indent: usize) -> Self` — sets indentation.
- `fn with_indent(self, indent: usize) -> Self`
- `fn with_preserve_images(self, preserve: bool) -> Self`

### PrettyOptions

```rust
pub struct PrettyOptions {
    pub ident: u8,
    pub wrap: Option<u16>,
}
```

- `ident`: spaces per indentation level, defaults to `2`.
- `wrap`: maximum text-content line length, defaults to `Some(80)`. Set to `None` to disable wrapping.
- Wrapping applies to `p`, `li`, `dt`, `dd`, `figcaption`, `caption`, `summary`, `blockquote`, and heading elements.
- Elements with block-level descendants are not text-wrapped.

### HtmlContent

```rust
pub enum HtmlContent<'a> {
    Str(&'a str),
    Parsed(HtmlParsed),
}
```

- Represents acceptable HTML source for `select`.
- `From` impls: `&str`, `String`, `HtmlParsed` via `Into`.

### HtmlParsed

```rust
pub struct HtmlParsed { /* private */ }
```

- Pre-parsed HTML document for repeated queries.
- `HtmlParsed::parse_document(html: &str) -> Self`

