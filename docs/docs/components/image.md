---
title: Görsel
description: GPUI `img()` öğesiyle görsel varlıklarını gösterir.
---

# Görsel

GPUI `img()` öğesiyle görsel varlıklarını gösterir. Güncel örnekler Kavis UI'nin Türkçe `img`, `ImageSource` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `img`, `ImageSource`

Ana tipler: `img`, `ImageSource`.

## Kullanım

```rust
img("images/kapak.png").w_64().h_40().rounded_lg()
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/image_story.rs` dosyasına bakın.
