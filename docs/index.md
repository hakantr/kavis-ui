---
layout: home
---

<script setup>
import Index from './index.vue'
</script>

<Index />

## Simple and Intuitive API

Get started with just a few lines of code. Stateless components
make it easy to build complex UIs.

```rs
Button::new("ok")
    .primary()
    .label("Click Me")
    .on_click(|_, _, _| println!("Button clicked!"))
```

## Install Kavis UI

Add the following to your `Cargo.toml`:

GPUI and Kavis UI are under active development, recently GPUI have some new features not published on crates.io, so we recommend using the git version for now.

The documentation on this site are based on the **Git main branch**, if you use the crates.io version, there may be some differences.

```toml
gpui = { path = "../zed/crates/gpui" }
gpui_platform = { path = "../zed/crates/gpui_platform", features = ["font-kit", "runtime_shaders", "screen-capture", "wayland", "x11"] }
kavis-ui = { git = "https://github.com/hakantr/kavis-ui" }
```

## Hello World

The following `src/main.rs` is a simple "Hello, World!" application:

```rs
use gpui::*;
use kavis_ui::{button::*, *};

pub struct HelloWorld;
impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }
}

fn main() {
    gpui_platform::application().run(move |cx| {
        // This must be called before using any Kavis UI features.
        kavis_ui::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| HelloWorld);
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
```

Run the program with the following command:

```sh
$ cargo run
```
