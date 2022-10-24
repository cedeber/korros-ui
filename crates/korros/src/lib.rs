use std::hash::BuildHasherDefault;

use gloo::{
	events::EventListener,
	utils::{body, document},
};
use log::{info, warn, Level};
use wasm_bindgen::prelude::*;
use web_sys::{Element, Event, HtmlElement};

trait ViewComponent {
	fn get(&self) -> &Element;
}

struct Body {
	element: HtmlElement,
}

impl Body {
	fn new() -> Self {
		Self { element: body() }
	}

	fn with_child(self, element: &impl ViewComponent) -> Self {
		self.element.append_child(element.get()).unwrap_throw();

		self
	}
}

enum ButtonIntent {
	Filled,
	Tinted,
	Gray,
	Outlined,
	Plain,
	Danger,
}

struct Button {
	element: Element,
}

impl ViewComponent for Button {
	fn get(&self) -> &Element {
		&self.element
	}
}

impl Button {
	fn new(label: &str) -> Self {
		let document = document();
		let button = document.create_element("action-button").unwrap_throw();
		button.set_attribute("role", "button").unwrap_throw();

		let button = Button { element: button }
			.set_label(label)
			.with_intent(ButtonIntent::Filled);

		button
	}

	fn on_press(self, callback: impl Fn(&Event) + 'static) -> Self {
		let event = EventListener::new(&self.element, "pointerup", callback);
		event.forget();
		self
	}

	fn with_intent(self, intent: ButtonIntent) -> Self {
		match intent {
			ButtonIntent::Filled => self
				.element
				.set_attribute("data-intent", "filled")
				.unwrap_throw(),
			ButtonIntent::Tinted => self
				.element
				.set_attribute("data-intent", "tinted")
				.unwrap_throw(),
			ButtonIntent::Gray => self
				.element
				.set_attribute("data-intent", "gray")
				.unwrap_throw(),
			ButtonIntent::Outlined => self
				.element
				.set_attribute("data-intent", "outlined")
				.unwrap_throw(),
			ButtonIntent::Plain => self
				.element
				.set_attribute("data-intent", "plain")
				.unwrap_throw(),
			ButtonIntent::Danger => self
				.element
				.set_attribute("data-intent", "danger")
				.unwrap_throw(),
		};

		self
	}

	fn is_disabled(self, value: bool) -> Self {
		self.element
			.set_attribute(
				"disabled",
				match value {
					true => "true",
					false => "false",
				},
			)
			.unwrap_throw();
		self
	}

	fn set_label(self, label: &str) -> Self {
		self.element.set_text_content(Some(label));
		self
	}
}

#[wasm_bindgen(start)]
pub fn main_wasm() -> Result<(), JsValue> {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();

	#[cfg(feature = "console_log")]
	console_log::init_with_level(Level::Trace).expect("error initializing log");

	info!("Hello, World!");

	let button = Button::new("Click me!")
		.on_press(|_| warn!("Hello, Button!"))
		.with_intent(ButtonIntent::Danger);

	Body::new().with_child(&button);

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use wasm_bindgen_test::*;

	#[wasm_bindgen_test]
	fn it_works() {
		assert_eq!(5, 3 + 2);
	}
}
