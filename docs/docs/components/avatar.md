---
title: Avatar
description: Kullanıcı görseli, baş harfleri veya simge yer tutucusu gösterir.
---

# Avatar

Kullanıcı görseli, baş harfleri veya simge yer tutucusu gösterir. Güncel örnekler Kavis UI'nin Türkçe `Avatar` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `Avatar`

Ana tipler: `Avatar`.

## Kullanım

```rust
Avatar::new().name("Ayşe Demir").large()
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/avatar_story.rs` dosyasına bakın.
