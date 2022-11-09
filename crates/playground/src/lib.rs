use futures_signals::signal::{Mutable, SignalExt};
use korros::widgets::{
	action_button::{Button, ButtonIntent},
	body::Body,
	icon::{Icon, IconSize},
	stack::{HStack, VStack},
	text::Text,
	toggle::Toggle,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_wasm() -> Result<(), JsValue> {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();

	let state = Mutable::new("Default");
	let state_bool = Mutable::new(false);

	let text1 = Text::new_with_text_signal(state.signal());
	let text2 = Text::new_with_text_signal(state_bool.signal_ref(|value| match value {
		true => "True",
		false => "False",
	}));
	let text3 = Text::new_with_text_signal(state.signal().map(|value| ["Hello,", value].join(" ")));

	let state_button = state_bool.clone();
	let button = Button::new("Click me!")
		.with_intent(ButtonIntent::Filled)
		// .with_disabled_signal(state_bool.signal())
		.on_press(move |_| {
			state.set("I changed the HTML text.");
			state_button.set(!state_button.get())
		});

	let v_stack = VStack::new()
		.with_child(&text1)
		.with_child(&text2)
		.with_child(&text3);

	let h_stack = HStack::new()
		.with_child(&button)
		.with_child(&v_stack)
		.with_gap(20)
		.with_padding(10, 20);

	let icon = Icon::new("delete").with_size(IconSize::Small);

	let state_switch = state_bool.clone();
	let state_switch2 = state_bool.clone();
	let switch = Toggle::new(true, true)
		.with_change_callback(move |is_checked| state_switch.set(is_checked));
	let switch2 = Toggle::new_with_checked_signal(state_switch2.signal()).with_change_callback(
		move |is_checked| {
			state_switch2.set(is_checked);
		},
	);

	Body::new()
		.with_child(&h_stack)
		.with_child(&icon)
		.with_child(&switch)
		.with_child(&switch2);

	Ok(())
}
