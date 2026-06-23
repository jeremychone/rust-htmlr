# Error and Result

Custom error types used throughout the crate.

## `Error`

```rust
pub enum Error {
    Custom(String),
    SelectorParse { selector: String, cause: String },
}
```

- `Custom` – generic internal processing error.
- `SelectorParse` – invalid CSS selector syntax; contains the offending selector and a descriptive cause.

## `Result<T>`

Type alias: `pub type Result<T> = Result<T, Error>;`
