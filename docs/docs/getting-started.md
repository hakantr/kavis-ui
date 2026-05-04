---
title: Başlarken
description: Kavis UI'yi projeye ekleme ve ilk pencereyi açma.
order: -2
---

# Başlarken

## Kurulum

```toml
[dependencies]
gpui = { path = "../zed/crates/gpui" }
gpui_platform = { path = "../zed/crates/gpui_platform", features = ["font-kit", "runtime_shaders", "screen-capture", "wayland", "x11"] }
kavis-ui = { git = "https://github.com/hakantr/kavis-ui" }
kavis-ui-assets = { git = "https://github.com/hakantr/kavis-ui" }
anyhow = "1"
```


:::tip
`kavis-ui-assets` isteğe bağlıdır. Kendi simge paketiniz varsa özel `AssetSource` kullanabilirsiniz.
:::

## İlk Uygulama

```rust
use kavis_ui::ham_gpui::*;
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
    kavis_ui::platform::application().run(move |cx| {
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


:::info
`kavis_ui::init(cx)` çağrısı uygulama başında yapılmalıdır. Tema, dialog, sheet, popover, input ve benzeri global sistemler bu çağrıyla kurulur.
:::

## Durumsuz Öğe

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;

struct Panel;

impl Render for Panel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .child(Dugme::new("kaydet").primary().label("Kaydet"))
            .child(Cip::secondary().child("Hazır"))
    }
}
```

## Durumlu Bileşen

```rust
struct FormOrnegi {
    input: Entity<InputState>,
}

impl FormOrnegi {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx).default_value("Merhaba"));
        Self { input }
    }
}

impl Render for FormOrnegi {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Input::new(&self.input)
    }
}
```
