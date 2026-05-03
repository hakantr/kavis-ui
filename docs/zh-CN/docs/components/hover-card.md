---
title: Üzerine Gelme Kartı
description: İmleç öğenin üzerindeyken bağlamsal içerik gösterir.
---

# Üzerine Gelme Kartı

İmleç öğenin üzerindeyken bağlamsal içerik gösterir. Güncel örnekler Kavis UI'nin Türkçe `UzerineGelmeKarti` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `UzerineGelmeKarti`

Ana tipler: `UzerineGelmeKarti`.

## Kullanım

```rust
UzerineGelmeKarti::new("profil")
    .trigger(Avatar::new().name("Ayşe Demir"))
    .content(|_, _| div().child("Profil özeti"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/hover_card_story.rs` dosyasına bakın.
