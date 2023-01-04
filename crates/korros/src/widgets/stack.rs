use super::Widget;
use crate::utils::{append_child, create_element, set_attribute, set_style};
use web_sys::{HtmlDivElement, Node};

#[derive(Clone)]
pub struct HStack {
	element: HtmlDivElement,
}

impl Widget for HStack {
	fn render(&self) -> &Node {
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
		let stack: HtmlDivElement = create_element("div");

		set_attribute(&stack, "class", "korros__h-stack");
		set_style(&stack, "gap", "5px");

		HStack { element: stack }
	}

	pub fn gap(self, gap: u32) -> Self {
		set_style(&self.element, "gap", &format!("{gap}px"));
		self
	}

	pub fn padding(self, top_bottom: u32, left_right: u32) -> Self {
		set_style(
			&self.element,
			"padding",
			&format!("{top_bottom}px {left_right}px"),
		);
		self
	}

	pub fn child(self, element: &impl Widget) -> Self {
		append_child(&self.element, element.render());
		self
	}
}

#[derive(Clone)]
pub struct VStack {
	element: HtmlDivElement,
}

impl Widget for VStack {
	fn render(&self) -> &Node {
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
		let stack: HtmlDivElement = create_element("div");

		set_attribute(&stack, "class", "korros__v-stack");
		set_style(&stack, "gap", "5px");

		VStack { element: stack }
	}

	pub fn gap(self, gap: u32) -> Self {
		set_style(&self.element, "gap", &format!("{gap}px"));
		self
	}

	pub fn padding(self, top_bottom: u32, left_right: u32) -> Self {
		set_style(
			&self.element,
			"padding",
			&format!("{top_bottom}px {left_right}px"),
		);
		self
	}

	pub fn child(self, element: &impl Widget) -> Self {
		append_child(&self.element, element.render());
		self
	}
}
