---
title: Araç İpucu
description: Kısa yardım metinlerini tetikleyici öğeye bağlar.
---

# Araç İpucu

Kısa yardım metinlerini tetikleyici öğeye bağlar. Güncel örnekler Kavis UI'nin Türkçe `AracIpucu` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `AracIpucu`

Ana tipler: `AracIpucu`.

## Kullanım

```rust
Dugme::new("yenile")
    .label("Yenile")
    .tooltip("Veriyi yenile")
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/tooltip_story.rs` dosyasına bakın.
