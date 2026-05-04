use crate::ham_gpui::{App, FontWeight, HighlightStyle, Hsla, SharedString};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, LazyLock, Mutex},
};

use crate::{
    DEFAULT_THEME_COLORS, EtkinTema, TemaModu,
    highlighter::{Language, languages},
};

pub(super) const HIGHLIGHT_NAMES: [&str; 40] = [
    "attribute",
    "boolean",
    "comment",
    "comment.doc",
    "constant",
    "constructor",
    "embedded",
    "emphasis",
    "emphasis.strong",
    "enum",
    "function",
    "hint",
    "keyword",
    "label",
    "link_text",
    "link_uri",
    "number",
    "operator",
    "predictive",
    "preproc",
    "primary",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "punctuation.list_marker",
    "punctuation.special",
    "string",
    "string.escape",
    "string.regex",
    "string.special",
    "string.special.symbol",
    "tag",
    "tag.doctype",
    "text.literal",
    "title",
    "type",
    "variable",
    "variable.special",
    "variant",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DilYapilandirmasi {
    pub name: SharedString,
    pub language: tree_sitter::Language,
    pub injection_languages: Vec<SharedString>,
    pub highlights: SharedString,
    pub injections: SharedString,
    pub locals: SharedString,
}

impl DilYapilandirmasi {
    pub fn new(
        name: impl Into<SharedString>,
        language: tree_sitter::Language,
        injection_languages: Vec<SharedString>,
        highlights: &str,
        injections: &str,
        locals: &str,
    ) -> Self {
        Self {
            name: name.into(),
            language,
            injection_languages,
            highlights: SharedString::from(highlights.to_string()),
            injections: SharedString::from(injections.to_string()),
            locals: SharedString::from(locals.to_string()),
        }
    }
}

/// Tema için Agac-sitter Vurgulama
///
/// https://docs.rs/ağaç-sitter-vurgulama/0.25.4/tree_sitter_highlight/
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
pub struct SozdizimiRenkleri {
    pub attribute: Option<TemaStili>,
    pub boolean: Option<TemaStili>,
    pub comment: Option<TemaStili>,
    pub comment_doc: Option<TemaStili>,
    pub constant: Option<TemaStili>,
    pub constructor: Option<TemaStili>,
    pub embedded: Option<TemaStili>,
    pub emphasis: Option<TemaStili>,
    #[serde(rename = "emphasis.strong")]
    pub emphasis_strong: Option<TemaStili>,
    #[serde(rename = "enum")]
    pub enum_: Option<TemaStili>,
    pub function: Option<TemaStili>,
    pub hint: Option<TemaStili>,
    pub keyword: Option<TemaStili>,
    pub label: Option<TemaStili>,
    #[serde(rename = "link_text")]
    pub link_text: Option<TemaStili>,
    #[serde(rename = "link_uri")]
    pub link_uri: Option<TemaStili>,
    pub number: Option<TemaStili>,
    pub operator: Option<TemaStili>,
    pub predictive: Option<TemaStili>,
    pub preproc: Option<TemaStili>,
    pub primary: Option<TemaStili>,
    pub property: Option<TemaStili>,
    pub punctuation: Option<TemaStili>,
    #[serde(rename = "punctuation.bracket")]
    pub punctuation_bracket: Option<TemaStili>,
    #[serde(rename = "punctuation.delimiter")]
    pub punctuation_delimiter: Option<TemaStili>,
    #[serde(rename = "punctuation.list_marker")]
    pub punctuation_list_marker: Option<TemaStili>,
    #[serde(rename = "punctuation.special")]
    pub punctuation_special: Option<TemaStili>,
    pub string: Option<TemaStili>,
    #[serde(rename = "string.escape")]
    pub string_escape: Option<TemaStili>,
    #[serde(rename = "string.regex")]
    pub string_regex: Option<TemaStili>,
    #[serde(rename = "string.special")]
    pub string_special: Option<TemaStili>,
    #[serde(rename = "string.special.symbol")]
    pub string_special_symbol: Option<TemaStili>,
    pub tag: Option<TemaStili>,
    #[serde(rename = "tag.doctype")]
    pub tag_doctype: Option<TemaStili>,
    #[serde(rename = "text.literal")]
    pub text_literal: Option<TemaStili>,
    pub title: Option<TemaStili>,
    #[serde(rename = "type")]
    pub type_: Option<TemaStili>,
    pub variable: Option<TemaStili>,
    #[serde(rename = "variable.special")]
    pub variable_special: Option<TemaStili>,
    pub variant: Option<TemaStili>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YaziTipiStili {
    Normal,
    Italic,
    Underline,
}

impl From<YaziTipiStili> for crate::ham_gpui::FontStyle {
    fn from(style: YaziTipiStili) -> Self {
        match style {
            YaziTipiStili::Normal => crate::ham_gpui::FontStyle::Normal,
            YaziTipiStili::Italic => crate::ham_gpui::FontStyle::Italic,
            YaziTipiStili::Underline => crate::ham_gpui::FontStyle::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize_repr, Deserialize_repr, JsonSchema)]
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

impl From<YaziKalinligiIcerigi> for FontWeight {
    fn from(value: YaziKalinligiIcerigi) -> Self {
        match value {
            YaziKalinligiIcerigi::Thin => FontWeight::THIN,
            YaziKalinligiIcerigi::ExtraLight => FontWeight::EXTRA_LIGHT,
            YaziKalinligiIcerigi::Light => FontWeight::LIGHT,
            YaziKalinligiIcerigi::Normal => FontWeight::NORMAL,
            YaziKalinligiIcerigi::Medium => FontWeight::MEDIUM,
            YaziKalinligiIcerigi::Semibold => FontWeight::SEMIBOLD,
            YaziKalinligiIcerigi::Bold => FontWeight::BOLD,
            YaziKalinligiIcerigi::ExtraBold => FontWeight::EXTRA_BOLD,
            YaziKalinligiIcerigi::Black => FontWeight::BLACK,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
pub struct TemaStili {
    color: Option<Hsla>,
    font_style: Option<YaziTipiStili>,
    font_weight: Option<YaziKalinligiIcerigi>,
}

impl From<TemaStili> for HighlightStyle {
    fn from(style: TemaStili) -> Self {
        HighlightStyle {
            color: style.color,
            font_weight: style.font_weight.map(Into::into),
            font_style: style.font_style.map(Into::into),
            ..Default::default()
        }
    }
}

impl SozdizimiRenkleri {
    pub fn style(&self, name: &str) -> Option<HighlightStyle> {
        if name.is_empty() {
            return None;
        }

        let style = match name {
            "attribute" => self.attribute,
            "boolean" => self.boolean,
            "comment" => self.comment,
            "comment.doc" => self.comment_doc,
            "constant" => self.constant,
            "constructor" => self.constructor,
            "embedded" => self.embedded,
            "emphasis" => self.emphasis,
            "emphasis.strong" => self.emphasis_strong,
            "enum" => self.enum_,
            "function" => self.function,
            "hint" => self.hint,
            "keyword" => self.keyword,
            "label" => self.label,
            "link_text" => self.link_text,
            "link_uri" => self.link_uri,
            "number" => self.number,
            "operator" => self.operator,
            "predictive" => self.predictive,
            "preproc" => self.preproc,
            "primary" => self.primary,
            "property" => self.property,
            "punctuation" => self.punctuation,
            "punctuation.bracket" => self.punctuation_bracket,
            "punctuation.delimiter" => self.punctuation_delimiter,
            "punctuation.list_marker" => self.punctuation_list_marker,
            "punctuation.special" => self.punctuation_special,
            "string" => self.string,
            "string.escape" => self.string_escape,
            "string.regex" => self.string_regex,
            "string.special" => self.string_special,
            "string.special.symbol" => self.string_special_symbol,
            "tag" => self.tag,
            "tag.doctype" => self.tag_doctype,
            "text.literal" => self.text_literal,
            "title" => self.title,
            "type" => self.type_,
            "variable" => self.variable,
            "variable.special" => self.variable_special,
            "variant" => self.variant,
            _ => None,
        }
        .map(|s| s.into());

        if style.is_some() {
            style
        } else {
            // Fallback `keyword.modifier` to `keyword`
            if name.contains(".") {
                if let Some(prefix) = name.split(".").next() {
                    return self.style(prefix);
                }

                None
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn style_for_index(&self, index: usize) -> Option<HighlightStyle> {
        HIGHLIGHT_NAMES.get(index).and_then(|name| self.style(name))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
pub struct DurumRenkleri {
    #[serde(rename = "error")]
    error: Option<Hsla>,
    #[serde(rename = "error.background")]
    error_background: Option<Hsla>,
    #[serde(rename = "error.border")]
    error_border: Option<Hsla>,
    #[serde(rename = "warning")]
    warning: Option<Hsla>,
    #[serde(rename = "warning.background")]
    warning_background: Option<Hsla>,
    #[serde(rename = "warning.border")]
    warning_border: Option<Hsla>,
    #[serde(rename = "info")]
    info: Option<Hsla>,
    #[serde(rename = "info.background")]
    info_background: Option<Hsla>,
    #[serde(rename = "info.border")]
    info_border: Option<Hsla>,
    #[serde(rename = "success")]
    success: Option<Hsla>,
    #[serde(rename = "success.background")]
    success_background: Option<Hsla>,
    #[serde(rename = "success.border")]
    success_border: Option<Hsla>,
    #[serde(rename = "hint")]
    hint: Option<Hsla>,
    #[serde(rename = "hint.background")]
    hint_background: Option<Hsla>,
    #[serde(rename = "hint.border")]
    hint_border: Option<Hsla>,
}

impl DurumRenkleri {
    #[inline]
    pub fn error(&self, cx: &App) -> Hsla {
        self.error.unwrap_or(cx.theme().red)
    }

    #[inline]
    pub fn error_background(&self, cx: &App) -> Hsla {
        let bg = cx.theme().background;
        self.error_background
            .unwrap_or(bg.blend(self.error(cx).alpha(0.2)))
    }

    #[inline]
    pub fn error_border(&self, cx: &App) -> Hsla {
        self.error_border.unwrap_or(self.error(cx))
    }

    #[inline]
    pub fn warning(&self, cx: &App) -> Hsla {
        self.warning.unwrap_or(cx.theme().yellow)
    }

    #[inline]
    pub fn warning_background(&self, cx: &App) -> Hsla {
        let bg = cx.theme().background;
        self.warning_background
            .unwrap_or(bg.blend(self.warning(cx).alpha(0.2)))
    }

    #[inline]
    pub fn warning_border(&self, cx: &App) -> Hsla {
        self.warning_border.unwrap_or(self.warning(cx))
    }

    #[inline]
    pub fn info(&self, cx: &App) -> Hsla {
        self.info.unwrap_or(cx.theme().blue)
    }

    #[inline]
    pub fn info_background(&self, cx: &App) -> Hsla {
        let bg = cx.theme().background;
        self.info_background
            .unwrap_or(bg.blend(self.info(cx).alpha(0.2)))
    }

    #[inline]
    pub fn info_border(&self, cx: &App) -> Hsla {
        self.info_border.unwrap_or(self.info(cx))
    }

    #[inline]
    pub fn success(&self, cx: &App) -> Hsla {
        self.success.unwrap_or(cx.theme().green)
    }

    #[inline]
    pub fn success_background(&self, cx: &App) -> Hsla {
        let bg = cx.theme().background;
        self.success_background
            .unwrap_or(bg.blend(self.success(cx).alpha(0.2)))
    }

    #[inline]
    pub fn success_border(&self, cx: &App) -> Hsla {
        self.success_border.unwrap_or(self.success(cx))
    }

    #[inline]
    pub fn hint(&self, cx: &App) -> Hsla {
        self.hint.unwrap_or(cx.theme().cyan)
    }

    #[inline]
    pub fn hint_background(&self, cx: &App) -> Hsla {
        let bg = cx.theme().background;
        self.hint_background
            .unwrap_or(bg.blend(self.hint(cx).alpha(0.2)))
    }

    #[inline]
    pub fn hint_border(&self, cx: &App) -> Hsla {
        self.hint_border.unwrap_or(self.hint(cx))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
pub struct VurguTemasiStili {
    #[serde(rename = "editor.background")]
    pub editor_background: Option<Hsla>,
    #[serde(rename = "editor.foreground")]
    pub editor_foreground: Option<Hsla>,
    #[serde(rename = "editor.active_line.background")]
    pub editor_active_line: Option<Hsla>,
    #[serde(rename = "editor.line_number")]
    pub editor_line_number: Option<Hsla>,
    #[serde(rename = "editor.active_line_number")]
    pub editor_active_line_number: Option<Hsla>,
    #[serde(rename = "editor.invisible")]
    pub editor_invisible: Option<Hsla>,
    #[serde(flatten)]
    pub status: DurumRenkleri,
    #[serde(rename = "syntax")]
    pub syntax: SozdizimiRenkleri,
}

/// JSON tema dosyasından tree-sitter vurgulama teması üretir.
///
/// Bu JSON, Zed tema biçimiyle uyumludur.
///
/// https://zed.dev/docs/extensions/languages#syntax-highlighting
#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema, Serialize, Deserialize)]
pub struct VurguTemasi {
    pub name: String,
    #[serde(default)]
    pub appearance: TemaModu,
    pub style: VurguTemasiStili,
}

impl Deref for VurguTemasi {
    type Target = SozdizimiRenkleri;

    fn deref(&self) -> &Self::Target {
        &self.style.syntax
    }
}

impl VurguTemasi {
    pub fn default_dark() -> Arc<Self> {
        DEFAULT_THEME_COLORS[&TemaModu::Dark].1.clone()
    }

    pub fn default_light() -> Arc<Self> {
        DEFAULT_THEME_COLORS[&TemaModu::Light].1.clone()
    }
}

/// Registry için kod vurgulayıcı languages.
pub struct DilKaydi {
    languages: Mutex<HashMap<SharedString, DilYapilandirmasi>>,
}

impl DilKaydi {
    /// singleton örnek `DilKaydi` ile varsayılan languages ve themes döndürür.
    pub fn singleton() -> &'static LazyLock<DilKaydi> {
        static INSTANCE: LazyLock<DilKaydi> = LazyLock::new(|| DilKaydi {
            languages: Mutex::new(
                languages::Language::all()
                    .map(|language| (language.name().into(), language.config()))
                    .collect(),
            ),
        });
        &INSTANCE
    }

    /// Registers bir yeni dil yapılandırma için kayıt.
    pub fn register(&self, lang: &str, config: &DilYapilandirmasi) {
        self.languages
            .lock()
            .unwrap()
            .insert(lang.to_string().into(), config.clone());
    }

    /// bir liste tüm registered dil adlar. döndürür.
    pub fn languages(&self) -> Vec<SharedString> {
        self.languages.lock().unwrap().keys().cloned().collect()
    }

    /// dil yapılandırma için verilen dil ad döndürür.
    pub fn language(&self, name: &str) -> Option<DilYapilandirmasi> {
        // Try to get by name first, there may have a custom language registered
        // Then try to get built-in language to support short language names, e.g. "js" for "javascript"
        let languages = self.languages.lock().unwrap();
        languages.get(name).cloned().or_else(|| {
            Language::from_name(name).and_then(|language| languages.get(language.name()).cloned())
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::highlighter::DilYapilandirmasi;

    #[test]
    fn test_registry() {
        use super::DilKaydi;
        let registry = DilKaydi::singleton();

        registry.register(
            "foo",
            &DilYapilandirmasi::new("foo", tree_sitter_json::LANGUAGE.into(), vec![], "", "", ""),
        );

        assert!(registry.language("foo").is_some());
        assert!(registry.language("json").is_some());
        assert!(registry.language("text").is_some());
        assert!(registry.language("unknown").is_none());

        #[cfg(feature = "tree-sitter-rust")]
        {
            assert!(registry.language("rust").is_some());
            assert!(registry.language("rs").is_some());
        }
        #[cfg(not(feature = "tree-sitter-rust"))]
        {
            assert!(registry.language("rust").is_none());
            assert!(registry.language("rs").is_none());
        }

        #[cfg(feature = "tree-sitter-javascript")]
        {
            assert!(registry.language("javascript").is_some());
            assert!(registry.language("js").is_some());
        }
        #[cfg(not(feature = "tree-sitter-javascript"))]
        {
            assert!(registry.language("javascript").is_none());
            assert!(registry.language("js").is_none());
        }
    }
}
