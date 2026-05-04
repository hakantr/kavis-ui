---
title: Sekmeler
description: Aynı alanda birden çok görünüm arasında geçiş sağlar.
---

# Sekmeler

Aynı alanda birden çok görünüm arasında geçiş sağlar. Güncel örnekler Kavis UI'nin Türkçe `SekmeCubugu`, `Sekme` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `SekmeCubugu`, `Sekme`

Ana tipler: `SekmeCubugu`, `Sekme`.

## Kullanım

```rust
SekmeCubugu::new("sayfalar")
    .child(Sekme::new("genel").label("Genel"))
    .child(Sekme::new("gelismis").label("Gelişmiş"))
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/tabs_story.rs` dosyasına bakın.
