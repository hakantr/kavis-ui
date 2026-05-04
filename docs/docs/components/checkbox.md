---
title: Onay Kutusu
description: İkili seçimler için etiketli onay kutusu.
---

# Onay Kutusu

İkili seçimler için etiketli onay kutusu. Güncel örnekler Kavis UI'nin Türkçe `OnayKutusu` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `OnayKutusu`

Ana tipler: `OnayKutusu`.

## Kullanım

```rust
OnayKutusu::new("bildirimler")
    .label("Bildirimleri aç")
    .checked(true)
    .on_click(|checked, _, _| println!("Durum: {checked}"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/checkbox_story.rs` dosyasına bakın.
