---
title: Yeniden Boyutlandırılabilir Panel
description: Bölünmüş panellerin sürüklenerek yeniden boyutlandırılmasını sağlar.
---

# Yeniden Boyutlandırılabilir Panel

Bölünmüş panellerin sürüklenerek yeniden boyutlandırılmasını sağlar. Güncel örnekler Kavis UI'nin Türkçe `YenidenBoyutlandirilabilirPanelGrubu`, `YenidenBoyutlandirilabilirPanel` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `YenidenBoyutlandirilabilirPanelGrubu`, `YenidenBoyutlandirilabilirPanel`

Ana tipler: `YenidenBoyutlandirilabilirPanelGrubu`, `YenidenBoyutlandirilabilirPanel`.

## Kullanım

```rust
YenidenBoyutlandirilabilirPanelGrubu::new("duzen")
    .children([YenidenBoyutlandirilabilirPanel::new().child("Sol")])
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/resizable_story.rs` dosyasına bakın.
