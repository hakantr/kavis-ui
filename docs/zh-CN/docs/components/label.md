---
title: Etiket
description: Birincil ve ikincil metinleri, maskeleme ve vurgulama desteğiyle gösterir.
---

# Etiket

Birincil ve ikincil metinleri, maskeleme ve vurgulama desteğiyle gösterir. Güncel örnekler Kavis UI'nin Türkçe `Etiket` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Etiket`

Ana tipler: `Etiket`.

## Kullanım

```rust
Etiket::new("API anahtarı").secondary("zorunlu").highlights("API")
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/label_story.rs` dosyasına bakın.
