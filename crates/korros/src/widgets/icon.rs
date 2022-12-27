use super::ViewComponent;
use crate::utils::{append_child, create_element, remove_attribute, set_attribute};
use gloo::utils::document;
use web_sys::{HtmlSpanElement, Node, Text as DomText};

pub enum IconSize {
	Normal,
	Small,
}

#[derive(Clone)]
pub struct Icon {
	element: DomText,
	parent: HtmlSpanElement,
}

impl ViewComponent for Icon {
	fn render(&self) -> &Node {
		&self.parent
	}
}

impl Icon {
	pub fn new(icon: &str) -> Self {
		let parent: HtmlSpanElement = create_element("span");
		let element = document().create_text_node(icon);

		set_attribute(&parent, "class", "material-symbols-outlined korros__icon");
		set_attribute(&parent, "aria-hidden", "true");
		append_child(&parent, &element);

		Icon { element, parent }.set_icon(icon)
	}

	pub fn size(self, size: IconSize) -> Self {
		match size {
			IconSize::Normal => remove_attribute(&self.parent, "data-size"),
			IconSize::Small => set_attribute(&self.parent, "data-size", "small"),
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
