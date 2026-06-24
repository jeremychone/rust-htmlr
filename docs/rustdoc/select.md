# select – CSS selector query

```rust
pub fn select<'a, S>(
    html_content: impl Into<HtmlContent<'a>>,
    selectors: S
) -> Result<Vec<Elem>>
where
    S: IntoIterator,
    S::Item: AsRef<str>,
```

Returns a list of [`Elem`] items matching any of the given CSS selectors (combined with OR).

- Empty selectors are silently ignored.
- Returns an empty vector when no valid selectors remain.
- Powered by the [scraper](https://crates.io/crates/scraper) crate.

## Example

```rust
let html = r#"<html><body><h1>Title</h1><p class="intro">Hello</p><a href="url">link</a></body></html>"#;

// From a raw string
let elements = select(&html, ["p.intro", "a"])?;
for e in &elements {
    println!("Tag: {}, Text: {:?}", e.tag, e.text);
}

// From a pre-parsed document (reusable across multiple calls)
let doc = htmlr::HtmlParsed::parse_document(html);
let headings = select(&doc, ["h1", "h2"])?;
let links = select(&doc, ["a"])?;
```
