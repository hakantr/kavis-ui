---
title: Grafik
description: Bar, çizgi, alan, pasta ve mum grafik bileşenlerini kapsar.
---

# Grafik

Bar, çizgi, alan, pasta ve mum grafik bileşenlerini kapsar. Güncel örnekler Kavis UI'nin Türkçe `BarChart`, `LineChart`, `AreaChart`, `PieChart`, `CandlestickChart` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `BarChart`, `LineChart`, `AreaChart`, `PieChart`, `CandlestickChart`

Ana tipler: `BarChart`, `LineChart`, `AreaChart`, `PieChart`, `CandlestickChart`.

## Kullanım

```rust
let veri = vec![("Ocak", 12.0), ("Şubat", 18.0)];
BarChart::new(veri).label(|item| item.0).value(|item| item.1)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/chart_story.rs` dosyasına bakın.
