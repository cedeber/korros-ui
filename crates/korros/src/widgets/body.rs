use super::Widget;
use crate::utils::{append_child, create_element};
use gloo::utils::{body, head};
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
		append_child(&head, &style);

		Self { element }
	}

	pub fn child(self, element: &impl Widget) -> Self {
		append_child(&self.element, element.render());
		self
	}
}
