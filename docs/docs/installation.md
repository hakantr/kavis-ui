---
title: Kurulum
description: Kavis UI bağımlılıklarını ve geliştirme komutlarını ayarlama.
---

# Kurulum

```toml
[dependencies]
gpui = { path = "../zed/crates/gpui" }
gpui_platform = { path = "../zed/crates/gpui_platform", features = ["font-kit", "runtime_shaders", "screen-capture", "wayland", "x11"] }
kavis-ui = { git = "https://github.com/hakantr/kavis-ui" }
kavis-ui-assets = { git = "https://github.com/hakantr/kavis-ui" }
anyhow = "1"
```


Uygulama başlangıcında Kavis UI başlatılmalıdır:

```rust
gpui_platform::application().run(move |cx| {
    kavis_ui::init(cx);
});
```

Yerel örnekler:

```bash
cargo run -p hello_world
cargo run -p input
cargo run -p app_assets
```
