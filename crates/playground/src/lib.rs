use futures_signals::signal::{Mutable, SignalExt};
use gloo::timers::callback::Interval;
use korros::widgets::progress_circle::ProgressCircle;
use korros::widgets::text::Text;
use korros::widgets::{body::Body, stack::VStack};
use wasm_bindgen::prelude::*;

mod components;

#[wasm_bindgen(start)]
pub fn main_wasm() -> Result<(), JsValue> {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();

	let offset = Mutable::new(0.0);
	let progress = ProgressCircle::new(24.0, true);
	let progress2 = ProgressCircle::new(24.0, false).with_progress_signal(offset.signal());

	let offset_timeout = offset.clone();
	let timeout = Interval::new(40, move || {
		offset_timeout.set((offset_timeout.get() + 1.0) % 100.0);
	});
	timeout.forget();

	let text = Text::new_with_text_signal(offset.signal().map(|value| format!("{value}%")));

	let final_stack = VStack::new()
		.with_child(&components::test())
		.with_child(&components::buttons())
		.with_child(&progress)
		.with_child(&progress2)
		.with_child(&text)
		.with_gap(20)
		.with_padding(10, 20);

	Body::new().with_child(&final_stack);

	Ok(())
}
