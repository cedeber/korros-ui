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

	let button_delete = Button::new("Delete")
		.intent(ButtonIntent::Danger)
		.disabled_signal(store.disabled.signal())
		.left_icon("delete");
	let button_close = Button::new("Close")
		.intent(ButtonIntent::Gray)
		.disabled_signal(store.disabled.signal())
		.left_icon("close");
	let button_outlined = Button::new("Outlined")
		.intent(ButtonIntent::Outlined)
		.disabled_signal(store.disabled.signal());
	let button_plain = Button::new("Plain")
		.intent(ButtonIntent::Plain)
		.disabled_signal(store.disabled.signal());
	let button_refresh = Button::new("Refresh")
		.intent(ButtonIntent::Tinted)
		.left_icon("refresh")
		.loading_signal(store.disabled.signal())
		.disabled_signal(store.disabled.signal());
	let button_arrow_alone = Button::new("")
		.aria_label("Next")
		.intent(ButtonIntent::Filled)
		.disabled_signal(store.disabled.signal())
		.left_icon("arrow_forward");
	let button_save = Button::new("Save")
		.intent(ButtonIntent::Filled)
		.disabled_signal(store.disabled.signal());
	let button_cancel = Button::new("Cancel")
		.intent(ButtonIntent::Gray)
		.disabled_signal(store.disabled.signal());
	let button_new_folder = Button::new("New Folder")
		.intent(ButtonIntent::Tinted)
		.disabled_signal(store.disabled.signal());
	let button_delete_icon = Button::new("")
		.aria_label("Delete")
		.intent(ButtonIntent::Danger)
		.disabled_signal(store.disabled.signal())
		.left_icon("delete");
	let button_next = Button::new("Next")
		.intent(ButtonIntent::Filled)
		.disabled_signal(store.disabled.signal())
		.right_icon("arrow_forward");

	let switch_disabled = Toggle::new_signal(store.disabled.signal())
		.on_change(move |is_checked| store.disabled.set(is_checked));

	HStack::new()
		.child(&switch_disabled)
		.child(&button_arrow_alone)
		.child(&button_next)
		.child(&button_save)
		.child(&button_refresh)
		.child(&button_new_folder)
		.child(&button_close)
		.child(&button_cancel)
		.child(&button_delete)
		.child(&button_delete_icon)
		.child(&button_outlined)
		.child(&button_plain)
}
