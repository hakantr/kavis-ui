---
title: Menü
description: Popup, bağlam ve uygulama menüsü altyapısını kapsar.
---

# Menü

Popup, bağlam ve uygulama menüsü altyapısını kapsar. Güncel örnekler Kavis UI'nin Türkçe `PopupMenu`, `PopupMenuItem`, `UygulamaMenusu` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `PopupMenu`, `PopupMenuItem`, `UygulamaMenusu`

Ana tipler: `PopupMenu`, `PopupMenuItem`, `UygulamaMenusu`.

## Kullanım

```rust
AcilirDugme::new("dosya")
    .button(Dugme::new("dosya-dugme").label("Dosya"))
    .dropdown_menu(|menu, _, _| menu.label("Yeni").label("Çıkış"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/menu_story.rs` dosyasına bakın.
