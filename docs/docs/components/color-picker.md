---
title: Renk Seçici
description: HSLA renk değerini seçmek için durumlu bileşendir.
---

# Renk Seçici

HSLA renk değerini seçmek için durumlu bileşendir. Güncel örnekler Kavis UI'nin Türkçe `RenkSeciciDurumu`, `RenkSecici` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `RenkSeciciDurumu`, `RenkSecici`

Ana tipler: `RenkSeciciDurumu`, `RenkSecici`.

## Kullanım

```rust
let renk = cx.new(|cx| RenkSeciciDurumu::new(window, cx).default_value(cx.theme().primary));
RenkSecici::new(&renk).label("Vurgu rengi")
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/color_picker_story.rs` dosyasına bakın.
