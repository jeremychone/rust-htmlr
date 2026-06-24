# htmlr – High-level HTML helpers

A collection of utilities for cleaning, transforming, and converting HTML content.

## Key Features

- **HTML cleaning** – `slim` removes non-content elements (scripts, styles, comments, empty tags) and filters attributes, leaving a clean HTML document.
- **CSS selection** – `select` queries HTML content with CSS selectors and returns a flat list of `Elem` items.
- **Flexible input** – `select` accepts both raw strings and pre-parsed documents via `HtmlContent` and `HtmlParsed`.
- **Markdown conversion** – `to_md` converts HTML into Markdown using the [htmd](https://crates.io/crates/htmd) library.
- **Error handling** – custom `Error` type and `Result` alias for robust error propagation.

## Quick Start

```rust
use htmlr::{slim, select, Elem};

let html = r#"<html><head><title>Demo</title></head>
    <body><p>Hello</p><a href="url">link</a></body></html>"#;

let clean = slim(html, SlimOptions::default())?;
let items: Vec<Elem> = select(&clean, ["p", "a"])?;

assert_eq!(items.len(), 2);
```

## Crate Status

Early development. Follows SemVer. See the [repository](https://github.com/jeremychone/rust-htmlr) for updates.
