use futures_signals::signal::Mutable;
use korros::widgets::{
	action_button::{Button, ButtonIntent},
	stack::HStack,
	toggle::Toggle,
	ViewComponent,
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

pub fn buttons() -> impl ViewComponent {
	let store = Store::default();

	let button2 = Button::new("Danger!")
		.with_intent(ButtonIntent::Danger)
		.with_disabled_signal(store.disabled.signal())
		.with_icon("delete");
	let button3 = Button::new("Gray")
		.with_intent(ButtonIntent::Gray)
		.with_disabled_signal(store.disabled.signal())
		.with_icon("refresh");
	let button4 = Button::new("Outlined")
		.with_intent(ButtonIntent::Outlined)
		.with_disabled_signal(store.disabled.signal());
	let button5 = Button::new("Plain")
		.with_intent(ButtonIntent::Plain)
		.with_disabled_signal(store.disabled.signal());
	let button6 = Button::new("Tinted")
		.with_intent(ButtonIntent::Tinted)
		.with_disabled_signal(store.disabled.signal());
	let button7 = Button::new("")
		.with_intent(ButtonIntent::Filled)
		.with_disabled_signal(store.disabled.signal())
		.with_icon("arrow_forward");

	let switch_disabled = Toggle::new_with_checked_signal(store.disabled.signal())
		.with_change_callback(move |is_checked| store.disabled.set(is_checked));

	HStack::new()
		.with_child(&switch_disabled)
		.with_child(&button7)
		.with_child(&button3)
		.with_child(&button4)
		.with_child(&button5)
		.with_child(&button6)
		.with_child(&button2)
}
