use web_sys::Node;

pub mod action_button;
pub mod body;
pub mod stack;
pub mod text;

pub trait ViewComponent {
	fn get(&self) -> &Node;
}
