---
title: Takvim
description: Tarih seçimi için durumlu takvim bileşenidir.
---

# Takvim

Tarih seçimi için durumlu takvim bileşenidir. Güncel örnekler Kavis UI'nin Türkçe `TakvimDurumu`, `Takvim` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `TakvimDurumu`, `Takvim`

Ana tipler: `TakvimDurumu`, `Takvim`.

## Kullanım

```rust
let takvim = cx.new(|cx| TakvimDurumu::new(window, cx));
Takvim::new(&takvim)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/calendar_story.rs` dosyasına bakın.
