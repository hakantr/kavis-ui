---
title: Anahtar
description: Açık/kapalı ayarları için anahtar bileşeni.
---

# Anahtar

Açık/kapalı ayarları için anahtar bileşeni. Güncel örnekler Kavis UI'nin Türkçe `Anahtar` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Anahtar`

Ana tipler: `Anahtar`.

## Kullanım

```rust
Anahtar::new("koyu-mod").label("Koyu mod").checked(true)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/switch_story.rs` dosyasına bakın.
