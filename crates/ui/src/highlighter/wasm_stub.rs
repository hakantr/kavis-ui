//! Vurgulayıcı modülü için WASM yer tutucu uygulaması.
//! Provides empty/no-op implementations since tree-sitter is not available in WASM.
//!
//! Not: diagnostics.rs WASM üzerinde kullanılabilir; yalnızca sözdizimi vurgulama yer tutucu gerektirir.

use gpui::{HighlightStyle, SharedString};
use std::ops::Range;
use std::time::Duration;

// Syntax highlighter stub
pub struct SozdizimiVurgulayici;

impl SozdizimiVurgulayici {
    pub fn new(_language: impl AsRef<str>) -> Self {
        Self
    }

    pub fn highlight(&self, _text: &ropey::Rope) -> Vec<(Range<usize>, HighlightStyle)> {
        Vec::new()
    }

    pub fn styles(
        &self,
        _range: &Range<usize>,
        _theme: &VurguTemasi,
    ) -> Vec<(Range<usize>, HighlightStyle)> {
        Vec::new()
    }

    pub fn update(
        &mut self,
        _edit: Option<crate::input::InputEdit>,
        _text: &ropey::Rope,
        _timeout: Option<Duration>,
    ) -> bool {
        // No-op in WASM
        true
    }

    pub fn language(&self) -> &SharedString {
        static EMPTY: SharedString = SharedString::new_static("");
        &EMPTY
    }

    pub fn text(&self) -> &ropey::Rope {
        static EMPTY_ROPE: LazyLock<ropey::Rope> = LazyLock::new(ropey::Rope::new);
        &EMPTY_ROPE
    }

    pub fn tree(&self) -> Option<&crate::input::Agac> {
        None
    }
}

// Language enum stub
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Unknown,
}

impl Language {
    pub fn from_str(_name: &str) -> Self {
        Language::Unknown
    }

    pub fn name(&self) -> &'static str {
        "unknown"
    }

    pub fn config(&self) -> DilYapilandirmasi {
        DilYapilandirmasi {
            name: "unknown".into(),
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        std::iter::once(Language::Unknown)
    }
}

// Language config stub (without tree_sitter::Language)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DilYapilandirmasi {
    pub name: SharedString,
}

// Re-export theme types from registry module (which will be conditionally compiled)
// For WASM, we create minimal stubs here
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YaziTipiStili {
    Normal,
    Italic,
    Underline,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, JsonSchema, Serialize, Deserialize)]
#[repr(u16)]
pub enum YaziKalinligiIcerigi {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    Semibold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
pub struct TemaStili {
    pub color: Option<gpui::Hsla>,
    pub font_style: Option<YaziTipiStili>,
    pub font_weight: Option<YaziKalinligiIcerigi>,
}

impl From<TemaStili> for HighlightStyle {
    fn from(style: TemaStili) -> Self {
        HighlightStyle {
            color: style.color,
            font_weight: style.font_weight.map(|w| match w {
                YaziKalinligiIcerigi::Thin => gpui::FontWeight::THIN,
                YaziKalinligiIcerigi::ExtraLight => gpui::FontWeight::EXTRA_LIGHT,
                YaziKalinligiIcerigi::Light => gpui::FontWeight::LIGHT,
                YaziKalinligiIcerigi::Normal => gpui::FontWeight::NORMAL,
                YaziKalinligiIcerigi::Medium => gpui::FontWeight::MEDIUM,
                YaziKalinligiIcerigi::Semibold => gpui::FontWeight::SEMIBOLD,
                YaziKalinligiIcerigi::Bold => gpui::FontWeight::BOLD,
                YaziKalinligiIcerigi::ExtraBold => gpui::FontWeight::EXTRA_BOLD,
                YaziKalinligiIcerigi::Black => gpui::FontWeight::BLACK,
            }),
            font_style: style.font_style.map(|s| match s {
                YaziTipiStili::Normal => gpui::FontStyle::Normal,
                YaziTipiStili::Italic => gpui::FontStyle::Italic,
                YaziTipiStili::Underline => gpui::FontStyle::Normal,
            }),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
pub struct SozdizimiRenkleri {
    // Minimal stub - actual fields are in native registry.rs
    // Adding commonly accessed fields to avoid compilation errors
    #[serde(rename = "link_text")]
    pub link_text: Option<TemaStili>,
}

impl SozdizimiRenkleri {
    pub fn style(&self, _name: &str) -> Option<HighlightStyle> {
        None
    }

    pub fn style_for_index(&self, _index: usize) -> Option<HighlightStyle> {
        None
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
pub struct DurumRenkleri {
    // Minimal stub
}

impl DurumRenkleri {
    pub fn error(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn error_background(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn error_border(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn warning(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn warning_background(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn warning_border(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn info(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn info_background(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn info_border(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn success(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn success_background(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn success_border(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn hint(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn hint_background(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }

    pub fn hint_border(&self, _cx: &gpui::App) -> gpui::Hsla {
        gpui::Hsla::default()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
pub struct VurguTemasiStili {
    pub editor_background: Option<gpui::Hsla>,
    pub editor_foreground: Option<gpui::Hsla>,
    pub editor_active_line: Option<gpui::Hsla>,
    pub editor_line_number: Option<gpui::Hsla>,
    pub editor_active_line_number: Option<gpui::Hsla>,
    pub editor_invisible: Option<gpui::Hsla>,
    #[serde(flatten)]
    pub status: DurumRenkleri,
    #[serde(rename = "syntax")]
    pub syntax: SozdizimiRenkleri,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
pub struct VurguTemasi {
    pub name: String,
    #[serde(default)]
    pub appearance: crate::TemaModu,
    pub style: VurguTemasiStili,
}

impl std::ops::Deref for VurguTemasi {
    type Target = SozdizimiRenkleri;

    fn deref(&self) -> &Self::Target {
        &self.style.syntax
    }
}

impl VurguTemasi {
    pub fn default_dark() -> std::sync::Arc<Self> {
        use crate::DEFAULT_THEME_COLORS;
        DEFAULT_THEME_COLORS[&crate::TemaModu::Dark].1.clone()
    }

    pub fn default_light() -> std::sync::Arc<Self> {
        use crate::DEFAULT_THEME_COLORS;
        DEFAULT_THEME_COLORS[&crate::TemaModu::Light].1.clone()
    }
}

// Language registry stub
pub struct DilKaydi {
    languages: Mutex<HashMap<SharedString, DilYapilandirmasi>>,
}

impl DilKaydi {
    pub fn singleton() -> &'static LazyLock<DilKaydi> {
        static INSTANCE: LazyLock<DilKaydi> = LazyLock::new(|| DilKaydi {
            languages: Mutex::new(HashMap::new()),
        });
        &INSTANCE
    }

    pub fn register(&self, lang: &str, config: &DilYapilandirmasi) {
        self.languages
            .lock()
            .unwrap()
            .insert(lang.to_string().into(), config.clone());
    }

    pub fn languages(&self) -> Vec<SharedString> {
        self.languages.lock().unwrap().keys().cloned().collect()
    }

    pub fn language(&self, name: &str) -> Option<DilYapilandirmasi> {
        self.languages.lock().unwrap().get(name).cloned()
    }
}
