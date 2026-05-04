---
title: Kaydırılabilir Alan
description: Özel kaydırma çubuğu ve maske yardımcılarını sağlar.
---

# Kaydırılabilir Alan

Özel kaydırma çubuğu ve maske yardımcılarını sağlar. Güncel örnekler Kavis UI'nin Türkçe `Kaydirilabilir`, `KaydirmaCubugu`, `KaydirilabilirMaske` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `Kaydirilabilir`, `KaydirmaCubugu`, `KaydirilabilirMaske`

Ana tipler: `Kaydirilabilir`, `KaydirmaCubugu`, `KaydirilabilirMaske`.

## Kullanım

```rust
div()
    .size_full()
    .overflow_y_scrollbar()
    .child("Uzun içerik")
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/scrollable_story.rs` dosyasına bakın.
