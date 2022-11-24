use super::ViewComponent;
use crate::utils::element::create_element;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlDivElement, Node};

#[derive(Clone)]
pub struct VisuallyHidden {
	element: HtmlDivElement,
}

impl ViewComponent for VisuallyHidden {
	fn render(&self) -> &Node {
		&self.element
	}
}

impl VisuallyHidden {
	pub fn new() -> Self {
		let element: HtmlDivElement = create_element("div");

		element
			.set_attribute("class", "korros__visually-hidden")
			.unwrap_throw();

		VisuallyHidden { element }
	}

	pub fn with_child(self, element: &impl ViewComponent) -> Self {
		self.element.append_child(element.render()).unwrap_throw();

		self
	}
}
