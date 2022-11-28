# Korros UI

## Goals

- Write Web application with Rust
- Swift UI, Flutter, Druid, Frui -like UI toolkit
- No JS(x) or CSS -like syntax or macro
- With Functional Reactive Programming
- Very opinionated Design and UX
- Fully accessible like react-aria

## Available Widgets

- Body
- Fragment
- VisuallyHidden
- Text
- HStack and VStack
- Icon

### Under development

- Toggle
- ProgressCircle

## Example

```rust
let state = Mutable::new("This is the default text.");

let text = Text::new_with_text_signal(state.signal());
let button = Button::new("Click me!")
    .with_intent(ButtonIntent::Filled)
    .on_press(move |_| {
        state.set("I changed the HTML text.");
    });
let stack = HStack::new().with_child(&button).with_child(&text);

Body::new().with_child(&stack);
```
