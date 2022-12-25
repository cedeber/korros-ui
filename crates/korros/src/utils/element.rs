use gloo::utils::document;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Element, HtmlElement, Node};

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

pub(crate) fn set_attribute(element: &Element, name: &str, value: &str) {
	element.set_attribute(name, value).unwrap_throw();
}

pub(crate) fn set_bool_attribute(element: &Element, name: &str) {
	element.set_attribute(name, "").unwrap_throw();
}

pub(crate) fn remove_attribute(element: &Element, name: &str) {
	element.remove_attribute(name).unwrap_throw();
}

pub(crate) fn append_child(parent: &Element, child: &Node) {
	parent.append_child(child).unwrap_throw();
}

pub(crate) fn remove_child(parent: &Element, child: &Node) {
	parent.remove_child(child).unwrap_throw();
}

pub(crate) fn set_style(element: &HtmlElement, name: &str, value: &str) {
	element.style().set_property(name, value).unwrap_throw();
}
