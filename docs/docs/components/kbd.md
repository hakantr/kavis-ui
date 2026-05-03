---
title: Klavye Tuşu
description: Kısayol tuşlarını görsel etiket olarak gösterir.
---

# Klavye Tuşu

Kısayol tuşlarını görsel etiket olarak gösterir. Güncel örnekler Kavis UI'nin Türkçe `KlavyeTusu` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `KlavyeTusu`

Ana tipler: `KlavyeTusu`.

## Kullanım

```rust
KlavyeTusu::new("cmd-k".parse().unwrap())
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/kbd_story.rs` dosyasına bakın.
