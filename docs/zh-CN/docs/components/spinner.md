---
title: Döner Gösterge
description: Yüklenme veya işlem devam ediyor durumunu gösterir.
---

# Döner Gösterge

Yüklenme veya işlem devam ediyor durumunu gösterir. Güncel örnekler Kavis UI'nin Türkçe `DonerGosterge` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `DonerGosterge`

Ana tipler: `DonerGosterge`.

## Kullanım

```rust
Dugme::new("senkron").icon(DonerGosterge::new()).label("Senkronize ediliyor")
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/spinner_story.rs` dosyasına bakın.
