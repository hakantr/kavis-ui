---
title: Cip
description: Kısa durum, kategori veya etiket bilgisini gösterir.
---

# Cip

Kısa durum, kategori veya etiket bilgisini gösterir. Güncel örnekler Kavis UI'nin Türkçe `Cip` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `Cip`

Ana tipler: `Cip`.

## Kullanım

```rust
Cip::primary().child("Yeni")
Cip::success().child("Aktif")
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/tag_story.rs` dosyasına bakın.
