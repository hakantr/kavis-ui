---
title: DropdownButton
description: A DropdownButton is a combination of a button and a trigger button. It allows us to display a dropdown menu when the trigger is clicked, but the left Button can still respond to independent events.
---

# DropdownButton

A [DropdownButton] is a combination of a button and a trigger button. It allows us to display a dropdown menu when the trigger is clicked, but the left Button can still respond to independent events.

And more option methods of [Button] are also available for the DropdownButton, such as setting different variants using [ButtonCustomVariant], sizes using [Sizable], adding icons, loading states.

## Import

```rust
use kavis_ui::button::{Button, DropdownButton};
```

## Usage

```rust
use gpui::Anchor;

DropdownButton::new("dropdown")
    .button(Button::new("btn").label("Click Me"))
    .dropdown_menu(|menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
            .menu("Option 2", Box::new(MyAction))
            .separator()
            .menu("Option 3", Box::new(MyAction))
    })
```

### Variants

Same as [Button], DropdownButton supports different variants.

````rust
DropdownButton::new("dropdown")
    .primary()
    .button(Button::new("btn").label("Primary"))
    .dropdown_menu(|menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
    })
```

### With custom anchor

```rust
// With custom anchor
DropdownButton::new("dropdown")
    .button(Button::new("btn").label("Click Me"))
    .dropdown_menu_with_anchor(Anchor::BottomRight, |menu, _, _| {
        menu.menu("Option 1", Box::new(MyAction))
    })
````

[Button]: https://docs.rs/kavis-ui/latest/kavis_ui/button/struct.Button.html
[DropdownButton]: https://docs.rs/kavis-ui/latest/kavis_ui/button/struct.DropdownButton.html
[ButtonCustomVariant]: https://docs.rs/kavis-ui/latest/kavis_ui/button/struct.ButtonCustomVariant.html
[Sizable]: https://docs.rs/kavis-ui/latest/kavis_ui/trait.Sizable.html
