use super::ViewComponent;
use crate::utils::element::{create_element, create_svg_element};
use futures_signals::signal::{Signal, SignalExt};
use std::f32::consts::PI;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlDivElement, Node, SvgAnimateTransformElement, SvgCircleElement, SvgsvgElement};

#[derive(Clone)]
pub struct ProgressCircle {
	container: HtmlDivElement,
	progress_circle: SvgCircleElement,
	circumference: f32,
}

impl ViewComponent for ProgressCircle {
	fn render(&self) -> &Node {
		&self.container
	}
}

impl ProgressCircle {
	// TODO Extract size and is_indeternimate. Add label instead.
	pub fn new(size: f32, is_indeterminate: bool) -> Self {
		let progress_element: HtmlDivElement = create_element("div");
		let loading: SvgsvgElement = create_svg_element("svg");
		let background_circle: SvgCircleElement = create_svg_element("circle");
		let progress_circle: SvgCircleElement = create_svg_element("circle");
		let transform: SvgAnimateTransformElement = create_svg_element("animateTransform");

		let progress = if is_indeterminate { 30.0 } else { 0.0 };
		let center = size / 2.0;
		let stroke_width = 2.0;
		let radius = center - stroke_width;
		let circumference = 2.0 * radius * PI;
		let offset = circumference - ((progress % 100.0) / 100.0) * circumference;

		let size_str = &(center * 2.0).to_string();

		progress_element
			.set_attribute("role", "progressbar")
			.unwrap_throw();
		progress_element
			.set_attribute("aria-label", "Loading")
			.unwrap_throw();
		progress_element
			.set_attribute("class", "korros__progress_circle")
			.unwrap_throw();

		loading.set_attribute("aria-hidden", "true").unwrap_throw();
		loading.set_attribute("width", size_str).unwrap_throw();
		loading.set_attribute("height", size_str).unwrap_throw();
		loading
			.set_attribute("viewBox", &format!("0 0 {size_str} {size_str}"))
			.unwrap_throw();
		loading.set_attribute("fill", "none").unwrap_throw();
		loading
			.set_attribute("stroke-width", &stroke_width.to_string())
			.unwrap_throw();

		background_circle
			.set_attribute("cx", &center.to_string())
			.unwrap_throw();
		background_circle
			.set_attribute("cy", &center.to_string())
			.unwrap_throw();
		background_circle
			.set_attribute("r", &radius.to_string())
			.unwrap_throw();
		background_circle
			.set_attribute("stroke", "currentColor")
			.unwrap_throw();
		background_circle
			.set_attribute("opacity", "0.2")
			.unwrap_throw();

		progress_circle
			.set_attribute("cx", &center.to_string())
			.unwrap_throw();
		progress_circle
			.set_attribute("cy", &center.to_string())
			.unwrap_throw();
		progress_circle
			.set_attribute("r", &radius.to_string())
			.unwrap_throw();
		progress_circle
			.set_attribute("stroke", "currentColor")
			.unwrap_throw();
		progress_circle
			.set_attribute("stroke-dasharray", &circumference.to_string())
			.unwrap_throw();
		progress_circle
			.set_attribute("stroke-dashoffset", &offset.to_string())
			.unwrap_throw();

		transform
			.set_attribute("attributeName", "transform")
			.unwrap_throw();
		transform.set_attribute("type", "rotate").unwrap_throw();
		transform.set_attribute("begin", "0s").unwrap_throw();
		transform
			.set_attribute("dur", if is_indeterminate { "1s" } else { "0s" })
			.unwrap_throw();
		transform
			.set_attribute("from", &format!("90 {center} {center}"))
			.unwrap_throw();
		transform
			.set_attribute("to", &format!("450 {center} {center}"))
			.unwrap_throw();
		transform
			.set_attribute(
				"repeatCount",
				if is_indeterminate { "indefinite" } else { "0" },
			)
			.unwrap_throw();

		progress_circle.append_child(&transform).unwrap_throw();

		loading.append_child(&background_circle).unwrap_throw();
		loading.append_child(&progress_circle).unwrap_throw();

		progress_element.append_child(&loading).unwrap_throw();

		ProgressCircle {
			container: progress_element,
			progress_circle,
			circumference,
		}
	}

	pub fn progress_signal(self, signal: impl Signal<Item = f32> + 'static) -> Self {
		let clone = self.clone();
		let future = signal.for_each(move |value| {
			let circle_clone = clone.clone();
			let offset =
				circle_clone.circumference - ((value % 100.0) / 100.0) * circle_clone.circumference;

			circle_clone
				.progress_circle
				.set_attribute("stroke-dashoffset", &offset.to_string())
				.unwrap_throw();

			circle_clone
				.container
				.set_attribute("aria-valuenow", &value.to_string())
				.unwrap_throw();

			async {}
		});

		spawn_local(future);

		self
	}
}
