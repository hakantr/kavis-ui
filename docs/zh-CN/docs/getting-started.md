---
title: 开始使用
description: 学习如何在项目中安装并使用 Kavis UI。
order: -2
---

# 开始使用

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
gpui = { path = "../zed/crates/gpui" }
gpui_platform = { path = "../zed/crates/gpui_platform", features = ["font-kit", "runtime_shaders", "screen-capture", "wayland", "x11"] }
kavis-ui = { git = "https://github.com/hakantr/kavis-ui" }
# 可选：使用内置默认资源
kavis-ui-assets = { git = "https://github.com/hakantr/kavis-ui" }
anyhow = "1.0"
```

:::tip
`kavis-ui-assets` 是可选依赖。

如果你希望自行管理图标与资源文件，可以不添加它。更多说明见 [资源与图标](./assets.md)。
:::

## 快速开始

下面是一个最小可运行示例：

```rust
use gpui::*;
use kavis_ui::{button::*, *};

pub struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(kavis_ui_assets::Assets);

    app.run(move |cx| {
        kavis_ui::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| HelloWorld);
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
```

:::info
请确保在 `app.run` 闭包中尽早调用 `kavis_ui::init(cx);`。它会初始化主题和全局配置。
:::

## 后续阅读

- [组件总览](./components/index)
- [资源与图标](./assets.md)
