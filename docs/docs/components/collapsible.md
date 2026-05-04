---
title: Daraltılabilir Alan
description: İçeriği açık veya kapalı durumda gösterir.
---

# Daraltılabilir Alan

İçeriği açık veya kapalı durumda gösterir. Güncel örnekler Kavis UI'nin Türkçe `Daraltilabilir` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `Daraltilabilir`

Ana tipler: `Daraltilabilir`.

## Kullanım

```rust
Daraltilabilir::new()
    .open(true)
    .content("Genişletilmiş içerik")
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/collapsible_story.rs` dosyasına bakın.
