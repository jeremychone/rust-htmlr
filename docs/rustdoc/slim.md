# slim – HTML content cleaner

```rust
pub fn slim(html_content: &str, options: impl Into<SlimOptions>) -> Result<String>
```

Slims an HTML page by removing non-content elements and filtering attributes, preserving only essential head tags and body content.

Removes:
- `<script>`, `<link>`, `<style>`, `<svg>`, `<base>`, HTML comments, empty whitespace text nodes.
- Specific tags (`div`, `span`, `p`, etc.) that become effectively empty after processing children.

Head preservation:
- Drops empty `<head>`.
- Keeps `<title>` and `<meta>` tags whose `property` attribute contains “title”, “url”, “image”, or “description”.

Attribute filtering:
- Outside `<head>` keeps `class`, `aria-label`, `href`, `title`, `id`.
- Inside `<head>` keeps only `property`/`content` on `<meta>` tags.

## Options

`SlimOptions` supports customization:

```rust
let options = SlimOptions::default()
    .with_indent(2)
    .with_preserve_images(true);
```

See the function documentation for the full list of options.
