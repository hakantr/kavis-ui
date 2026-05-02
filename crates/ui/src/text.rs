mod document;
mod format;
mod inline;
mod node;
mod state;
mod style;
mod text_view;
mod utils;

use gpui::{App, ElementId, IntoElement, RenderOnce, SharedString, Window};
pub use state::*;
pub use style::*;
pub use text_view::*;

pub(crate) fn init(cx: &mut App) {
    state::init(cx);
}

/// Yeni bir markdown metin görünüm ile kod location olarak id oluşturur.
#[track_caller]
pub fn markdown(source: impl Into<SharedString>) -> MetinGorunumu {
    let id: ElementId = ElementId::CodeLocation(*std::panic::Location::caller());
    MetinGorunumu::markdown(id, source)
}

/// Yeni bir html metin görünüm ile kod location olarak id oluşturur.
#[track_caller]
pub fn html(source: impl Into<SharedString>) -> MetinGorunumu {
    let id: ElementId = ElementId::CodeLocation(*std::panic::Location::caller());
    MetinGorunumu::html(id, source)
}

#[derive(IntoElement, Clone)]
pub enum Text {
    String(SharedString),
    MetinGorunumu(Box<MetinGorunumu>),
}

impl From<SharedString> for Text {
    fn from(s: SharedString) -> Self {
        Self::String(s)
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::String(SharedString::from(s.to_string()))
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::String(s.into())
    }
}

impl From<MetinGorunumu> for Text {
    fn from(e: MetinGorunumu) -> Self {
        Self::MetinGorunumu(Box::new(e))
    }
}

impl Text {
    /// stil için [`MetinGorunumu`] ayarlar.
    ///
    /// Bu `metin` ise işlem yapmaz.
    pub fn style(self, style: MetinGorunumuStili) -> Self {
        match self {
            Self::String(s) => Self::String(s),
            Self::MetinGorunumu(e) => Self::MetinGorunumu(Box::new(e.style(style))),
        }
    }

    /// metin içerik döndürür.
    pub(crate) fn get_text(&self, cx: &App) -> SharedString {
        match self {
            Self::String(s) => s.clone(),
            Self::MetinGorunumu(view) => {
                if let Some(state) = &view.state {
                    state.read(cx).source()
                } else {
                    SharedString::default()
                }
            }
        }
    }
}

impl RenderOnce for Text {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        match self {
            Self::String(s) => s.into_any_element(),
            Self::MetinGorunumu(e) => e.into_any_element(),
        }
    }
}
