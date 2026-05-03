---
title: Puanlama
description: Yıldız benzeri puan seçimi gösterir.
---

# Puanlama

Yıldız benzeri puan seçimi gösterir. Güncel örnekler Kavis UI'nin Türkçe `Puanlama` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Puanlama`

Ana tipler: `Puanlama`.

## Kullanım

```rust
Puanlama::new("puan").max(5).value(4)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/rating_story.rs` dosyasına bakın.
