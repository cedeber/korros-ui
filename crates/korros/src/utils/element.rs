use gloo::utils::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

pub(crate) fn create_element<T: JsCast>(tag: &str) -> T {
	document()
		.create_element(tag)
		.unwrap_throw()
		.dyn_into::<T>()
		.unwrap_throw()
}
