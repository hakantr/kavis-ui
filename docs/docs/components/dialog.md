---
title: İletişim Kutusu
description: Modal içerik, başlık, açıklama ve eylemler için kullanılır.
---

# İletişim Kutusu

Modal içerik, başlık, açıklama ve eylemler için kullanılır. Güncel örnekler Kavis UI'nin Türkçe `IletisimKutusu` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `IletisimKutusu`

Ana tipler: `IletisimKutusu`.

## Kullanım

```rust
IletisimKutusu::new(cx)
    .title("Ayarlar")
    .content(|_, _| div().child("İçerik"))
    .on_ok(|_, _, _| println!("Tamam"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/dialog_story.rs` dosyasına bakın.
