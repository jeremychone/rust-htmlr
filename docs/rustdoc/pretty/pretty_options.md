# PrettyOptions

Configuration for the [`pretty`](crate::pretty) HTML formatter.

```rust
pub struct PrettyOptions {
    pub ident: u8,
    pub wrap: Option<u16>,
}
```

## Fields

- `ident` controls the number of spaces used for each indentation level.
- `wrap` sets the maximum text-content line length. Set it to `None` to disable wrapping.

The default configuration uses two spaces per indentation level and wraps supported text content at 80 characters.

Wrapping applies to paragraphs, list items, description terms and details, figure captions, table captions, summaries, block quotes, and heading elements. Elements containing block-level descendants are not text-wrapped.

## Example

```rust
use htmlr::{PrettyOptions, pretty};

let options = PrettyOptions {
    ident: 4,
    wrap: None,
};

let formatted = pretty("<section><p>Hello</p></section>", options);

assert_eq!(
    formatted,
    "<section>\n    <p>Hello</p>\n</section>"
);
```
