use futures_signals::signal::{Mutable, SignalExt};
use gloo::timers::callback::Interval;
use korros::widgets::{
	body::Body,
	fragment::Fragment,
	progress_circle::ProgressCircle,
	stack::{HStack, VStack},
	text::Text,
	toggle::Toggle,
};
use wasm_bindgen::prelude::*;

mod components;

#[wasm_bindgen(start)]
pub fn main_wasm() -> Result<(), JsValue> {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();

	let title = Text::new("Korros UI")
		.set_style("font-size", "80px")
		.set_style("font-weight", "100")
		.set_style("line-height", "1")
		.set_style("text-align", "center");

	let subtitle = Text::new("Accessible and Reactive User Interface")
		.set_style("font-size", "18px")
		.set_style("line-height", "1")
		.set_style("text-align", "center");
	let subtitle2 = Text::new("for Rust and WebAssembly Applications")
		.set_style("font-size", "18px")
		.set_style("line-height", "1")
		.set_style("text-align", "center");

	let header = VStack::new()
		.child(&title)
		.child(&subtitle)
		.child(&subtitle2)
		.padding(10, 10);

	let offset = Mutable::new(0.0);
	let progress = ProgressCircle::new(24.0, true);
	let progress2 = ProgressCircle::new(24.0, false).progress_signal(offset.signal());

	let offset_timeout = offset.clone();
	let timeout = Interval::new(40, move || {
		offset_timeout.set((offset_timeout.get() + 1.0) % 100.0);
	});
	timeout.forget();

	let text = Text::new_signal(offset.signal().map(|value| format!("{value}%")));
	let show = Mutable::new(true);
	let fragment = Fragment::new()
		.child(text.clone())
		.show_signal(show.signal());
	//	let fragment = Fragment::new().child(text.clone());
	let show_check = Toggle::new_signal(show.signal()).on_change(move |value| {
		show.set(value);
	});
	let show_stack = HStack::new().child(&show_check).child(&fragment);

	let final_stack = VStack::new()
		.child(&header)
		.child(&components::test())
		.child(&components::action_buttons())
		.child(&components::trigger_buttons())
		.child(&progress)
		.child(&progress2)
		.child(&show_stack)
		.child(&show_stack)
		.gap(20)
		.padding(10, 20);

	Body::new().child(&final_stack);

	Ok(())
}
