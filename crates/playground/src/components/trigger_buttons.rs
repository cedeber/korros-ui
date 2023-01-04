use futures_signals::signal::Mutable;
use korros::widgets::{
	stack::HStack,
	toggle::Toggle,
	trigger_button::{TriggerButton, TriggerButtonIntent},
	Widget,
};

struct Store {
	disabled: Mutable<bool>,
}

impl Default for Store {
	fn default() -> Self {
		Self {
			disabled: Mutable::new(false),
		}
	}
}

pub fn trigger_buttons() -> impl Widget {
	let store = Store::default();

	let button_delete = TriggerButton::new("Delete", "delete")
		.intent(TriggerButtonIntent::Danger)
		.disabled_signal(store.disabled.signal());

	let button2 = TriggerButton::new("Save", "save").disabled_signal(store.disabled.signal());

	let button3 = TriggerButton::new("Save", "save")
		.intent(TriggerButtonIntent::Outlined)
		.disabled_signal(store.disabled.signal());

	let button4 = TriggerButton::new("Save", "save")
		.intent(TriggerButtonIntent::Active)
		.disabled_signal(store.disabled.signal());

	let switch_disabled = Toggle::new_signal(store.disabled.signal())
		.on_change(move |is_checked| store.disabled.set(is_checked));

	HStack::new()
		.gap(1)
		.child(&switch_disabled)
		.child(&button_delete)
		.child(&button2)
		.child(&button3)
		.child(&button4)
}
