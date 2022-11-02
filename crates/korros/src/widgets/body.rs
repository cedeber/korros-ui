use gloo::utils::body;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlElement;

use super::ViewComponent;

pub struct Body {
	element: HtmlElement,
}

impl Body {
	pub fn new() -> Self {
		Self { element: body() }
	}

	pub fn with_child(self, element: &impl ViewComponent) -> Self {
		self.element.append_child(element.get()).unwrap_throw();

		self
	}
}
