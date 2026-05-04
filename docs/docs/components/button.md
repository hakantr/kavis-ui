---
title: Düğme
description: Tıklanabilir eylemler için varyant, boyut, simge ve yüklenme durumu destekleyen bileşendir.
---

# Düğme

Tıklanabilir eylemler için varyant, boyut, simge ve yüklenme durumu destekleyen bileşendir. Güncel örnekler Kavis UI'nin Türkçe `Dugme`, `DugmeGrubu`, `DugmeSimgesi` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `Dugme`, `DugmeGrubu`, `DugmeSimgesi`

Ana tipler: `Dugme`, `DugmeGrubu`, `DugmeSimgesi`.

## Kullanım

```rust
Dugme::new("kaydet")
    .primary()
    .label("Kaydet")
    .icon(SimgeAdi::Check)
    .on_click(|_, _, _| println!("Kaydedildi"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/button_story.rs` dosyasına bakın.
