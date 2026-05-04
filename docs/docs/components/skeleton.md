---
title: İskelet
description: Yüklenme sırasında yer tutucu yüzey gösterir.
---

# İskelet

Yüklenme sırasında yer tutucu yüzey gösterir. Güncel örnekler Kavis UI'nin Türkçe `Iskelet` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `Iskelet`

Ana tipler: `Iskelet`.

## Kullanım

```rust
Iskelet::new().h_4().w_48()
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/skeleton_story.rs` dosyasına bakın.
