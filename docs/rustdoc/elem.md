# Elem – Simplified HTML element

Represents a simplified HTML element extracted via `select`.

## Fields

- `tag: String` – element tag name in lowercase (e.g., `"a"`, `"p"`).
- `attrs: Option<HashMap<String, String>>` – key-value attribute map, or `None` when the element has no attributes.
- `text: Option<String>` – visible text content collected from all descendants, or `None` if effectively empty.
- `inner_html: Option<String>` – raw inner HTML of the element, or `None` if effectively empty.

## Example

```rust
let items: Vec<Elem> = select(html, ["a"])?;
for item in items {
    if let Some(ref attrs) = item.attrs {
        if let Some(href) = attrs.get("href") {
            println!("Link: {}", href);
        }
    }
}
```
