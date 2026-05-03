---
title: Sanal Liste
description: Büyük koleksiyonları yalnızca görünür satırları çizerek gösterir.
---

# Sanal Liste

Büyük koleksiyonları yalnızca görünür satırları çizerek gösterir. Güncel örnekler Kavis UI'nin Türkçe `SanalListe`, `SanalListeKaydirmaTutamaci` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `SanalListe`, `SanalListeKaydirmaTutamaci`

Ana tipler: `SanalListe`, `SanalListeKaydirmaTutamaci`.

## Kullanım

```rust
use std::rc::Rc;

let item_sizes = Rc::new(vec![size(px(320.0), px(28.0)); 10_000]);
v_virtual_list(cx.entity(), "satirlar", item_sizes, |_, range, _, _| {
    range.map(|ix| div().child(format!("Satır {ix}"))).collect()
})
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/virtual_list_story.rs` dosyasına bakın.
