use futures_signals::signal::Mutable;
use korros::widgets::{
	action_button::{ActionButton, ActionButtonIntent},
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

	let button_delete = ActionButton::new("Delete")
		.intent(ActionButtonIntent::Danger)
		.disabled_signal(store.disabled.signal())
		.left_icon("delete");
	let button_close = ActionButton::new("Close")
		.intent(ActionButtonIntent::Gray)
		.disabled_signal(store.disabled.signal())
		.left_icon("close");
	let button_outlined = ActionButton::new("Outlined")
		.intent(ActionButtonIntent::Outlined)
		.disabled_signal(store.disabled.signal());
	let button_plain = ActionButton::new("Plain")
		.intent(ActionButtonIntent::Plain)
		.disabled_signal(store.disabled.signal());
	let button_refresh = ActionButton::new("Refresh")
		.intent(ActionButtonIntent::Tinted)
		.left_icon("refresh")
		.loading_signal(store.disabled.signal())
		.disabled_signal(store.disabled.signal());
	let button_arrow_alone = ActionButton::new("")
		.aria_label("Next")
		.intent(ActionButtonIntent::Filled)
		.disabled_signal(store.disabled.signal())
		.left_icon("arrow_forward");
	let button_save = ActionButton::new("Save")
		.intent(ActionButtonIntent::Filled)
		.disabled_signal(store.disabled.signal());
	let button_cancel = ActionButton::new("Cancel")
		.intent(ActionButtonIntent::Gray)
		.disabled_signal(store.disabled.signal());
	let button_new_folder = ActionButton::new("New Folder")
		.intent(ActionButtonIntent::Tinted)
		.disabled_signal(store.disabled.signal());
	let button_delete_icon = ActionButton::new("")
		.aria_label("Delete")
		.intent(ActionButtonIntent::Danger)
		.disabled_signal(store.disabled.signal())
		.left_icon("delete");
	let button_next = ActionButton::new("Next")
		.intent(ActionButtonIntent::Filled)
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
