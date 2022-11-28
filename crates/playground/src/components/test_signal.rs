use futures_signals::signal::{Mutable, SignalExt};
use korros::widgets::{
	action_button::{Button, ButtonIntent},
	stack::{HStack, VStack},
	text::Text,
	toggle::Toggle,
	ViewComponent,
};

pub fn test() -> impl ViewComponent {
	let state = Mutable::new("Default");
	let state_bool = Mutable::new(false);

	let text1 = Text::new_with_text_signal(state.signal());
	let text2 = Text::new_with_text_signal(state_bool.signal_ref(|value| match value {
		true => "True",
		false => "False",
	}));
	let text3 = Text::new_with_text_signal(state.signal().map(|value| ["Hello,", value].join(" ")));

	let state_button = state_bool.clone();
	let button = Button::new("Click me!")
		.with_intent(ButtonIntent::Filled)
		.on_press(move |_| {
			state.set("I changed the HTML text.");
			state_button.set(!state_button.get());
		});

	let state_switch2 = state_bool.clone();
	let switch2 = Toggle::new_with_checked_signal(state_switch2.signal()).with_change_callback(
		move |is_checked| {
			state_switch2.set(is_checked);
		},
	);

	let text_stack = VStack::new()
		.with_child(&text1)
		.with_child(&text2)
		.with_child(&text3);

	HStack::new()
		.with_child(&switch2)
		.with_child(&button)
		.with_child(&text_stack)
}
