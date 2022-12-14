use super::Widget;
use crate::utils::{append_child, create_element, set_attribute};
use web_sys::{HtmlDivElement, Node};

#[derive(Clone)]
pub struct VisuallyHidden {
	element: HtmlDivElement,
}

impl Widget for VisuallyHidden {
	fn render(&self) -> &Node {
		&self.element
	}
}

impl Default for VisuallyHidden {
	fn default() -> Self {
		Self::new()
	}
}

impl VisuallyHidden {
	pub fn new() -> Self {
		let element: HtmlDivElement = create_element("div");

		set_attribute(&element, "class", "korros__visually-hidden");

		VisuallyHidden { element }
	}

	pub fn child(self, element: &impl Widget) -> Self {
		append_child(&self.element, element.render());

		self
	}
}
