---
title: Kök Görünüm
description: Pencere başına zorunlu KokGorunum kullanımı.
---

# Kök Görünüm

Her pencerenin ilk seviye görünümü `KokGorunum` olmalıdır. Bu katman sheet, dialog, bildirim, popover ve klavye gezinimi altyapılarını bağlar.

```rust
cx.open_window(WindowOptions::default(), |window, cx| {
    let view = cx.new(|_| UygulamaGorunumu);
    cx.new(|cx| KokGorunum::new(view, window, cx))
})?;
```
