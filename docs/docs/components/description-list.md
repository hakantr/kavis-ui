---
title: Açıklama Listesi
description: Etiket ve değer çiftlerini düzenli biçimde gösterir.
---

# Açıklama Listesi

Etiket ve değer çiftlerini düzenli biçimde gösterir. Güncel örnekler Kavis UI'nin Türkçe `AciklamaListesi` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `AciklamaListesi`

Ana tipler: `AciklamaListesi`.

## Kullanım

```rust
AciklamaListesi::new()
    .item("Durum", "Aktif", 1)
    .item("Sürüm", "0.1.0", 1)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/description_list_story.rs` dosyasına bakın.
