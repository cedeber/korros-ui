use std::sync::Arc;

use gloo::{console::info, events::EventListener, utils::document};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Element, Event, KeyboardEvent, Node};

use super::ViewComponent;

pub enum ButtonIntent {
	Filled,
	Tinted,
	Gray,
	Outlined,
	Plain,
	Danger,
}

#[derive(Clone)]
pub struct Button {
	element: Element,
}

impl ViewComponent for Button {
	fn get(&self) -> &Node {
		&self.element
	}
}

impl Button {
	pub fn new(label: &str) -> Self {
		let document = document();
		let button = document.create_element("action-button").unwrap_throw();
		button.set_attribute("role", "button").unwrap_throw();
		button.set_attribute("tabindex", "0").unwrap_throw();

		Button { element: button }
			.set_label(label)
			.with_intent(ButtonIntent::Filled)
	}

	pub fn on_press(self, callback: impl Fn(&Event) + 'static) -> Self {
		let pointer_cb = Arc::new(callback);
		let keyboard_cb = pointer_cb.clone();

		// Pointer
		EventListener::new(&self.element, "pointerup", move |event| {
			pointer_cb.clone()(event)
		})
		.forget();

		// Keyboard : Enter + Space
		EventListener::new(&self.element, "keyup", move |event| {
			let event = event.dyn_ref::<KeyboardEvent>().unwrap_throw();
			info!("{:?}", event.key());
			if event.code() == "Enter" || event.code() == "Space" {
				keyboard_cb(event);
			}
		})
		.forget();

		self
	}

	pub fn with_intent(self, intent: ButtonIntent) -> Self {
		match intent {
			ButtonIntent::Filled => {
				self.element
					.set_attribute("data-intent", "filled")
					.unwrap_throw();
			}
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

	pub fn is_disabled(self, value: bool) -> Self {
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

	pub fn set_label(self, label: &str) -> Self {
		self.element.set_text_content(Some(label));
		self
	}
}
