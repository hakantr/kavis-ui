---
title: Rozet
description: Bir öğenin üzerinde sayı, nokta veya simge rozeti gösterir.
---

# Rozet

Bir öğenin üzerinde sayı, nokta veya simge rozeti gösterir. Güncel örnekler Kavis UI'nin Türkçe `Rozet` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Rozet`

Ana tipler: `Rozet`.

## Kullanım

```rust
Rozet::new().count(8).child(SimgeAdi::Bell)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/badge_story.rs` dosyasına bakın.
