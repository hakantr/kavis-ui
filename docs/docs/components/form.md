---
title: Form
description: Alanları etiket ve açıklamalarıyla hizalamak için kullanılır.
---

# Form

Alanları etiket ve açıklamalarıyla hizalamak için kullanılır. Güncel örnekler Kavis UI'nin Türkçe `Form`, `Field` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `Form`, `Field`

Ana tipler: `Form`, `Field`.

## Kullanım

```rust
Form::new()
    .label_width(px(120.0))
    .child(Field::new().label("Ad").child(Input::new(&self.ad)))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/form_story.rs` dosyasına bakın.
