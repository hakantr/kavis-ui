---
title: Sheet
description: Kenar veya modal katman olarak açılan ikincil panel.
---

# Sheet

Kenar veya modal katman olarak açılan ikincil panel. Güncel örnekler Kavis UI'nin Türkçe `SayfaKatmani` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `SayfaKatmani`

Ana tipler: `SayfaKatmani`.

## Kullanım

```rust
SayfaKatmani::new(window, cx)
    .title("Ayrıntılar")
    .child(div().p_4().child("Panel içeriği"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/sheet_story.rs` dosyasına bakın.
