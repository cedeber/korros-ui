use super::ViewComponent;
use crate::utils::element::create_element;
use futures_signals::signal::{Signal, SignalExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{Node, SvgsvgElement};

#[derive(Clone)]
pub struct ProgressCircle {
	container: SvgsvgElement,
}

impl ViewComponent for ProgressCircle {
	fn render(&self) -> &Node {
		&self.container
	}
}

impl ProgressCircle {
	pub fn new(size: u32) -> Self {
		let container: SvgsvgElement = create_element("svg");

		ProgressCircle { container }
	}

	pub fn with_progress_signal(self, signal: impl Signal<Item = u8> + 'static) -> Self {
		let clone = self.clone();
		let future = signal.for_each(move |value| {
			// clone.clone().set_text(&value.into());
			async {}
		});

		spawn_local(future);

		self
	}
}
