# Kavis UI

> Bu dosya adı geriye dönük uyumluluk için korunur; içerik Türkçedir.


[![Derleme Durumu](https://github.com/hakantr/kavis-ui/actions/workflows/ci.yml/badge.svg)](https://github.com/hakantr/kavis-ui/actions/workflows/ci.yml) [![Docs](https://docs.rs/kavis-ui/badge.svg)](https://docs.rs/kavis-ui/) [![Crates.io](https://img.shields.io/crates/v/kavis-ui.svg)](https://crates.io/crates/kavis-ui)

Kavis UI, [GPUI](https://gpui.rs) üzerinde modern masaüstü uygulamaları geliştirmek için hazırlanmış Rust bileşen kütüphanesidir. Güncel API yüzeyi Türkçe adlandırılmış bileşenleri öne çıkarır: `Dugme`, `KokGorunum`, `Tema`, `Simge`, `SimgeAdi`, `KavisMotoru` ve ilgili state tipleri.

## Özellikler

- 60'tan fazla çapraz platform masaüstü UI bileşeni.
- macOS ve Windows kontrollerinden esinlenen modern masaüstü davranışı.
- `Tema`, `TemaRengi` ve `EtkinTema` ile merkezi tema yönetimi.
- Sanallaştırılmış `Liste`, `SanalListe`, `Tablo` ve `VeriTablosu` altyapısı.
- Markdown, basit HTML, Tree-sitter sözdizimi vurgulama ve LSP destekli editör bileşenleri.
- Dock, sekme, panel, sheet, dialog, popover ve bildirim katmanları.
- Bar, çizgi, alan, pasta ve mum grafik bileşenleri.

## Galeri

https://hakantr.github.io/kavis-ui/gallery/

## Kurulum

Kavis UI, GPUI'nin güncel Zed API'siyle geliştirilir. Workspace içi geliştirmede `../zed` path bağımlılığı kullanılır; uygulama tarafında git bağımlılığı tercih edilebilir.

```toml
[dependencies]
gpui = { path = "../zed/crates/gpui" }
gpui_platform = { path = "../zed/crates/gpui_platform", features = ["font-kit", "runtime_shaders", "screen-capture", "wayland", "x11"] }
kavis-ui = { git = "https://github.com/hakantr/kavis-ui" }
kavis-ui-assets = { git = "https://github.com/hakantr/kavis-ui" }
anyhow = "1"
```


`kavis-ui-assets` isteğe bağlıdır. Kendi SVG varlıklarınızı bağlayacaksanız özel `AssetSource` kullanabilirsiniz.

## Temel Örnek

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


## Geliştirme

```bash
cargo run
cargo check
cargo fmt --check
cargo test --all
```

Tekil örnek crate'leri:

```bash
cargo run -p hello_world
cargo run -p system_monitor
cargo run -p window_title
cargo run -p app_assets
```

Story crate içindeki örnekler:

```bash
cargo run --example editor
cargo run --example dock
cargo run --example markdown
cargo run --example html
```

Web galerisi:

```bash
cd crates/story-web
make install
make dev
```

## Karşılaştırma

| Özellik | Kavis UI | Iced | egui | Qt 6 |
| --- | --- | --- | --- | --- |
| Dil | Rust | Rust | Rust | C++/QML |
| Çizim altyapısı | GPUI | wgpu | wgpu | Qt |
| Lisans | Apache-2.0 | MIT | MIT/Apache-2.0 | Ticari/LGPL |
| Web | WASM | Var | Var | Var |
| Tema | Yerleşik | Var | Var | Var |
| Sanal tablo | Satır ve sütun | Yok | Satır | Satır ve sütun |
| Dock yerleşimi | Var | Var | Var | Var |
| Markdown | Var | Var | Temel | Yok |
| Sözdizimi vurgulama | Tree-sitter | Syntect | Syntect | QSyntaxHighlighter |

## Lisans

Apache-2.0
