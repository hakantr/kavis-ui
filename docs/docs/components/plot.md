---
title: Plot
description: Düşük seviye çizim, eksen, grid, şekil ve araç ipucu altyapısıdır.
---

# Plot

Düşük seviye çizim, eksen, grid, şekil ve araç ipucu altyapısıdır. Güncel örnekler Kavis UI'nin Türkçe `PlotAxis`, `Grid`, `PlotLabel`, `AracIpucu` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `PlotAxis`, `Grid`, `PlotLabel`, `AracIpucu`

Ana tipler: `PlotAxis`, `Grid`, `PlotLabel`, `AracIpucu`.

## Kullanım

```rust
PlotAxis::new()
Grid::new()
AracIpucu::new().content(|_, _| div().child("Değer"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/plot_story.rs` dosyasına bakın.
