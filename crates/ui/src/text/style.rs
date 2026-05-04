use std::sync::Arc;

use crate::ham_gpui::{Pixels, Rems, StyleRefinement, px, rems};

use crate::highlighter::VurguTemasi;

/// MetinGorunumuStili için kullanılır customize stil için [`MetinGorunumu`].
#[derive(Clone)]
pub struct MetinGorunumuStili {
    /// Paragraflar arasındaki boşluk. Varsayılan 1 rem.
    pub paragraph_gap: Rems,
    /// Başlıklar için taban font boyutu. Varsayılan 14pxdir.
    pub heading_base_font_size: Pixels,
    /// fonksiyon için hesaplar heading font boyut temelli üzerinde heading seviye (1-6).
    ///
    /// İlk parametre başlık seviyesi (1-6), ikinci parametre taban font boyutudur.
    /// İkinci parametre taban font boyutudur.
    pub heading_font_size: Option<Arc<dyn Fn(u8, Pixels) -> Pixels + Send + Sync + 'static>>,
    /// Vurgulama tema için kod blocks. varsayılan: [`VurguTemasi::default_light()`]
    pub highlight_theme: Arc<VurguTemasi>,
    /// stil refinement için kod blocks.
    pub code_block: StyleRefinement,
    pub is_dark: bool,
}

impl PartialEq for MetinGorunumuStili {
    fn eq(&self, other: &Self) -> bool {
        self.paragraph_gap == other.paragraph_gap
            && self.heading_base_font_size == other.heading_base_font_size
            && self.highlight_theme == other.highlight_theme
    }
}

impl Default for MetinGorunumuStili {
    fn default() -> Self {
        Self {
            paragraph_gap: rems(1.),
            heading_base_font_size: px(14.),
            heading_font_size: None,
            highlight_theme: VurguTemasi::default_light().clone(),
            code_block: StyleRefinement::default(),
            is_dark: false,
        }
    }
}

impl MetinGorunumuStili {
    /// Paragraf boşluğunu ayarlar. Varsayılan 1remdir.
    pub fn paragraph_gap(mut self, gap: Rems) -> Self {
        self.paragraph_gap = gap;
        self
    }

    pub fn heading_font_size<F>(mut self, f: F) -> Self
    where
        F: Fn(u8, Pixels) -> Pixels + Send + Sync + 'static,
    {
        self.heading_font_size = Some(Arc::new(f));
        self
    }

    /// stil için kod blocks ayarlar.
    pub fn code_block(mut self, style: StyleRefinement) -> Self {
        self.code_block = style;
        self
    }
}
