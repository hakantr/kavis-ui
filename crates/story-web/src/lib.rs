use std::borrow::Cow;

use gpui::{prelude::*, *};
use kavis_ui::{KokGorunum, theme::Tema};
use kavis_ui_assets::Varliklar;
use kavis_ui_story::{Gallery, StoryRoot};
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
fn escape_html(message: &str) -> String {
    message
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(target_family = "wasm")]
fn show_startup_error(message: &str) {
    let escaped_message = escape_html(message);
    let markup = format!(
        r#"<div class="error">
  <h2>Galeri başlatılamadı</h2>
  <p>Tarayıcı GPU bağlamı başlatılamadı. Lütfen WebGPU desteği olan güncel bir tarayıcı kullanın ve donanım hızlandırmanın açık olduğundan emin olun.</p>
  <pre>{escaped_message}</pre>
</div>"#
    );

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    if let Some(loading) = document.get_element_by_id("loading") {
        loading.set_inner_html(&markup);
        return;
    }

    let Ok(error_container) = document.create_element("div") else {
        return;
    };
    error_container.set_inner_html(&markup);
    error_container.set_class_name("startup-error");

    if let Some(body) = document.body() {
        let _ = body.append_child(&error_container);
    }
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    #[cfg(target_family = "wasm")]
    gpui_platform::web_init();
    #[cfg(not(target_family = "wasm"))]
    let app = gpui_platform::application();
    #[cfg(target_family = "wasm")]
    let app = {
        let app = gpui_platform::single_threaded_web();

        // Temporary fix: intentionally leak the `Rc<AppCell>` to keep the application alive
        struct WasmApplication(std::rc::Rc<AppCell>);
        let wasm_app = unsafe { std::mem::transmute::<Application, WasmApplication>(app) };
        std::mem::forget(wasm_app.0.clone());
        unsafe { std::mem::transmute::<WasmApplication, Application>(wasm_app) }
    };

    app.with_assets(Varliklar::new(
        "https://hakantr.github.io/kavis-ui/gallery/",
    ))
    .run(|cx: &mut App| {
        kavis_ui_story::init(cx);

        // Load fonts for WASM (system fonts are not available in the browser).
        // - Noto Sans SC: subset covering GB2312 Level 1 (~3755 common Chinese characters) + Latin
        // - Noto Emoji: monochrome emoji glyphs
        let cjk_font =
            Cow::Borrowed(include_bytes!("../fonts/NotoSansSC-Regular-subset.ttf").as_slice());
        let emoji_font = Cow::Borrowed(include_bytes!("../fonts/NotoEmoji-Regular.ttf").as_slice());
        cx.text_system()
            .add_fonts(vec![cjk_font, emoji_font])
            .expect("Yazı tipleri yüklenemedi");

        // Use Noto Sans SC as the default font family for unified CJK + Latin rendering.
        cx.global_mut::<Tema>().font_family = "Noto Sans SC".into();

        if let Err(error) = cx.open_window(WindowOptions::default(), |window, cx| {
            let view = Gallery::view(None, window, cx);
            let story_root = cx.new(|cx| StoryRoot::new("Kavis UI", view, window, cx));
            cx.new(|cx| KokGorunum::new(story_root, window, cx))
        }) {
            let message = format!("Pencere açılamadı: {error:#}");

            #[cfg(target_family = "wasm")]
            show_startup_error(&message);

            #[cfg(not(target_family = "wasm"))]
            log::error!("{message}");

            return;
        }

        let _ = console_log::init_with_level(log::Level::Info);
        let _ = tracing_wasm::try_set_as_global_default();
        cx.activate(true);
    });

    Ok(())
}
