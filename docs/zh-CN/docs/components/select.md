---
title: Seçim
description: Aranabilir veya gruplanabilir seçim bileşeni.
---

# Seçim

Aranabilir veya gruplanabilir seçim bileşeni. Güncel örnekler Kavis UI'nin Türkçe `SecimDurumu`, `Secim`, `SearchableVec`, `SecimGrubu` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `SecimDurumu`, `Secim`, `SearchableVec`, `SecimGrubu`

Ana tipler: `SecimDurumu`, `Secim`, `SearchableVec`, `SecimGrubu`.

## Kullanım

```rust
let items = SearchableVec::new(vec!["Rust", "GPUI", "Kavis"]);
let state = cx.new(|cx| SecimDurumu::new(items, None, window, cx));
Secim::new(&state).placeholder("Seçim yap")
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/select_story.rs` dosyasına bakın.
