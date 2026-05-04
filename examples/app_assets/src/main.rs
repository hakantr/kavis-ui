use anyhow::anyhow;
use kavis_ui::ham_gpui::*;
use kavis_ui::{KokGorunum, SimgeAdi, v_flex};
use rust_embed::RustEmbed;
use std::borrow::Cow;

/// `./assets` klasöründen varlıkları yükleyen bir varlık kaynağı.
#[derive(RustEmbed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
pub struct Varliklar;

impl AssetSource for Varliklar {
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

pub struct Example;
impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .text_center()
            .child(SimgeAdi::Inbox)
            .child(SimgeAdi::Bot)
    }
}

fn main() {
    // Register Varliklar to GPUI application.
    let app = kavis_ui::platform::application().with_assets(Varliklar);

    app.run(move |cx| {
        // We must initialize kavis_ui before using it.
        kavis_ui::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| Example);
                // The first level on the window must be KokGorunum.
                cx.new(|cx| KokGorunum::new(view, window, cx))
            })
            .expect("Pencere açılamadı");
        })
        .detach();
    });
}
