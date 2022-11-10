use gloo::utils::{body, document, head};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlElement;

use super::ViewComponent;

pub struct Body {
	element: HtmlElement,
}

impl Default for Body {
	fn default() -> Self {
		Self::new()
	}
}

impl Body {
	pub fn new() -> Self {
		let element = body();

		// Inject CSS styles
		let styles = include_str!("../assets/styles.css");
		let head = head();
		let style = document().create_element("style").unwrap_throw();
		style.set_text_content(Some(styles));
		head.append_child(&style).unwrap_throw();

		Self { element }
	}

	pub fn with_child(self, element: &impl ViewComponent) -> Self {
		self.element.append_child(element.render()).unwrap_throw();

		self
	}
}
