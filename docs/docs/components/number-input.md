---
title: Sayı Girdisi
description: Sayısal değerler için input state kullanan bileşen.
---

# Sayı Girdisi

Sayısal değerler için input state kullanan bileşen. Güncel örnekler Kavis UI'nin Türkçe `NumberInput`, `InputState` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `NumberInput`, `InputState`

Ana tipler: `NumberInput`, `InputState`.

## Kullanım

```rust
let sayi = cx.new(|cx| InputState::new(window, cx).default_value("42"));
NumberInput::new(&sayi)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/number_input_story.rs` dosyasına bakın.
