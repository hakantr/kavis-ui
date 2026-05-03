---
title: Liste
description: Temsilci tabanlı sanal liste bileşeni.
---

# Liste

Temsilci tabanlı sanal liste bileşeni. Güncel örnekler Kavis UI'nin Türkçe `ListeDurumu`, `Liste`, `ListeOgesi` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `ListeDurumu`, `Liste`, `ListeOgesi`

Ana tipler: `ListeDurumu`, `Liste`, `ListeOgesi`.

## Kullanım

```rust
let liste = cx.new(|cx| ListeDurumu::new(temsilci, window, cx));
Liste::new(&liste)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/list_story.rs` dosyasına bakın.
