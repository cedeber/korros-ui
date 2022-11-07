use super::ViewComponent;
use futures_signals::signal::{Signal, SignalExt};
use gloo::utils::document;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Node, Text as DomText};

#[derive(Clone)]
pub struct Text {
	element: DomText,
	container: Element,
}

impl ViewComponent for Text {
	fn render(&self) -> &Node {
		self.container.append_child(&self.element).unwrap_throw();

		&self.container
	}
}

impl Text {
	pub fn new(text: &str) -> Self {
		let container = document().create_element("span").unwrap_throw();
		let element = document().create_text_node(text);

		Text { element, container }.set_text(text)
	}

	pub fn new_with_signal<T: Into<String>>(signal: impl Signal<Item = T> + 'static) -> Self {
		let text = Text::new("");
		let clone = text.clone();
		let future = signal.for_each(move |value| {
			clone.clone().set_text(&value.into());
			async {}
		});

		spawn_local(future);

		text
	}

	fn set_text<T: Into<String>>(self, text: T) -> Self {
		let text = text.into();

		if !text.is_empty() {
			self.element.set_text_content(Some(&text));
		}

		self
	}
}
