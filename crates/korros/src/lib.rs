use futures_signals::signal::{Mutable, SignalExt};
use log::{info, Level};
use wasm_bindgen::prelude::*;

use crate::widgets::{
	action_button::{Button, ButtonIntent},
	body::Body,
	stack::{HStack, VStack},
	text::Text,
};

mod widgets;

#[wasm_bindgen(start)]
pub fn main_wasm() -> Result<(), JsValue> {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();

	#[cfg(feature = "console_log")]
	console_log::init_with_level(Level::Trace).expect("error initializing log");

	info!("Hello, World!");

	let state = Mutable::new("Default");
	let state_bool = Mutable::new(false);

	let text1 = Text::signal(state.signal());
    let text2 = Text::signal(state_bool.signal_ref(|value| match value {
		true => "True",
		false => "False",
	}));
    let text3 = Text::signal(state.signal().map(|value| value));

	let button = Button::new("Click me!")
		.on_press(move |_| {
			state.set("I changed the HTML text.");
			state_bool.set(true)
		})
		.with_intent(ButtonIntent::Filled);

	let v_stack = VStack::new().with_child(&text1).with_child(&text2).with_child(&text3);

	let h_stack = HStack::new()
		.with_child(&button)
		.with_child(&v_stack)
		.with_gap(20)
		.with_padding(10, 20);

	Body::new().with_child(&h_stack);

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
