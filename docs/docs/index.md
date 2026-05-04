---
title: Giriş
description: GPUI üzerinde Rust ile masaüstü uygulamaları geliştirmek için Kavis UI tanıtımı.
---

# Kavis UI'ye Giriş

Not: Kavis UI, [longbridge/gpui-component](https://github.com/longbridge/gpui-component) kütüphanesinin `0.5.1` sürümü baz alınarak oluşturulmuş sert bir çatalıdır; bu upstream kütüphane ile geriye doğru veya ileriye dönük senkronize, eş kodlama hedeflenmez. Kavis UI `0.1.0` bağımsız erken sürüm çizgisindedir; `docs.rs` ve `crates.io` yayınları kütüphane olgunlaştığında açılacaktır.

Kavis UI, [GPUI](https://gpui.rs) üzerinde modern masaüstü uygulamaları kurmak için hazırlanmış Rust bileşen kütüphanesidir. Bileşenler, tema sistemi, sanallaştırılmış liste ve tablo, Markdown/HTML içerik, grafikler ve kod editörü altyapısı sağlar.

## Hızlı Kurulum

```toml
[dependencies]
gpui = { path = "../zed/crates/gpui" }
gpui_platform = { path = "../zed/crates/gpui_platform", features = ["font-kit", "runtime_shaders", "screen-capture", "wayland", "x11"] }
kavis-ui = { git = "https://github.com/hakantr/kavis-ui" }
kavis-ui-assets = { git = "https://github.com/hakantr/kavis-ui" }
anyhow = "1"
```


## İlk Pencere

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


## Bağlantılar

- [GitHub deposu](https://github.com/hakantr/kavis-ui)
- [Katkı rehberi](https://github.com/hakantr/kavis-ui/blob/main/CONTRIBUTING.md)
- [Bileşenler](./components/index)
