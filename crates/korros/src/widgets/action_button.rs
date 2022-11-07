use std::sync::{Arc, Mutex};

use futures_signals::signal::{Signal, SignalExt};
use gloo::{events::EventListener, utils::document};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
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
struct ButtonState {
	disabled: bool,
	loading: bool,
}

#[derive(Clone)]
pub struct Button {
	element: Element,
	state: Arc<Mutex<ButtonState>>,
}

impl ViewComponent for Button {
	fn render(&self) -> &Node {
		&self.element
	}
}

impl Button {
	pub fn new(label: &str) -> Self {
		let document = document();
		let button = document.create_element("button").unwrap_throw();
		button
			.set_attribute("class", "action-button")
			.unwrap_throw();

		Button {
			element: button,
			state: Arc::new(Mutex::new(ButtonState {
				disabled: false,
				loading: false,
			})),
		}
		.set_label(label)
		.with_intent(ButtonIntent::Filled)
	}

	pub fn on_press(self, callback: impl Fn(&Event) + 'static) -> Self {
		let state = Arc::clone(&self.state);

		let cb = move |e: &Event| {
			let data = state.lock().unwrap_throw();
			if !data.disabled {
				callback(e);
			}
		};

		let pointer_cb = Arc::new(cb);
		let keyboard_cb = pointer_cb.clone();

		// Pointer
		EventListener::new(&self.element, "pointerup", move |event| {
			pointer_cb.clone()(event)
		})
		.forget();

		// Keyboard : Enter + Space
		EventListener::new(&self.element, "keyup", move |event| {
			let event = event.dyn_ref::<KeyboardEvent>().unwrap_throw();
			if event.code() == "Enter" || event.code() == "Space" {
				keyboard_cb(event);
			}
		})
		.forget();

		self
	}

	pub fn with_intent(self, intent: ButtonIntent) -> Self {
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

	pub fn with_disabled_signal(self, signal: impl Signal<Item = bool> + 'static) -> Self {
		let clone = self.clone();
		let future = signal.for_each(move |value| {
			clone.clone().set_disabled(value);
			async {}
		});

		spawn_local(future);

		self
	}

	fn set_disabled(self, value: bool) -> Self {
		let state = Arc::clone(&self.state);
		let mut data = state.lock().unwrap_throw();
		data.disabled = value;

		match value {
			true => self.element.set_attribute("disabled", "").unwrap_throw(),
			false => self.element.remove_attribute("disabled").unwrap_throw(),
		};

		self
	}

	fn set_label(self, label: &str) -> Self {
		self.element.set_text_content(Some(label));
		self
	}
}
