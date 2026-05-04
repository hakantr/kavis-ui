---
title: Odak Tuzağı
description: Modal katman içinde klavye odağını sınırlar.
---

# Odak Tuzağı

Modal katman içinde klavye odağını sınırlar. Güncel örnekler Kavis UI'nin Türkçe `FocusTrapElement` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `FocusTrapElement`

Ana tipler: `FocusTrapElement`.

## Kullanım

```rust
div()
    .track_focus(&self.focus_handle(cx))
    .child(Dugme::new("kapat").label("Kapat"))
    .focus_trap()
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/focus_trap_story.rs` dosyasına bakın.
