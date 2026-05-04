---
title: Yan Çubuk
description: Gezinme menüsü ve gruplar için yan panel bileşenleri.
---

# Yan Çubuk

Gezinme menüsü ve gruplar için yan panel bileşenleri. Güncel örnekler Kavis UI'nin Türkçe `YanCubuk`, `YanCubukMenusu`, `YanCubukMenuOgesi` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `YanCubuk`, `YanCubukMenusu`, `YanCubukMenuOgesi`

Ana tipler: `YanCubuk`, `YanCubukMenusu`, `YanCubukMenuOgesi`.

## Kullanım

```rust
YanCubukMenusu::new()
    .child(YanCubukMenuOgesi::new("Gösterge").icon(SimgeAdi::LayoutDashboard))
    .child(YanCubukMenuOgesi::new("Ayarlar").icon(SimgeAdi::Settings))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/sidebar_story.rs` dosyasına bakın.
