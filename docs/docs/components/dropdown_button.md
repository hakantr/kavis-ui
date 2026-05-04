---
title: Açılır Düğme
description: Düğme ile açılır menüyü birleştirir.
---

# Açılır Düğme

Düğme ile açılır menüyü birleştirir. Güncel örnekler Kavis UI'nin Türkçe `AcilirDugme` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `AcilirDugme`

Ana tipler: `AcilirDugme`.

## Kullanım

```rust
AcilirDugme::new("islemler")
    .button(Dugme::new("islemler-dugme").label("İşlemler"))
    .acilir_menu(|menu, _, _| menu.label("Yenile").label("Kapat"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/dropdown_button_story.rs` dosyasına bakın.
