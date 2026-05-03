---
title: Sayfalama
description: Sayfa numaraları ve gezinme eylemleri sağlar.
---

# Sayfalama

Sayfa numaraları ve gezinme eylemleri sağlar. Güncel örnekler Kavis UI'nin Türkçe `Sayfalama` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Sayfalama`

Ana tipler: `Sayfalama`.

## Kullanım

```rust
Sayfalama::new("sonuclar")
    .current_page(3)
    .total_pages(12)
    .on_click(|page, _, _| println!("Sayfa: {page}"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/pagination_story.rs` dosyasına bakın.
