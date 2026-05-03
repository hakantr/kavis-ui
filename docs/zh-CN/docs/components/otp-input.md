---
title: OTP Girdisi
description: Tek kullanımlık kod girişleri için durumlu bileşen.
---

# OTP Girdisi

Tek kullanımlık kod girişleri için durumlu bileşen. Güncel örnekler Kavis UI'nin Türkçe `OtpState`, `OtpInput` yüzeyini kullanır.

## İçe Aktarma

```rust
use gpui::*;
use kavis_ui::*;
```

## `OtpState`, `OtpInput`

Ana tipler: `OtpState`, `OtpInput`.

## Kullanım

```rust
let otp = cx.new(|cx| OtpState::new(6, window, cx));
OtpInput::new(&otp)
```

## Notlar

- Durumsuz bileşenler doğrudan `RenderOnce` öğesi olarak döndürülebilir.
- Durum tutan bileşenlerde state `Entity<T>` içinde oluşturulur ve render sırasında bileşene verilir.
- Kapsamlı çalışan örnek için `crates/story/src/stories/otp_input_story.rs` dosyasına bakın.
