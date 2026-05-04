/// Kavis UI için uygulama varlıklarını gömer.
///
/// Bu varlık paketi `kavis_ui::SimgeAdi` için SVG simge dosyaları sağlar.
///
/// ## Kullanım
///
/// ```rust,no_run
/// use gpui::*;
/// use kavis_ui_assets::Varliklar;
///
/// let app = gpui_platform::application().with_assets(Varliklar);
/// ```
///
/// ## Platform Farkları
///
/// - **Native (Masaüstü)**: Simgeler RustEmbed kullanılarak ikili dosyaya gömülür
/// - **WASM (Web)**: Simgeler web_sys::Request kullanılarak CDN üzerinden indirilir
/// - Bu, WASM paket boyutunu önemli ölçüde azaltır
/// - Simgeler ilk kullanıldıklarında ihtiyaç üzerine indirilir
/// - İndirilen simgeler bellekte önbelleğe alınır
#[cfg(not(target_family = "wasm"))]
mod native_assets;

#[cfg(target_family = "wasm")]
mod wasm_assets;

#[cfg(not(target_family = "wasm"))]
pub use native_assets::Varliklar;

#[cfg(target_family = "wasm")]
pub use wasm_assets::Varliklar;
