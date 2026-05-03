---
title: Ağaç
description: Hiyerarşik verileri genişletilebilir öğelerle gösterir.
---

# Ağaç

Hiyerarşik verileri genişletilebilir öğelerle gösterir. Güncel örnekler Kavis UI'nin Türkçe `AgacDurumu`, `Agac`, `AgacOgesi` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `AgacDurumu`, `Agac`, `AgacOgesi`

Ana tipler: `AgacDurumu`, `Agac`, `AgacOgesi`.

## Kullanım

```rust
let state = cx.new(|cx| AgacDurumu::new(cx).items(vec![AgacOgesi::new("src", "src")]));
Agac::new(&state, |item, _, _| item.label.clone())
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/tree_story.rs` dosyasına bakın.
