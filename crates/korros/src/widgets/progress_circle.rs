use super::ViewComponent;
use crate::utils::{append_child, create_element, create_svg_element, set_attribute};
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

		set_attribute(&progress_element, "role", "progressbar");
		set_attribute(&progress_element, "aria-label", "Loading");
		set_attribute(&progress_element, "class", "korros__progress_circle");

		set_attribute(&loading, "aria-hidden", "true");
		set_attribute(&loading, "width", size_str);
		set_attribute(&loading, "height", size_str);
		set_attribute(&loading, "viewBox", &format!("0 0 {size_str} {size_str}"));
		set_attribute(&loading, "fill", "none");
		set_attribute(&loading, "stroke-width", &stroke_width.to_string());

		set_attribute(&background_circle, "cx", &center.to_string());
		set_attribute(&background_circle, "cy", &center.to_string());
		set_attribute(&background_circle, "r", &radius.to_string());
		set_attribute(&background_circle, "stroke", "currentColor");
		set_attribute(&background_circle, "opacity", "0.2");

		set_attribute(&progress_circle, "cx", &center.to_string());
		set_attribute(&progress_circle, "cy", &center.to_string());
		set_attribute(&progress_circle, "r", &radius.to_string());
		set_attribute(&progress_circle, "stroke", "currentColor");
		set_attribute(
			&progress_circle,
			"stroke-dasharray",
			&circumference.to_string(),
		);
		set_attribute(&progress_circle, "stroke-dashoffset", &offset.to_string());

		set_attribute(&transform, "attributeName", "transform");
		set_attribute(&transform, "type", "rotate");
		set_attribute(&transform, "begin", "0s");
		set_attribute(
			&transform,
			"dur",
			if is_indeterminate { "1s" } else { "0s" },
		);
		set_attribute(&transform, "from", &format!("90 {center} {center}"));
		set_attribute(&transform, "to", &format!("450 {center} {center}"));
		set_attribute(
			&transform,
			"repeatCount",
			if is_indeterminate { "indefinite" } else { "0" },
		);

		append_child(&progress_circle, &transform);
		append_child(&loading, &background_circle);
		append_child(&loading, &progress_circle);
		append_child(&progress_element, &loading);

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
