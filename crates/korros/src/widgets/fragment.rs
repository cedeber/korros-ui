use super::Widget;
use futures_signals::signal::{Signal, SignalExt};
use gloo::utils::document;
use std::sync::{Arc, Mutex};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Comment, DocumentFragment, Node};

struct FragmentState {
	children: Vec<Box<dyn Widget>>,
	show: bool,
}

#[derive(Clone)]
pub struct Fragment {
	element: DocumentFragment,
	comment: Comment,
	state: Arc<Mutex<FragmentState>>,
}

impl Widget for Fragment {
	fn render(&self) -> &Node {
		// In the DOM tree, the document fragment is replaced by all its children.
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
		let element = document().create_document_fragment();
		let comment = Comment::new().unwrap_throw();

		element.append_child(&comment).unwrap_throw();

		Fragment {
			element,
			comment,
			state: Arc::new(Mutex::new(FragmentState {
				children: Vec::new(),
				show: true,
			})),
		}
	}

	pub fn new_hidden() -> Self {
		let element = document().create_document_fragment();
		let comment = Comment::new().unwrap_throw();

		element.append_child(&comment).unwrap_throw();

		Fragment {
			element,
			comment,
			state: Arc::new(Mutex::new(FragmentState {
				children: Vec::new(),
				show: false,
			})),
		}
	}

	pub fn child(self, element: impl Widget + Clone + 'static) -> Self {
		let state = Arc::clone(&self.state);
		let mut data = state.lock().unwrap_throw();

		data.children.push(Box::new(element.clone()));

		if data.show {
			self.element.append_child(element.render()).unwrap_throw();
		}

		self
	}

	pub fn show_signal(self, signal: impl Signal<Item = bool> + 'static) -> Self {
		let clone = self.clone();
		let state = Arc::clone(&self.state);

		let future = signal.for_each(move |value| {
			let mut data = state.lock().unwrap_throw();

			if value {
				if data.show != value {
					// add content again
					let fragment = document().create_document_fragment();

					for child in &data.children {
						fragment.append_child(child.render()).unwrap_throw();
					}

					clone.comment.after_with_node_1(&fragment).unwrap_throw();
				}
			} else {
				for child in &data.children {
					let node = child.render();
					if let Some(parent) = node.parent_element() {
						parent.remove_child(node).unwrap_throw();
					}
				}
			}

			data.show = value;
			async {}
		});

		spawn_local(future);

		self
	}
}
