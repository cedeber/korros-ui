use super::{
	icon::{Icon, IconSize},
	text::Text,
	ViewComponent,
};
use crate::utils::element::create_element;
use futures_signals::signal::{Signal, SignalExt};
use gloo::events::EventListener;
use std::sync::{Arc, Mutex};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlButtonElement, KeyboardEvent, Node};

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
		}

		if let Some(text) = &self.text {
			self.element.append_child(text.render()).unwrap_throw();
		}

		if let Some(icon) = &self.right_icon {
			self.element.append_child(icon.render()).unwrap_throw();
		}

		&self.element
	}
}

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
			})),
			text,
			left_icon: None,
			right_icon: None,
		}
		.intent(ButtonIntent::Filled)
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

	pub fn icon(mut self, icon: &str) -> Self {
		self.left_icon = Some(Icon::new(icon).size(IconSize::Small));

		self
	}

	// fn set_label(self, label: &str) -> Self {
	// 	self.element.set_text_content(Some(label));
	// 	self
	// }
}
