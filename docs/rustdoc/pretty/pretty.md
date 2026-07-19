# pretty

Formats an HTML document or fragment with block-level indentation and optional text wrapping.

```rust
pub fn pretty(html: &str, options: impl Into<PrettyOptions>) -> String
```

The `options` argument accepts `None`, `Some(PrettyOptions)`, or a direct `PrettyOptions` value. Passing `None` uses the default formatting options.

The input is parsed and serialized as HTML. Documents beginning with a doctype or an `html`, `head`, or `body` element are parsed as complete documents. Other input is parsed as an HTML fragment.

Block-level elements are placed on separate indented lines. Text wrapping applies only to supported text-content elements that do not contain block-level descendants.

Direct child elements of `head`, including `meta`, `title`, and `link`, are placed on separate indented lines.

## Example

```rust
use htmlr::{PrettyOptions, pretty};

let html = "<main><p>This is a short paragraph.</p></main>";

let formatted = pretty(html, PrettyOptions {
    ident: 2,
    wrap: Some(80),
});

assert_eq!(
    formatted,
    "<main>\n  <p>This is a short paragraph.</p>\n</main>"
);
```

Use the defaults by passing `None`:

```rust
use htmlr::pretty;

let formatted = pretty("<div><p>Hello</p></div>", None);
```
