use futures_signals::signal::{Mutable, SignalExt};
use korros::widgets::{
	action_button::{ActionButton, ActionButtonIntent},
	stack::{HStack, VStack},
	text::Text,
	toggle::Toggle,
	Widget,
};

pub fn test() -> impl Widget {
	let state = Mutable::new("Default");
	let state_bool = Mutable::new(false);

	let text1 = Text::new_signal(state.signal());
	let text2 = Text::new_signal(state_bool.signal_ref(|value| match value {
		true => "True",
		false => "False",
	}));
	let text3 = Text::new_signal(state.signal().map(|value| ["Hello,", value].join(" ")));

	let state_button = state_bool.clone();
	let button = ActionButton::new("Click me!")
		.intent(ActionButtonIntent::Primary)
		.on_press(move |_| {
			state.set("I changed the HTML text.");
			state_button.set(!state_button.get());
		})
		.loading_signal(state_bool.signal());

	let switch2 = Toggle::new_signal(state_bool.signal()).on_change(move |is_checked| {
		state_bool.set(is_checked);
	});

	let text_stack = VStack::new().child(&text1).child(&text2).child(&text3);

	HStack::new()
		.child(&switch2)
		.child(&button)
		.child(&text_stack)
}
