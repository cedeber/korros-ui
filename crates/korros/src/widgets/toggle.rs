use super::ViewComponent;
use crate::utils::element::create_element;
use futures_signals::signal::{Signal, SignalExt};
use gloo::events::EventListener;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, Node};

struct ToogleState {
	callback: Option<Box<dyn Fn(bool) + 'static>>,
	checked: bool,
	disabled: bool,
}

#[derive(Clone)]
pub struct Toggle {
	element: HtmlInputElement,
	state: Rc<RefCell<ToogleState>>,
}

impl ViewComponent for Toggle {
	fn render(&self) -> &Node {
		&self.element
	}
}

// TODO: Add an inner signal to update the checked attribute/state?
impl Toggle {
	pub fn new(checked: bool, disabled: bool) -> Self {
		let element: HtmlInputElement = create_element("input");
		element.set_attribute("type", "checkbox").unwrap_throw();

		let toggle = Toggle {
			element,
			state: Rc::new(RefCell::new(ToogleState {
				callback: None,
				checked,
				disabled,
			})),
		}
		.set_checked(checked)
		.set_disabled(disabled);

		let clone = toggle.clone();

		// TODO: We should set the event later, only if a callback is set.
		EventListener::new(&toggle.element, "change", move |_| {
			let state = clone.state.borrow();
			let is_checked = clone.element.checked();

			if let Some(cb) = &state.callback {
				cb(is_checked);
			}

			clone.clone().set_checked(is_checked);
		})
		.forget();

		toggle
	}

	pub fn new_signal(signal: impl Signal<Item = bool> + 'static) -> Self {
		let toggle = Toggle::new(false, false);
		let clone = toggle.clone();

		let future = signal.for_each(move |checked| {
			let state = clone.state.borrow();
			let was_checked = clone.element.checked();

			clone.clone().set_checked(checked);

			if let Some(cb) = &state.callback {
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

	pub fn disabled_signal(self, signal: impl Signal<Item = bool> + 'static) -> Self {
		let clone = self.clone();

		let future = signal.for_each(move |disabled| {
			clone.clone().set_disabled(disabled);
			async {}
		});

		spawn_local(future);
		self
	}

	pub fn on_change(self, callback: impl Fn(bool) + 'static) -> Self {
		let state = Rc::clone(&self.state);
		let mut state = state.borrow_mut();

		state.callback = Some(Box::new(callback));
		self
	}

	fn set_checked(self, checked: bool) -> Self {
		let state = Rc::clone(&self.state);
		let mut state = state.borrow_mut();

		state.checked = checked;
		self.element.set_checked(checked);

		match checked {
			true => self.element.set_attribute("checked", "").unwrap_throw(),
			false => self.element.remove_attribute("checked").unwrap_throw(),
		}

		self
	}

	fn set_disabled(self, disabled: bool) -> Self {
		let state = Rc::clone(&self.state);
		let mut state = state.borrow_mut();

		state.disabled = disabled;
		self.element.set_disabled(disabled);

		match disabled {
			true => self.element.set_attribute("disabled", "").unwrap_throw(),
			false => self.element.remove_attribute("disabled").unwrap_throw(),
		}

		self
	}
}
