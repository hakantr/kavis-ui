---
title: Akordeon
description: İlişkili içerikleri açılır kapanır bölümler halinde gösterir.
---

# Akordeon

İlişkili içerikleri açılır kapanır bölümler halinde gösterir. Güncel örnekler Kavis UI'nin Türkçe `Akordeon`, `AkordeonOgesi` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `Akordeon`, `AkordeonOgesi`

Ana tipler: `Akordeon`, `AkordeonOgesi`.

## Kullanım

```rust
Akordeon::new("sss")
    .item(|item| item.title("Kavis UI nedir?").open(true).child("GPUI tabanlı Rust UI kütüphanesi."))
    .item(|item| item.title("Tema var mı?").child("Evet."))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/accordion_story.rs` dosyasına bakın.
