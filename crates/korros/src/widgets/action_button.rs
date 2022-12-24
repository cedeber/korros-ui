use super::{
	fragment::{self, Fragment},
	icon::{Icon, IconSize},
	progress_circle::ProgressCircle,
	text::Text,
	ViewComponent,
};
use crate::utils::element::create_element;
use futures_signals::signal::{Signal, SignalExt};
use gloo::{console::externs::log, events::EventListener};
use std::{
	cell::RefCell,
	rc::Rc,
	sync::{Arc, Mutex},
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, Event, HtmlButtonElement, HtmlDivElement, KeyboardEvent, Node};

pub enum ButtonIntent {
	Filled,
	Tinted,
	Gray,
	Danger,
	Outlined,
	Plain,
}

#[derive(Clone)]
struct ButtonState {
	disabled: bool,
	loading: bool,
	loading_fragment: Option<Node>,
}

#[derive(Clone)]
pub struct Button {
	element: HtmlButtonElement,
	state: Arc<Mutex<ButtonState>>,
	text: Option<Text>,
	left_icon: Option<Icon>,
	right_icon: Option<Icon>,
}

impl ViewComponent for Button {
	fn render(&self) -> &Node {
		if let Some(icon) = &self.left_icon {
			self.element.append_child(icon.render()).unwrap_throw();
			self.element
				.set_attribute("data-icon", "left")
				.unwrap_throw();
		}

		if let Some(text) = &self.text {
			self.element.append_child(text.render()).unwrap_throw();
		}

		if self.left_icon.is_none() {
			if let Some(icon) = &self.right_icon {
				self.element.append_child(icon.render()).unwrap_throw();
				self.element
					.set_attribute("data-icon", "right")
					.unwrap_throw();
			}
		}

		if self.text.is_none() {
			self.element
				.set_attribute("data-icon", "single")
				.unwrap_throw();
		}

		&self.element
	}
}

// TODO Add aria-label? or label struct?
impl Button {
	pub fn new(label: &str) -> Self {
		let button: HtmlButtonElement = create_element("button");
		button
			.set_attribute("class", "korros__action-button")
			.unwrap_throw();

		let text = if label.is_empty() {
			None
		} else {
			Some(Text::new(label))
		};

		Button {
			element: button,
			state: Arc::new(Mutex::new(ButtonState {
				disabled: false,
				loading: false,
				loading_fragment: None,
			})),
			text,
			left_icon: None,
			right_icon: None,
		}
		.intent(ButtonIntent::Filled)
	}

	/// Won't call the callback is disabled or loading
	pub fn on_press(self, callback: impl Fn(&Event) + 'static) -> Self {
		let state = Arc::clone(&self.state);

		let cb = move |e: &Event| {
			let data = state.lock().unwrap_throw();
			if !data.disabled && !data.loading {
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

	pub fn intent(self, intent: ButtonIntent) -> Self {
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

	pub fn disabled_signal(self, signal: impl Signal<Item = bool> + 'static) -> Self {
		let clone = self.clone();
		let future = signal.for_each(move |value| {
			clone.clone().set_disabled(value);
			async {}
		});

		spawn_local(future);

		self
	}

	pub fn left_icon(mut self, icon: &str) -> Self {
		self.left_icon = Some(Icon::new(icon).size(IconSize::Small));

		self
	}

	pub fn right_icon(mut self, icon: &str) -> Self {
		self.right_icon = Some(Icon::new(icon).size(IconSize::Small));

		self
	}
	pub fn loading_signal(self, signal: impl Signal<Item = bool> + 'static) -> Self {
		let clone = self.clone();
		let future = signal.for_each(move |value| {
			clone.clone().set_loading(value);
			async {}
		});

		spawn_local(future);

		self
	}

	fn set_disabled(&self, value: bool) {
		let state = Arc::clone(&self.state);
		let mut data = state.lock().unwrap_throw();
		data.disabled = value;

		match value {
			true => self.element.set_attribute("disabled", "").unwrap_throw(),
			false => self.element.remove_attribute("disabled").unwrap_throw(),
		};
	}

	fn set_loading(self, value: bool) {
		let state = Arc::clone(&self.state);
		let mut data = state.lock().unwrap_throw();
		data.loading = value;
		let loading_fragment = data.loading_fragment.clone();

		match value {
			true => {
				self.element
					.set_attribute("data-loading", "true")
					.unwrap_throw();

				if let Some(fragment) = loading_fragment {
					self.element.remove_child(&fragment).unwrap_throw();
				}

				let fragment = ProgressCircle::new(18.0, true);
				let fragment = fragment.render();
				self.element.append_child(fragment).unwrap_throw();
				data.loading_fragment = Some(fragment.clone());
			}
			false => {
				self.element.remove_attribute("data-loading").unwrap_throw();
				if let Some(fragment) = loading_fragment {
					self.element.remove_child(&fragment).unwrap_throw();
					data.loading_fragment = None;
				}
			}
		};
	}

	pub fn aria_label(self, label: &str) -> Self {
		self.element
			.set_attribute("aria-label", label)
			.unwrap_throw();
		self
	}
}
