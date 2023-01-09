use super::{
	icon::{Icon, IconSize},
	progress_circle::ProgressCircle,
	text::Text,
	Widget,
};
use crate::utils::{
	append_child, create_element, remove_attribute, remove_child, set_attribute, set_bool_attribute,
};
use futures_signals::signal::{Signal, SignalExt};
use gloo::events::EventListener;
use std::{
	rc::Rc,
	sync::{Arc, Mutex},
};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlButtonElement, KeyboardEvent, Node};

pub enum ActionButtonIntent {
	Primary,
	Active,
	Secondary,
	Danger,
	Outlined,
	Discrete,
}

struct ActionButtonState {
	disabled: bool,
	loading: bool,
	loading_fragment: Option<Node>,
}

#[derive(Clone)]
pub struct ActionButton {
	element: HtmlButtonElement,
	state: Arc<Mutex<ActionButtonState>>,
	text: Option<Text>,
	left_icon: Option<Icon>,
	right_icon: Option<Icon>,
}

impl Widget for ActionButton {
	fn render(&self) -> &Node {
		if let Some(icon) = &self.left_icon {
			append_child(&self.element, icon.render());
			set_attribute(&self.element, "data-icon", "left");
		}

		if let Some(text) = &self.text {
			append_child(&self.element, text.render());
		}

		if self.left_icon.is_none() {
			if let Some(icon) = &self.right_icon {
				append_child(&self.element, icon.render());
				set_attribute(&self.element, "data-icon", "right");
			}
		}

		if self.text.is_none() {
			set_attribute(&self.element, "data-icon", "single");
		}

		&self.element
	}
}

// TODO Add aria-label? or label struct?
impl ActionButton {
	pub fn new(label: &str) -> Self {
		let button: HtmlButtonElement = create_element("button");
		set_attribute(&button, "class", "korros__action-button");

		let text = if label.is_empty() {
			None
		} else {
			Some(Text::new(label))
		};

		ActionButton {
			element: button,
			state: Arc::new(Mutex::new(ActionButtonState {
				disabled: false,
				loading: false,
				loading_fragment: None,
			})),
			text,
			left_icon: None,
			right_icon: None,
		}
		.intent(ActionButtonIntent::Primary)
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

		let pointer_cb = Rc::new(cb);
		let keyboard_cb = Rc::clone(&pointer_cb);

		// Pointer
		EventListener::new(&self.element, "pointerup", move |event| pointer_cb(event)).forget();

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

	pub fn intent(self, intent: ActionButtonIntent) -> Self {
		match intent {
			ActionButtonIntent::Primary => set_attribute(&self.element, "data-intent", "filled"),
			ActionButtonIntent::Active => set_attribute(&self.element, "data-intent", "tinted"),
			ActionButtonIntent::Secondary => set_attribute(&self.element, "data-intent", "gray"),
			ActionButtonIntent::Outlined => set_attribute(&self.element, "data-intent", "outlined"),
			ActionButtonIntent::Discrete => set_attribute(&self.element, "data-intent", "plain"),
			ActionButtonIntent::Danger => set_attribute(&self.element, "data-intent", "danger"),
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
			true => set_bool_attribute(&self.element, "disabled"),
			false => remove_attribute(&self.element, "disabled"),
		};
	}

	fn set_loading(self, value: bool) {
		let state = Arc::clone(&self.state);
		let mut data = state.lock().unwrap_throw();
		data.loading = value;
		let loading_fragment = data.loading_fragment.clone();

		match value {
			true => {
				set_attribute(&self.element, "data-loading", "true");

				if let Some(fragment) = loading_fragment {
					remove_child(&self.element, &fragment);
				}

				let fragment = ProgressCircle::new(18.0, true);
				let fragment = fragment.render();
				append_child(&self.element, fragment);
				data.loading_fragment = Some(fragment.clone());
			}
			false => {
				remove_attribute(&self.element, "data-loading");

				if let Some(fragment) = loading_fragment {
					remove_child(&self.element, &fragment);
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
