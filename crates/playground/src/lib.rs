use korros::widgets::{body::Body, stack::VStack};
use wasm_bindgen::prelude::*;

mod components;

#[wasm_bindgen(start)]
pub fn main_wasm() -> Result<(), JsValue> {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();

	let final_stack = VStack::new()
		.with_child(&components::test())
		.with_child(&components::buttons())
		.with_gap(20)
		.with_padding(10, 20);

	Body::new().with_child(&final_stack);

	Ok(())
}
