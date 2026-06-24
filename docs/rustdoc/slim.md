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

`SlimOptions` allows controlling output indentation with the following fields:

- `indent`: An `Option<u8>` specifying the number of spaces per indentation level, or `None` for flat output (no indentation).
- `indent_with_tabs`: A `bool` indicating whether to use tabs instead of spaces.

When `None` is passed as the options argument, the default `SlimOptions` are used: `indent` is `None` (flat output) and `indent_with_tabs` is `false`.

### Example

```rust
use re_doc::slim;

// Use default options (flat, no indentation)
let html = slim("<div>hello</div>", None).unwrap();

// Use 2-space indentation
let options = SlimOptions::default().with_indent(2);
let html = slim("<div>hello</div>", options).unwrap();
```

See the function documentation for more details.
