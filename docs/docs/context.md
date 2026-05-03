---
title: Bağlam
description: App, Window ve Context kullanım notları.
---

# Bağlam

Kavis UI, GPUI'nin `App`, `Window` ve `Context<T>` tipleriyle çalışır. Türkçe alias'lar `Uygulama`, `Pencere` ve `GorunumBaglami` olarak dışa aktarılır.

```rust
impl Render for Panel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(format!("Birincil renk: {:?}", cx.theme().primary))
    }
}
```
