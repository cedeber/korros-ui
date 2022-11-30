use gloo::utils::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

pub(crate) fn create_element<T: JsCast>(tag: &str) -> T {
	document()
		.create_element(tag)
		.unwrap_throw()
		.dyn_into::<T>()
		.unwrap_throw()
}

pub(crate) fn create_svg_element<T: JsCast>(tag: &str) -> T {
	document()
		.create_element_ns(Some("http://www.w3.org/2000/svg"), tag)
		.unwrap_throw()
		.dyn_into::<T>()
		.unwrap_throw()
}
