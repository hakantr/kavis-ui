---
title: Bildirim
description: Kök görünüm üzerinde geçici veya kalıcı bildirim gösterir.
---

# Bildirim

Kök görünüm üzerinde geçici veya kalıcı bildirim gösterir. Güncel örnekler Kavis UI'nin Türkçe `Bildirim`, `BildirimListesi` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `Bildirim`, `BildirimListesi`

Ana tipler: `Bildirim`, `BildirimListesi`.

## Kullanım

```rust
Bildirim::success("Ayarlar kaydedildi").title("Başarılı")
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/notification_story.rs` dosyasına bakın.
