use super::ViewComponent;
use futures_signals::signal::{ Signal, SignalExt};
use gloo::utils::document;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Node, Text as DomText};

#[derive(Clone)]
pub struct Text {
	element: DomText,
	parent: Element,
}

impl ViewComponent for Text {
	fn get(&self) -> &Node {
		&self.parent
	}
}

impl Text {
	pub fn new(text: &str) -> Self {
		let parent = document().create_element("span").unwrap_throw();
		let element = document().create_text_node(text);

		parent.append_child(&element).unwrap_throw();

		Text { element, parent }.set_text(text)
	}

    pub fn signal(signal: impl Signal<Item = &'static str> + 'static) -> Self {
        let text = Text::new("");
        let clone = text.clone();
        let future = signal.for_each(move |value| {
            clone.clone().set_text(value);
            async {}
        });

        spawn_local(future);

        text
    }

	fn set_text(self, text: &str) -> Self {
        if !text.is_empty() {
            self.element.set_text_content(Some(text));
        }

		self
	}
}