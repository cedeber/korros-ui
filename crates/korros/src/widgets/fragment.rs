use super::ViewComponent;
use gloo::utils::document;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{DocumentFragment, Node};

#[derive(Clone)]
pub struct Fragment {
	element: DocumentFragment,
}

impl ViewComponent for Fragment {
	fn render(&self) -> &Node {
		&self.element
	}
}

impl Default for Fragment {
	fn default() -> Self {
		Self::new()
	}
}

impl Fragment {
	pub fn new() -> Self {
		let element: DocumentFragment = document().create_document_fragment();
		Fragment { element }
	}

	pub fn with_child(self, element: &impl ViewComponent) -> Self {
		self.element.append_child(element.render()).unwrap_throw();

		self
	}
}
