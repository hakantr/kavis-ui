# Kavis UI'de Simge Varlıkları

`Simge` ve `SimgeAdi` SVG dosyalarını GPUI varlık kaynağından okur. Ana `kavis-ui` crate'i bütün simgeleri varsayılan olarak gömmez; uygulama kendi `AssetSource` tipini kaydedebilir veya `kavis-ui-assets` crate'ini kullanabilir.

## Klasör Düzeni

```text
app_root
  assets
    icons
      bot.svg
      inbox.svg
  src
    main.rs
  Cargo.toml
```

## Özel Varlık Kaynağı

```rust
use anyhow::anyhow;
use gpui::*;
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
pub struct Varliklar;

impl AssetSource for Varliklar {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("varlık bulunamadı: {path}"))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}
```

## Uygulamada Kullanım

```rust
let app = gpui_platform::application().with_assets(Varliklar);
let hazir_paket = gpui_platform::application().with_assets(kavis_ui_assets::Assets);
```
