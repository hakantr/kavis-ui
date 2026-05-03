---
title: Adımlayıcı
description: Çok adımlı süreçlerin mevcut aşamasını gösterir.
---

# Adımlayıcı

Çok adımlı süreçlerin mevcut aşamasını gösterir. Güncel örnekler Kavis UI'nin Türkçe `Adimlayici`, `AdimlayiciOgesi` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Adimlayici`, `AdimlayiciOgesi`

Ana tipler: `Adimlayici`, `AdimlayiciOgesi`.

## Kullanım

```rust
Adimlayici::new("kurulum")
    .items([
        AdimlayiciOgesi::new().child("Hesap"),
        AdimlayiciOgesi::new().child("Tamamla"),
    ])
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/stepper_story.rs` dosyasına bakın.
