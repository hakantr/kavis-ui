---
title: Geçiş
description: Seçili/seçili değil durumunu düğme biçiminde gösterir.
---

# Geçiş

Seçili/seçili değil durumunu düğme biçiminde gösterir. Güncel örnekler Kavis UI'nin Türkçe `Gecis`, `GecisGrubu` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Gecis`, `GecisGrubu`

Ana tipler: `Gecis`, `GecisGrubu`.

## Kullanım

```rust
GecisGrubu::new("hizalama")
    .children([Gecis::new("sol").label("Sol"), Gecis::new("orta").label("Orta")])
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/toggle_story.rs` dosyasına bakın.
