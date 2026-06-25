# htmlr - A html helpers crate

**A collection of high-level utilities for cleaning, transforming, and converting HTML content.**

Bring high level html apis such as
- `slim(html)`
- `select(html, selectors)`
- `decode_html_entities(content)`

> ⚠️ Very early release, slow loc version as `htmlr = "=0.1.2"`
> Will follow semver for API update after `0.2.x` and above

## Example

```rust
let content: String = /* full HTML page */;

let slim_content = htmlr::slim(&content, SlimOptions::default().with_indent(2))?;
```

## Origins

This crate, `htmlr v0.1.0`, is the continuation of [html-helpers](https://crates.io/crates/html-helpers) (v0.2.2). The crate was renamed because the original name felt too long.

This crate is used in [aipack](https://aipack.ai), and [aiprog](https://crates.io/crates/aiprog) which will be the core AI Program engine for [zcoder](https://zcoder.run) (Parallel first coding harness)

---

[This repo](https://github.com/jeremychone/rust-htmlr)
