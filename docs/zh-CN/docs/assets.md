---
title: Simgeler ve Varlıklar
description: Kavis UI simge varlıklarını uygulamaya bağlama.
---

# Simgeler ve Varlıklar

`Simge` bileşeni SVG dosyalarını GPUI varlık kaynağından okur. Hazır paket için `kavis-ui-assets`, özel paket için kendi `AssetSource` uygulamanızı kullanın.

```rust
let app = gpui_platform::application().with_assets(kavis_ui_assets::Assets);
```

Özel kaynak örneği:

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

`SimgeAdi::Inbox` gibi adlar `assets/icons/inbox.svg` dosyasıyla eşleşmelidir.
