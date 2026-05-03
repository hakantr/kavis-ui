---
title: Editör
description: Rope tabanlı metin düzenleme, LSP ve sözdizimi vurgulama altyapısını kullanır.
---

# Editör

Rope tabanlı metin düzenleme, LSP ve sözdizimi vurgulama altyapısını kullanır. Güncel örnekler Kavis UI'nin Türkçe `InputState`, `Input`, `MetinGorunumu` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `InputState`, `Input`, `MetinGorunumu`

Ana tipler: `InputState`, `Input`, `MetinGorunumu`.

## Kullanım

```rust
let editor = cx.new(|cx| InputState::new(window, cx).default_value("fn main() {}"));
Input::new(&editor)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/editor_story.rs` dosyasına bakın.
