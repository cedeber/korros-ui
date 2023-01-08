use super::{
	fragment::Fragment,
	icon::{Icon, IconSize},
	text::Text,
	Widget,
};
use crate::utils::{
	append_child, create_element, remove_attribute, set_attribute, set_bool_attribute,
};
use futures_signals::signal::{Signal, SignalExt};
use gloo::events::EventListener;
use std::sync::{Arc, Mutex};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlButtonElement, KeyboardEvent, Node};

pub enum TriggerButtonIntent {
	Active,
	Danger,
	Outlined,
	Discrete,
}

#[derive(Clone)]
struct TriggerButtonState {
	disabled: bool,
}

#[derive(Clone)]
pub struct TriggerButton {
	element: HtmlButtonElement,
	state: Arc<Mutex<TriggerButtonState>>,
	text: Fragment,
	icon: Icon,
}

impl Widget for TriggerButton {
	fn render(&self) -> &Node {
		append_child(&self.element, self.icon.render());
		append_child(&self.element, self.text.render());
		&self.element
	}
}

// TODO Add aria-label? or label struct?
impl TriggerButton {
	pub fn new(label: &str, icon: &str) -> Self {
		let button: HtmlButtonElement = create_element("button");
		set_attribute(&button, "class", "korros__trigger-button");
		set_attribute(&button, "data-size", "default");
		set_attribute(&button, "aria-label", label);

		TriggerButton {
			element: button,
			state: Arc::new(Mutex::new(TriggerButtonState { disabled: false })),
			text: Fragment::new_hidden().child(Text::new(label)),
			icon: Icon::new(icon).size(IconSize::Small),
		}
		.intent(TriggerButtonIntent::Discrete)
	}

	/// Won't call the callback is disabled or loading
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

	pub fn intent(self, intent: TriggerButtonIntent) -> Self {
		match intent {
			TriggerButtonIntent::Active => set_attribute(&self.element, "data-intent", "tinted"),
			TriggerButtonIntent::Outlined => {
				set_attribute(&self.element, "data-intent", "outlined")
			}
			TriggerButtonIntent::Discrete => set_attribute(&self.element, "data-intent", "plain"),
			TriggerButtonIntent::Danger => set_attribute(&self.element, "data-intent", "danger"),
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

	fn set_disabled(&self, value: bool) {
		let state = Arc::clone(&self.state);
		let mut data = state.lock().unwrap_throw();
		data.disabled = value;

		match value {
			true => set_bool_attribute(&self.element, "disabled"),
			false => remove_attribute(&self.element, "disabled"),
		};
	}

	pub fn show_label_signal(mut self, signal: impl Signal<Item = bool> + 'static) -> Self {
		let broadcaster = signal.broadcast();
		self.text = self.text.show_signal(broadcaster.signal());

		let clone = self.clone();

		let future = broadcaster.signal().for_each(move |value| {
			set_attribute(
				&clone.element,
				"data-size",
				if value { "large" } else { "default" },
			);

			async {}
		});

		spawn_local(future);

		self
	}
}
