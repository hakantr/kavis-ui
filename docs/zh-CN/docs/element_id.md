---
title: Öğe Kimliği
description: ElementId değerlerinin bileşenlerdeki rolü.
---

# Öğe Kimliği

Birçok bileşen `ElementId` ister. Bu değer GPUI'nin olay, odak, popover ve state eşleme mekanizmalarında sabit kimlik sağlar.

```rust
Dugme::new("kaydet-dugmesi").label("Kaydet")
OnayKutusu::new("bildirimleri-ac").label("Bildirimler")
Ilerleme::new("indirme-ilerlemesi").value(64.0)
```
