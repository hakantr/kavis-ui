---
title: Menü
description: Popup, bağlam ve uygulama menüsü altyapısını kapsar.
---

# Menü

Popup, bağlam ve uygulama menüsü altyapısını kapsar. Güncel örnekler Kavis UI'nin Türkçe `AcilirMenu`, `AcilirMenuOgesi`, `UygulamaMenusu` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `AcilirMenu`, `AcilirMenuOgesi`, `UygulamaMenusu`

Ana tipler: `AcilirMenu`, `AcilirMenuOgesi`, `UygulamaMenusu`.

## Kullanım

```rust
AcilirDugme::new("dosya")
    .button(Dugme::new("dosya-dugme").label("Dosya"))
    .acilir_menu(|menu, _, _| menu.label("Yeni").label("Çıkış"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/menu_story.rs` dosyasına bakın.
