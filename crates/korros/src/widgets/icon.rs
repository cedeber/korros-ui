use super::ViewComponent;
use gloo::utils::document;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{Element, Node, Text as DomText};

pub enum IconSize {
	Normal,
	Small,
}

#[derive(Clone)]
pub struct Icon {
	element: DomText,
	parent: Element,
}

impl ViewComponent for Icon {
	fn render(&self) -> &Node {
		&self.parent
	}
}

impl Icon {
	pub fn new(icon: &str) -> Self {
		let parent = document().create_element("span").unwrap_throw();
		let element = document().create_text_node(icon);

		parent
			.set_attribute("class", "material-symbols-outlined")
			.unwrap_throw();
		parent.append_child(&element).unwrap_throw();

		Icon { element, parent }.set_icon(icon)
	}

	pub fn with_size(self, size: IconSize) -> Self {
		match size {
			IconSize::Normal => self.parent.remove_attribute("data-size").unwrap_throw(),
			IconSize::Small => self
				.parent
				.set_attribute("data-size", "small")
				.unwrap_throw(),
		}

		self
	}

	fn set_icon(self, text: &str) -> Self {
		if !text.is_empty() {
			self.element.set_text_content(Some(text));
		}

		self
	}
}
