use super::ViewComponent;
use gloo::utils::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlElement, Node};

#[derive(Clone)]
pub struct VisuallyHidden {
	element: HtmlElement,
}

impl ViewComponent for VisuallyHidden {
	fn render(&self) -> &Node {
		&self.element
	}
}

impl VisuallyHidden {
	pub fn new(text: &str) -> Self {
		let element = document()
			.create_element("div")
			.unwrap_throw()
			.dyn_into::<HtmlElement>()
			.unwrap_throw();

		element
			.set_attribute("class", "visually-hidden")
			.unwrap_throw();

		VisuallyHidden { element }
	}

	pub fn with_child(self, element: &impl ViewComponent) -> Self {
		self.element.append_child(element.render()).unwrap_throw();

		self
	}
}
