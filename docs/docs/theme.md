---
title: Tema
description: Kavis UI tema sistemi ve renk erişimi.
---

# Tema

Kavis UI bileşenleri global `Tema` üzerinden renk, yarıçap, gölge, yazı tipi ve kaydırma çubuğu tercihlerini okur.

```rust
use kavis_ui::EtkinTema;

let arka_plan = cx.theme().background;
let metin = cx.theme().foreground;
let birincil = cx.theme().primary;
```

Temayı güncellemek için:

```rust
let mut tema = cx.theme().clone();
tema.shadow = !tema.shadow;
cx.set_global::<Tema>(tema);
window.refresh();
```
