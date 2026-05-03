---
title: Simge
description: Varlık kaynağından SVG simgeleri çizer.
---

# Simge

Varlık kaynağından SVG simgeleri çizer. Güncel örnekler Kavis UI'nin Türkçe `Simge`, `SimgeAdi` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Simge`, `SimgeAdi`

Ana tipler: `Simge`, `SimgeAdi`.

## Kullanım

```rust
Simge::new(SimgeAdi::Search).small()
SimgeAdi::Inbox
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/icon_story.rs` dosyasına bakın.
