use serde::Serialize;
use std::collections::HashMap;

#[doc = include_str!("../docs/rustdoc/elem.md")]
#[derive(Debug, Serialize)]
pub struct Elem {
	pub tag: String,
	pub attrs: Option<HashMap<String, String>>,
	pub text: Option<String>,
	pub inner_html: Option<String>,
}

impl Elem {
	pub fn attr(&self, name: &str) -> Option<&str> {
		self.attrs.as_ref()?.get(name).map(String::as_str)
	}

	/// Creates a new `Elem` from a `scraper::ElementRef`.
	pub(crate) fn from_element_ref(el_ref: scraper::ElementRef) -> Self {
		let el = el_ref.value();
		let tag = el.name().to_string();

		let attrs = if el.attrs().next().is_some() {
			let attrs = el.attrs().map(|(k, v)| (k.to_string(), v.to_string())).collect();
			Some(attrs)
		} else {
			None
		};

		let full_text = el_ref.text().collect::<String>();
		let text = if full_text.trim().is_empty() {
			None
		} else {
			Some(full_text.to_string())
		};

		let html_content = el_ref.inner_html();
		let inner_html = if html_content.trim().is_empty() {
			None
		} else {
			Some(html_content.to_string())
		};

		Elem {
			tag,
			attrs,
			text,
			inner_html,
		}
	}
}

// region:    --- Tests

#[cfg(test)]
mod tests {
	use super::*;

	type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

	#[test]
	fn test_elem_attr_present_and_missing() -> Result<()> {
		// -- Setup & Fixtures
		let mut attrs = HashMap::new();
		attrs.insert("href".to_string(), "https://example.com".to_string());
		let elem = Elem {
			tag: "a".to_string(),
			attrs: Some(attrs),
			text: None,
			inner_html: None,
		};

		// -- Exec & Check
		assert_eq!(elem.attr("href"), Some("https://example.com"));
		assert_eq!(elem.attr("title"), None);

		Ok(())
	}

	#[test]
	fn test_elem_attr_without_attributes() -> Result<()> {
		// -- Setup & Fixtures
		let elem = Elem {
			tag: "p".to_string(),
			attrs: None,
			text: None,
			inner_html: None,
		};

		// -- Exec & Check
		assert_eq!(elem.attr("class"), None);

		Ok(())
	}
}

// endregion: --- Tests
