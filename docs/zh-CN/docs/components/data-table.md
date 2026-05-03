---
title: Veri Tablosu
description: Büyük veri kümeleri için temsilci tabanlı sanal tablo.
---

# Veri Tablosu

Büyük veri kümeleri için temsilci tabanlı sanal tablo. Güncel örnekler Kavis UI'nin Türkçe `TabloDurumu`, `VeriTablosu` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `TabloDurumu`, `VeriTablosu`

Ana tipler: `TabloDurumu`, `VeriTablosu`.

## Kullanım

```rust
let tablo = cx.new(|cx| TabloDurumu::new(temsilci, window, cx));
VeriTablosu::new(&tablo)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/data_table_story.rs` dosyasına bakın.
