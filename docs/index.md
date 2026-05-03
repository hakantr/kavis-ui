---
layout: home
---

<script setup>
import Index from './index.vue'
</script>

<Index />

## Basit API

```rust
Dugme::new("kaydet")
    .primary()
    .label("Kaydet")
    .on_click(|_, _, _| println!("Kaydedildi"))
```

## Kurulum

```toml
[dependencies]
gpui = { path = "../zed/crates/gpui" }
gpui_platform = { path = "../zed/crates/gpui_platform", features = ["font-kit", "runtime_shaders", "screen-capture", "wayland", "x11"] }
kavis-ui = { git = "https://github.com/hakantr/kavis-ui" }
kavis-ui-assets = { git = "https://github.com/hakantr/kavis-ui" }
anyhow = "1"
```


## Merhaba Dünya

```rust
use gpui::*;
use kavis_ui::*;

pub struct Merhaba;

impl Render for Merhaba {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Merhaba, dünya!")
            .child(
                Dugme::new("basla")
                    .primary()
                    .label("Başla")
                    .on_click(|_, _, _| println!("Tıklandı!")),
            )
    }
}

fn main() {
    gpui_platform::application().run(move |cx| {
        kavis_ui::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| Merhaba);
                cx.new(|cx| KokGorunum::new(view, window, cx).bg(cx.theme().background))
            })
            .expect("Pencere açılamadı");
        })
        .detach();
    });
}
```


```bash
cargo run
```
