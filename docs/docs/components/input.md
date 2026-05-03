---
title: Girdi
description: Tek satır veya çok satırlı metin girişi için state tabanlı bileşen.
---

# Girdi

Tek satır veya çok satırlı metin girişi için state tabanlı bileşen. Güncel örnekler Kavis UI'nin Türkçe `InputState`, `Input` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `InputState`, `Input`

Ana tipler: `InputState`, `Input`.

## Kullanım

```rust
let input = cx.new(|cx| InputState::new(window, cx).placeholder("Ara..."));
Input::new(&input)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/input_story.rs` dosyasına bakın.
