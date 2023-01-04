use web_sys::Node;

pub mod action_button;
pub mod body;
pub mod fragment;
pub mod icon;
pub mod progress_circle;
pub mod stack;
pub mod text;
pub mod toggle;
pub mod trigger_button;
pub mod visually_hidden;

pub trait Widget {
	fn render(&self) -> &Node;
}
