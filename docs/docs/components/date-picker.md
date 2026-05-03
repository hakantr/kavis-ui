---
title: Tarih Seçici
description: Tek tarih veya tarih aralığı seçimi için durumlu bileşen.
---

# Tarih Seçici

Tek tarih veya tarih aralığı seçimi için durumlu bileşen. Güncel örnekler Kavis UI'nin Türkçe `TarihSeciciDurumu`, `TarihSecici` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `TarihSeciciDurumu`, `TarihSecici`

Ana tipler: `TarihSeciciDurumu`, `TarihSecici`.

## Kullanım

```rust
let tarih = cx.new(|cx| TarihSeciciDurumu::new(window, cx));
TarihSecici::new(&tarih)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/date_picker_story.rs` dosyasına bakın.
