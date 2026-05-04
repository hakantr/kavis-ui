---
title: Uyarı
description: Sayfa içinde bilgi, başarı, uyarı veya hata mesajı gösterir.
---

# Uyarı

Sayfa içinde bilgi, başarı, uyarı veya hata mesajı gösterir. Güncel örnekler Kavis UI'nin Türkçe `Uyari` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `Uyari`

Ana tipler: `Uyari`.

## Kullanım

```rust
Uyari::success("kayit", "Ayarlar kaydedildi")
    .title("Başarılı")
    .on_close(|_, _, _| println!("Kapatıldı"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/alert_story.rs` dosyasına bakın.
