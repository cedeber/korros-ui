use web_sys::Node;

pub mod action_button;
pub mod body;
pub mod icon;
pub mod stack;
pub mod text;
pub mod toggle;

pub trait ViewComponent {
	fn render(&self) -> &Node;
}
