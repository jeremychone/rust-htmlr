# select – CSS selector query

```rust
pub fn select<S>(html_content: &str, selectors: S) -> Result<Vec<Elem>>
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
let elements = select(&html, ["p.intro", "a.link"])?;
for e in elements {
    println!("Tag: {}, Text: {:?}", e.tag, e.text);
}
```
