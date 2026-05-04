---
title: Açılır Katman
description: Tetikleyici öğeye bağlı küçük katman içerikleri gösterir.
---

# Açılır Katman

Tetikleyici öğeye bağlı küçük katman içerikleri gösterir. Güncel örnekler Kavis UI'nin Türkçe `AcilirKatman` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `AcilirKatman`

Ana tipler: `AcilirKatman`.

## Kullanım

```rust
AcilirKatman::new("filtre")
    .trigger(Dugme::new("filtre-ac").label("Filtre"))
    .content(|_, _| div().p_4().child("Filtre seçenekleri"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/popover_story.rs` dosyasına bakın.
