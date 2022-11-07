use std::sync::{Arc, Mutex};

use super::ViewComponent;
use futures_signals::signal::{Signal, SignalExt};
use gloo::{events::EventListener, utils::document};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, Node};

struct ToogleState {
	callback: Option<Box<dyn Fn(bool) + 'static>>,
}

#[derive(Clone)]
pub struct Toggle {
	element: HtmlInputElement,
	state: Arc<Mutex<ToogleState>>,
}

impl ViewComponent for Toggle {
	fn render(&self) -> &Node {
		&self.element
	}
}

// TODO: Add an inner signal to update the checked attribute/state?
impl Toggle {
	pub fn new(checked: bool) -> Self {
		let element = document()
			.create_element("input")
			.unwrap_throw()
			.dyn_into::<HtmlInputElement>()
			.unwrap_throw();
		element.set_attribute("type", "checkbox").unwrap_throw();

		let toggle = Toggle {
			element,
			state: Arc::new(Mutex::new(ToogleState { callback: None })),
		}
		.set_checked(checked);

		let clone = toggle.clone();

		// TODO: We should set the event later, only if a callback is set.
		EventListener::new(&toggle.element, "change", move |_| {
			let state2 = Arc::clone(&clone.state);
			let data2 = state2.lock().unwrap_throw();
			let is_checked = clone.element.checked();

			clone.clone().set_checked(is_checked);

			if let Some(cb) = &data2.callback {
				cb(is_checked);
			}
		})
		.forget();

		toggle
	}

	pub fn new_with_signal(signal: impl Signal<Item = bool> + 'static) -> Self {
		let toggle = Toggle::new(false);
		let clone = toggle.clone();
		let future = signal.for_each(move |checked| {
			let was_checked = clone.element.checked();

			clone.clone().set_checked(checked);

			let state = Arc::clone(&clone.state);
			let data = state.lock().unwrap_throw();
			if let Some(cb) = &data.callback {
				// In case the on_change callback update the attached signal,
				// we do nothing because the value didn't change.
				// => Was already set by the UI, aka. on_change.
				if checked != was_checked {
					cb(checked);
				}
			}

			async {}
		});

		spawn_local(future);

		toggle
	}

	pub fn with_change_callback(self, callback: impl Fn(bool) + 'static) -> Self {
		let state = Arc::clone(&self.state);
		let mut data = state.lock().unwrap_throw();

		data.callback = Some(Box::new(callback));

		self
	}

	fn set_checked(self, value: bool) -> Self {
		self.element.set_checked(value);
		match value {
			true => self.element.set_attribute("checked", "").unwrap_throw(),
			false => self.element.remove_attribute("checked").unwrap_throw(),
		}

		self
	}
}
