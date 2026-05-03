---
title: Uyarı İletişim Kutusu
description: Kullanıcıdan onay isteyen modal iletişim kutusudur.
---

# Uyarı İletişim Kutusu

Kullanıcıdan onay isteyen modal iletişim kutusudur. Güncel örnekler Kavis UI'nin Türkçe `UyariIletisimKutusu` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `UyariIletisimKutusu`

Ana tipler: `UyariIletisimKutusu`.

## Kullanım

```rust
UyariIletisimKutusu::new(cx)
    .title("Kaydı sil")
    .description("Bu işlem geri alınamaz.")
    .on_ok(|_, _, _| println!("Onaylandı"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/alert_dialog_story.rs` dosyasına bakın.
