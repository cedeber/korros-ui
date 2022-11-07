use futures_signals::signal::{Mutable, SignalExt};
use wasm_bindgen::prelude::*;

use crate::widgets::{
	action_button::{Button, ButtonIntent},
	body::Body,
	icon::{Icon, IconSize},
	stack::{HStack, VStack},
	text::Text,
	toggle::Toggle,
};

mod widgets;

#[wasm_bindgen(start)]
pub fn main_wasm() -> Result<(), JsValue> {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();

	let state = Mutable::new("Default");
	let state_bool = Mutable::new(false);

	let text1 = Text::new_with_signal(state.signal());
	let text2 = Text::new_with_signal(state_bool.signal_ref(|value| match value {
		true => "True",
		false => "False",
	}));
	let text3 = Text::new_with_signal(state.signal().map(|value| ["Hello,", value].join(" ")));

	let state_button = state_bool.clone();
	let button = Button::new("Click me!")
		.with_intent(ButtonIntent::Filled)
		.with_disabled_signal(state_bool.signal())
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
	let switch = Toggle::new(false).on_change(move |is_checked| state_switch.set(is_checked));

	Body::new()
		.with_child(&h_stack)
		.with_child(&icon)
		.with_child(&switch);

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use wasm_bindgen_test::*;

	#[wasm_bindgen_test]
	fn it_works() {
		// assert_eq!(5, 3 + 2);
	}
}
