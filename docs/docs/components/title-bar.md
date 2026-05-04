---
title: Başlık Çubuğu
description: Özel pencere başlığı ve pencere kontrolleri için kullanılır.
---

# Başlık Çubuğu

Özel pencere başlığı ve pencere kontrolleri için kullanılır. Güncel örnekler Kavis UI'nin Türkçe `BaslikCubugu` yüzeyini kullanır.

## İçe Aktarma

```rust
use kavis_ui::ham_gpui::*;
use kavis_ui::*;
```

## `BaslikCubugu`

Ana tipler: `BaslikCubugu`.

## Kullanım

```rust
BaslikCubugu::new().child("Kavis UI")
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/title_bar.rs` dosyasına bakın.
