# to_md – HTML to Markdown conversion

```rust
pub fn to_md(html_content: &str, options: impl Into<ToMdOptions>) -> Result<String>
```

Converts an HTML string into Markdown. Uses the [htmd](https://crates.io/crates/htmd) library under the hood.

`html_content`: The raw HTML content to convert.

## Options

`options`: Optional conversion configuration (implements `Into<ToMdOptions>`). When `None` is passed, the default options are applied, which correspond to:
- `bullet_list_marker` set to `Dash` (list items start with `-`),
- `ul_bullet_spacing` set to `1`,
- `ol_number_spacing` set to `1`,
- all other htmd options remain at their library defaults.

## Example

```rust
use re_doc::to_md;

// Use default options (None)
let md = to_md("<h1>Hello</h1>", None).unwrap();

// Use custom bullet marker
let custom = htmd::Options {
    bullet_list_marker: htmd::options::BulletListMarker::Asterisk,
    ..Default::default()
};
let md = to_md("<ul><li>Item</li></ul>", custom).unwrap();
```

This function is typically used after cleaning HTML with `slim` to produce readable Markdown from web content.
