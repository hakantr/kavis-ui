---
title: Radyo
description: Tekil seçimler ve radyo grupları için kullanılır.
---

# Radyo

Tekil seçimler ve radyo grupları için kullanılır. Güncel örnekler Kavis UI'nin Türkçe `Radyo`, `RadyoGrubu` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Radyo`, `RadyoGrubu`

Ana tipler: `Radyo`, `RadyoGrubu`.

## Kullanım

```rust
RadyoGrubu::new("tema")
    .children([Radyo::new("acik").label("Açık"), Radyo::new("koyu").label("Koyu")])
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/radio_story.rs` dosyasına bakın.
