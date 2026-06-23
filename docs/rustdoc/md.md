# to_md – HTML to Markdown conversion

```rust
pub fn to_md(html_content: &str) -> String
```

Converts an HTML string into Markdown. Uses the [htmd](https://crates.io/crates/htmd) library under the hood.

This function is typically used after cleaning HTML with `slim` to produce readable Markdown from web content.
