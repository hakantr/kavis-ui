---
title: Pano
description: Metni panoya kopyalayan eylem bileşeni.
---

# Pano

Metni panoya kopyalayan eylem bileşeni. Güncel örnekler Kavis UI'nin Türkçe `Pano` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Pano`

Ana tipler: `Pano`.

## Kullanım

```rust
Pano::new("api-anahtari")
    .value("kavis-demo-key")
    .on_copied(|_, _| println!("Kopyalandı"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/clipboard_story.rs` dosyasına bakın.
