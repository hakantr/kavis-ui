# CLAUDE.md

Bu dosya, depoda çalışan kod asistanları için güncel proje notlarını içerir.

## Proje Özeti

Kavis UI, GPUI üzerinde masaüstü uygulamaları geliştirmek için hazırlanmış Rust UI bileşen kütüphanesidir.

- `crates/ui`: Ana `kavis-ui` crate'i.
- `crates/story`: Bileşen galerisi ve kapsamlı örnekler.
- `crates/story-web`: Galerinin WebAssembly sürümü.
- `crates/assets`: Varsayılan simge varlıkları.
- `crates/macros`: Prosedürel makrolar.
- `crates/webview`: Deneysel WebView desteği.
- `examples/`: Tekil özellik örnekleri.

## Güncel API Yüzeyi

Doküman ve örneklerde Türkçe adlar tercih edilir: `Dugme`, `DugmeGrubu`, `Gecis`, `KokGorunum`, `Tema`, `TemaRengi`, `EtkinTema`, `Simge`, `SimgeAdi`, `KavisMotoru`, `PencereAyarlari`, `Uygulama`, `Pencere` ve `Varlik`.

Kavis UI kullanılmadan önce `kavis_ui::init(cx)` çağrılmalıdır. Her pencerenin ilk görünümü `KokGorunum` olmalıdır.

```rust
use gpui::*;
use kavis_ui::*;

pub struct Ornek;

impl Render for Ornek {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child(Dugme::new("kaydet").primary().label("Kaydet"))
    }
}
```

## Komutlar

```bash
cargo run
cargo check
cargo fmt --check
cargo test --all
cargo run -p hello_world
cargo run --example editor
```

Web galerisi:

```bash
cd crates/story-web
make install
make dev
```

## Mimari Notlar

- `KokGorunum`, sheet, dialog, bildirim, popover ve klavye gezinimi katmanlarını bağlar.
- Tema global `Tema` üzerinden tutulur; `EtkinTema` ile `cx.theme()` kullanılır.
- `InputState`, Rope, LSP ve Tree-sitter metin altyapısının merkezindedir.
- Dock sistemi panel, split, sekme, sürükle-bırak ve yerleşim serileştirmeyi destekler.
- Durumsuz bileşenler `RenderOnce`, durumlu bileşenler `Entity<T>` state modeliyle kullanılır.

## Dokümantasyon

Markdown içerik Türkçedir. Ana dokümantasyon ağacı `docs/docs` altında tutulur.

## Upstream Birleştirme Notları

`../gpui-component` karşılaştırmalarına devam etmeden önce ilgili merge haritasını oku. 2026-05-04 uyarlaması için yol haritası `docs/gpui-component-2026-05-04-merge-map.md` dosyasındadır.
