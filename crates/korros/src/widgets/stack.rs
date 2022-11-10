use gloo::utils::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;

use super::ViewComponent;

pub struct HStack {
	element: HtmlElement,
}

impl ViewComponent for HStack {
	fn render(&self) -> &web_sys::Node {
		&self.element
	}
}

impl Default for HStack {
	fn default() -> Self {
		Self::new()
	}
}

impl HStack {
	pub fn new() -> Self {
		let document = document();
		let stack = document
			.create_element("div")
			.unwrap_throw()
			.dyn_into::<HtmlElement>()
			.unwrap_throw();

		stack
			.set_attribute("class", "korros__h-stack")
			.unwrap_throw();
		stack.style().set_property("gap", "5px").unwrap_throw();

		HStack { element: stack }
	}

	pub fn with_gap(self, gap: u32) -> Self {
		self.element
			.style()
			.set_property("gap", &format!("{gap}px"))
			.unwrap_throw();

		self
	}

	pub fn with_padding(self, top_bottom: u32, left_right: u32) -> Self {
		self.element
			.style()
			.set_property("padding", &format!("{top_bottom}px {left_right}px"))
			.unwrap_throw();

		self
	}

	pub fn with_child(self, element: &impl ViewComponent) -> Self {
		self.element.append_child(element.render()).unwrap_throw();

		self
	}
}

pub struct VStack {
	element: HtmlElement,
}

impl ViewComponent for VStack {
	fn render(&self) -> &web_sys::Node {
		&self.element
	}
}

impl Default for VStack {
	fn default() -> Self {
		Self::new()
	}
}

impl VStack {
	pub fn new() -> Self {
		let document = document();
		let stack = document
			.create_element("div")
			.unwrap_throw()
			.dyn_into::<HtmlElement>()
			.unwrap_throw();

		stack
			.set_attribute("class", "korros__v-stack")
			.unwrap_throw();
		stack.style().set_property("gap", "5px").unwrap_throw();

		VStack { element: stack }
	}

	pub fn with_gap(self, gap: u32) -> Self {
		self.element
			.style()
			.set_property("gap", &format!("{gap}px"))
			.unwrap_throw();

		self
	}

	pub fn with_padding(self, top_bottom: u32, left_right: u32) -> Self {
		self.element
			.style()
			.set_property("padding", &format!("{top_bottom}px {left_right}px"))
			.unwrap_throw();

		self
	}

	pub fn with_child(self, element: &impl ViewComponent) -> Self {
		self.element.append_child(element.render()).unwrap_throw();

		self
	}
}
