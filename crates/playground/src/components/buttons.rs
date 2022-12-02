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
		.intent(ButtonIntent::Danger)
		.disabled_signal(store.disabled.signal())
		.icon("delete");
	let button3 = Button::new("Gray")
		.intent(ButtonIntent::Gray)
		.disabled_signal(store.disabled.signal())
		.icon("refresh");
	let button4 = Button::new("Outlined")
		.intent(ButtonIntent::Outlined)
		.disabled_signal(store.disabled.signal());
	let button5 = Button::new("Plain")
		.intent(ButtonIntent::Plain)
		.disabled_signal(store.disabled.signal());
	let button6 = Button::new("Tinted")
		.intent(ButtonIntent::Tinted)
		.disabled_signal(store.disabled.signal());
	let button7 = Button::new("")
		.intent(ButtonIntent::Filled)
		.disabled_signal(store.disabled.signal())
		.icon("arrow_forward");

	let switch_disabled = Toggle::new_signal(store.disabled.signal())
		.on_change(move |is_checked| store.disabled.set(is_checked));

	HStack::new()
		.child(&switch_disabled)
		.child(&button7)
		.child(&button3)
		.child(&button4)
		.child(&button5)
		.child(&button6)
		.child(&button2)
}
