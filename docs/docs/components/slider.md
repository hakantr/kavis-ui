---
title: Kaydırıcı
description: Sayısal değerleri sürüklenebilir kontrolle seçtirir.
---

# Kaydırıcı

Sayısal değerleri sürüklenebilir kontrolle seçtirir. Güncel örnekler Kavis UI'nin Türkçe `KaydiriciDurumu`, `Kaydirici` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `KaydiriciDurumu`, `Kaydirici`

Ana tipler: `KaydiriciDurumu`, `Kaydirici`.

## Kullanım

```rust
let state = cx.new(|_| KaydiriciDurumu::new().default_value(42.0));
Kaydirici::new(&state)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/slider_story.rs` dosyasına bakın.
