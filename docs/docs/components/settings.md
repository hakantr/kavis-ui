---
title: Ayarlar
description: Ayar sayfaları, gruplar ve alan tipleri için hazır düzen.
---

# Ayarlar

Ayar sayfaları, gruplar ve alan tipleri için hazır düzen. Güncel örnekler Kavis UI'nin Türkçe `Ayarlar`, `AyarSayfasi`, `AyarGrubu`, `AyarOgesi` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Ayarlar`, `AyarSayfasi`, `AyarGrubu`, `AyarOgesi`

Ana tipler: `Ayarlar`, `AyarSayfasi`, `AyarGrubu`, `AyarOgesi`.

## Kullanım

```rust
Ayarlar::new("ayarlar")
    .page(AyarSayfasi::new("Genel"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/settings_story.rs` dosyasına bakın.
