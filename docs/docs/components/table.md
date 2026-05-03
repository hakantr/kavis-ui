---
title: Tablo
description: Basit statik tablo düzeni için satır ve hücre bileşenleri.
---

# Tablo

Basit statik tablo düzeni için satır ve hücre bileşenleri. Güncel örnekler Kavis UI'nin Türkçe `Tablo`, `TabloSatiri`, `TabloHucresi` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Tablo`, `TabloSatiri`, `TabloHucresi`

Ana tipler: `Tablo`, `TabloSatiri`, `TabloHucresi`.

## Kullanım

```rust
Tablo::new().child(
    TabloGovdesi::new().child(TabloSatiri::new().child(TabloHucresi::new().child("Ad")))
)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/table_story.rs` dosyasına bakın.
