use super::ViewComponent;
use crate::utils::element::create_element;
use gloo::utils::{body, head};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlElement, HtmlStyleElement};

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
		let style: HtmlStyleElement = create_element("style");
		style.set_text_content(Some(styles));
		head.append_child(&style).unwrap_throw();

		Self { element }
	}

	pub fn child(self, element: &impl ViewComponent) -> Self {
		self.element.append_child(element.render()).unwrap_throw();

		self
	}
}
