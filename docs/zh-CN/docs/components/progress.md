---
title: İlerleme
description: Çubuk veya dairesel ilerleme göstergesi.
---

# İlerleme

Çubuk veya dairesel ilerleme göstergesi. Güncel örnekler Kavis UI'nin Türkçe `Ilerleme`, `DaireselIlerleme` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Ilerleme`, `DaireselIlerleme`

Ana tipler: `Ilerleme`, `DaireselIlerleme`.

## Kullanım

```rust
Ilerleme::new("yukleme").value(72.0)
DaireselIlerleme::new("kurulum").value(45.0)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/progress_story.rs` dosyasına bakın.
