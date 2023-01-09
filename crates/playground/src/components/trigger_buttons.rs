use futures_signals::signal::Mutable;
use korros::widgets::{
	stack::HStack,
	text::Text,
	toggle::Toggle,
	trigger_button::{TriggerButton, TriggerButtonIntent},
	Widget,
};

struct Store {
	disabled: Mutable<bool>,
	show_label: Mutable<bool>,
}

impl Default for Store {
	fn default() -> Self {
		Self {
			disabled: Mutable::new(false),
			show_label: Mutable::new(false),
		}
	}
}

pub fn trigger_buttons() -> impl Widget {
	let store = Store::default();

	let reactive_text = Mutable::new("Press a button!");
	let text = Text::new_signal(reactive_text.signal());

	let clone_signal = reactive_text.clone();
	let button_delete = TriggerButton::new("Delete", "delete")
		.intent(TriggerButtonIntent::Danger)
		.disabled_signal(store.disabled.signal())
		.show_label_signal(store.show_label.signal())
		.on_press(move |_| clone_signal.set("Pressed Button 1"));

	let clone_signal = reactive_text.clone();
	let button2 = TriggerButton::new("Save", "save")
		.disabled_signal(store.disabled.signal())
		.show_label_signal(store.show_label.signal())
		.on_press(move |_| clone_signal.set("Pressed Button 2"));

	let clone_signal = reactive_text.clone();
	let button3 = TriggerButton::new("Save", "save")
		.intent(TriggerButtonIntent::Outlined)
		.disabled_signal(store.disabled.signal())
		.show_label_signal(store.show_label.signal())
		.on_press(move |_| clone_signal.set("Pressed Button 3"));

	let clone_signal = reactive_text.clone();
	let button4 = TriggerButton::new("Save", "save")
		.intent(TriggerButtonIntent::Active)
		.disabled_signal(store.disabled.signal())
		.show_label_signal(store.show_label.signal())
		.on_press(move |_| clone_signal.set("Pressed Button 4"));

	let switch_disabled = Toggle::new_signal(store.disabled.signal())
		.on_change(move |is_checked| store.disabled.set(is_checked));

	let switch_label = Toggle::new_signal(store.show_label.signal())
		.on_change(move |is_checked| store.show_label.set(is_checked));

	HStack::new()
		.gap(1)
		.child(&switch_disabled)
		.child(&switch_label)
		.child(&button_delete)
		.child(&button2)
		.child(&button3)
		.child(&button4)
		.child(&text)
}
