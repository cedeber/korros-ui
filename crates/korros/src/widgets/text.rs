use super::ViewComponent;
use crate::utils::element::create_element;
use futures_signals::signal::{Signal, SignalExt};
use gloo::utils::document;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlSpanElement, Node, Text as DomText};

#[derive(Clone)]
pub struct Text {
	element: DomText,
	container: HtmlSpanElement,
}

impl ViewComponent for Text {
	fn render(&self) -> &Node {
		self.container.append_child(&self.element).unwrap_throw();

		&self.container
	}
}

impl Text {
	pub fn new(text: &str) -> Self {
		let container: HtmlSpanElement = create_element("span");
		let element = document().create_text_node(text);

		Text { element, container }.set_text(text)
	}

	pub fn new_signal<U: Into<String>>(signal: impl Signal<Item = U> + 'static) -> Self {
		let text = Text::new("");
		let clone = text.clone();
		let future = signal.for_each(move |value| {
			clone.clone().set_text(&value.into());
			async {}
		});

		spawn_local(future);

		text
	}

	fn set_text<U: Into<String>>(self, text: U) -> Self {
		let text = text.into();

		if !text.is_empty() {
			self.element.set_text_content(Some(&text));
		}

		self
	}
}
