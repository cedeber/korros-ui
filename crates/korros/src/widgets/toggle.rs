use super::ViewComponent;
use gloo::{events::EventListener, utils::document};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, Node};

#[derive(Clone)]
pub struct Toggle {
	element: HtmlInputElement,
}

impl ViewComponent for Toggle {
	fn render(&self) -> &Node {
		&self.element
	}
}

impl Toggle {
	pub fn new(checked: bool) -> Self {
		let element = document()
			.create_element("input")
			.unwrap_throw()
			.dyn_into::<HtmlInputElement>()
			.unwrap_throw();
		element.set_attribute("type", "checkbox").unwrap_throw();

		Toggle { element }.set_checked(checked)
	}

	pub fn on_change(self, callback: impl Fn(bool) + 'static) -> Self {
		let clone = self.clone();

		EventListener::new(&self.element, "change", move |_| {
			let is_checked = clone.element.checked();
			clone.clone().set_checked(is_checked);
			callback(is_checked);
		})
		.forget();

		self
	}

	fn set_checked(self, value: bool) -> Self {
		match value {
			true => self.element.set_attribute("checked", "").unwrap_throw(),
			false => self.element.remove_attribute("checked").unwrap_throw(),
		}

		self
	}
}
