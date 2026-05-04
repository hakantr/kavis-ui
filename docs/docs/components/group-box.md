---
title: Grup Kutusu
description: İlişkili kontrolleri başlıklı bir çerçeve içinde toplar.
---

# Grup Kutusu

İlişkili kontrolleri başlıklı bir çerçeve içinde toplar. Güncel örnekler Kavis UI'nin Türkçe `GrupKutusu` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `GrupKutusu`

Ana tipler: `GrupKutusu`.

## Kullanım

```rust
GrupKutusu::new()
    .title("Görünüm")
    .child(OnayKutusu::new("golge").label("Gölge kullan"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/group_box_story.rs` dosyasına bakın.
