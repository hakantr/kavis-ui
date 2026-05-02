---
title: Icons & Assets
order: -4
---

# Icons & Assets

The [IconName] and [Icon] in Kavis UI provide a comprehensive set of icons and assets that can be easily integrated into your GPUI applications.

But for minimal size applications, **we have not embedded any icon assets by default** in `kavis-ui` crate.

We split the icon assets into a separate crate [kavis-ui-assets] to allow developers to choose whether to include the icon assets in their applications or if you don't need the icons at all, you can build your own assets.

## Use default bundled assets

The [kavis-ui-assets] crate provides a default bundled assets implementation that includes all the icon files in the `assets/icons` folder.

To use the default bundled assets, you need to add the `kavis-ui-assets` crate as a dependency in your `Cargo.toml`:

```toml
[dependencies]
gpui = { path = "../zed/crates/gpui" }
gpui_platform = { path = "../zed/crates/gpui_platform", features = ["font-kit", "runtime_shaders", "screen-capture", "wayland", "x11"] }
kavis-ui = { git = "https://github.com/hakantr/kavis-ui" }
kavis-ui-assets = { git = "https://github.com/hakantr/kavis-ui" }
```

Then we need call the `with_assets` method when creating the GPUI application to register the asset source:

```rs
use gpui::*;
use kavis_ui_assets::Assets;

let app = gpui_platform::application().with_assets(Assets);
```

Now, we can use `IconName` and `Icon` in our application as usual, the all icon assets are loaded from the default bundled assets.

Continue [Use the icons](#use-the-icons) section to see how to use the icons in your application.

## Build you own assets

You may have a specific set of icons that you want to use in your application, or you may want to reduce the size of your application binary by including only the icons you need.

In this case, you can build your own assets by following these steps.

The [assets](https://github.com/hakantr/kavis-ui/tree/main/crates/assets/assets/) folder in source code contains all the available icons in SVG format, every file is that Kavis UI support, it matched with the [IconName] enum.

You can download the SVG files you need from the [assets] folder, or you can use your own SVG files by following the [IconName] naming convention.

In GPUI application, we can use the [rust-embed] crate to embed the SVG files into the application binary.

And GPUI Application providers an `AssetSource` trait to load the assets.

```rs
use anyhow::anyhow;
use gpui::*;
use kavis_ui::{v_flex, IconName, Root};
use rust_embed::RustEmbed;
use std::borrow::Cow;

/// An asset source that loads assets from the `./assets` folder.
#[derive(RustEmbed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}
```

We need call the `with_assets` method when creating the GPUI application to register the asset source:

```rs
fn main() {
    // Register Assets to GPUI application.
    let app = gpui_platform::application().with_assets(Assets);

    app.run(move |cx| {
        // We must initialize kavis_ui before using it.
        kavis_ui::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| Example);
                // The first level on the window must be Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
```

## Use the icons

Now we can use the icons in our application:

```rs
pub struct Example;

impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .text_center()
            .child(IconName::Inbox)
            .child(IconName::Bot)
    }
}
```

## Resources

- [Lucide Icons](https://lucide.dev/) - The icon set used in Kavis UI is based on the open-source Lucide Icons library, which provides a wide range of customizable SVG icons.

[rust-embed]: https://docs.rs/rust-embed/latest/rust_embed/
[IconName]: https://docs.rs/kavis_ui/latest/kavis_ui/icon/enum.IconName.html
[Icon]: https://docs.rs/kavis_ui/latest/kavis_ui/icon/struct.Icon.html
[assets]: https://github.com/hakantr/kavis-ui/tree/main/crates/assets/assets/
[kavis-ui-assets]: https://crates.io/crates/kavis-ui-assets
