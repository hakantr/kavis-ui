use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder, relative,
};
use smallvec::SmallVec;

use crate::{EtkinTema, StilUzantisi as _, v_flex};

/// varyant GrupKutusu.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum GrupKutusuVaryanti {
    #[default]
    Normal,
    Fill,
    Outline,
}

/// Trait için ekler GrupKutusu varyant yöntemler için öğeler.
pub trait GrupKutusuVaryantlari: Sized {
    /// varyant [`GrupKutusu`] ayarlar.
    fn with_variant(self, variant: GrupKutusuVaryanti) -> Self;
    /// kullanmak için [`GrupKutusuVaryanti::Normal`] için GrupKutusu ayarlar.
    fn normal(mut self) -> Self {
        self = self.with_variant(GrupKutusuVaryanti::Normal);
        self
    }
    /// kullanmak için [`GrupKutusuVaryanti::Fill`] için GrupKutusu ayarlar.
    fn fill(mut self) -> Self {
        self = self.with_variant(GrupKutusuVaryanti::Fill);
        self
    }
    /// kullanmak için [`GrupKutusuVaryanti::çerçeve`] için GrupKutusu ayarlar.
    fn outline(mut self) -> Self {
        self = self.with_variant(GrupKutusuVaryanti::Outline);
        self
    }
}

impl GrupKutusuVaryanti {
    /// Bir GrupKutusuVaryanti bir metin. oluşturur.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "fill" => GrupKutusuVaryanti::Fill,
            "outline" => GrupKutusuVaryanti::Outline,
            _ => GrupKutusuVaryanti::Normal,
        }
    }

    /// GrupKutusuVaryanti için bir metin. dönüştürür.
    pub fn as_str(&self) -> &str {
        match self {
            GrupKutusuVaryanti::Normal => "normal",
            GrupKutusuVaryanti::Fill => "fill",
            GrupKutusuVaryanti::Outline => "outline",
        }
    }
}

/// GrupKutusu, başlık ve içerik alanı olan stilli bir kapsayıcı öğedir.
/// bir isteğe bağlı başlık için gruplar related içerik together.
#[derive(IntoElement)]
pub struct GrupKutusu {
    id: Option<ElementId>,
    variant: GrupKutusuVaryanti,
    style: StyleRefinement,
    title_style: StyleRefinement,
    title: Option<AnyElement>,
    content_style: StyleRefinement,
    children: SmallVec<[AnyElement; 1]>,
}

impl GrupKutusu {
    /// Yeni bir GrupKutusu oluşturur.
    pub fn new() -> Self {
        Self {
            id: None,
            variant: GrupKutusuVaryanti::default(),
            style: StyleRefinement::default(),
            title_style: StyleRefinement::default(),
            content_style: StyleRefinement::default(),
            title: None,
            children: SmallVec::new(),
        }
    }

    /// Grup kutusunun id değerini ayarlar. Varsayılan None değeridir.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Grup kutusunun başlığını ayarlar. Varsayılan None değeridir.
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }

    /// Grup kutusu başlığı için varsayılan stili geçersiz kılacak stili ayarlar. Varsayılan None.
    pub fn title_style(mut self, style: StyleRefinement) -> Self {
        self.title_style = style;
        self
    }

    /// Grup kutusu içeriği için varsayılan stili geçersiz kılacak stili ayarlar. Varsayılan None.
    pub fn content_style(mut self, style: StyleRefinement) -> Self {
        self.content_style = style;
        self
    }
}

impl ParentElement for GrupKutusu {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for GrupKutusu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl GrupKutusuVaryantlari for GrupKutusu {
    fn with_variant(mut self, variant: GrupKutusuVaryanti) -> Self {
        self.variant = variant;
        self
    }
}

impl RenderOnce for GrupKutusu {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let (bg, border, has_paddings) = match self.variant {
            GrupKutusuVaryanti::Normal => (None, None, false),
            GrupKutusuVaryanti::Fill => (Some(cx.theme().group_box), None, true),
            GrupKutusuVaryanti::Outline => (None, Some(cx.theme().border), true),
        };

        // Add `div` wrapper to avoid sometime width not full issue.
        div().child(
            v_flex()
                .id(self.id.unwrap_or("group-box".into()))
                .w_full()
                .when(has_paddings, |this| this.gap_3())
                .when(!has_paddings, |this| this.gap_4())
                .refine_style(&self.style)
                .when_some(self.title, |this, title| {
                    this.child(
                        div()
                            .text_color(cx.theme().muted_foreground)
                            .line_height(relative(1.))
                            .refine_style(&self.title_style)
                            .child(title),
                    )
                })
                .child(
                    v_flex()
                        .when_some(bg, |this, bg| this.bg(bg))
                        .when_some(border, |this, border| this.border_color(border).border_1())
                        .text_color(cx.theme().group_box_foreground)
                        .when(has_paddings, |this| this.p_4())
                        .gap_4()
                        .rounded(cx.theme().radius)
                        .refine_style(&self.content_style)
                        .children(self.children),
                ),
        )
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_group_variant_from_str() {
        use super::GrupKutusuVaryanti;

        assert_eq!(
            GrupKutusuVaryanti::from_str("normal"),
            GrupKutusuVaryanti::Normal
        );
        assert_eq!(
            GrupKutusuVaryanti::from_str("fill"),
            GrupKutusuVaryanti::Fill
        );
        assert_eq!(
            GrupKutusuVaryanti::from_str("outline"),
            GrupKutusuVaryanti::Outline
        );
        assert_eq!(
            GrupKutusuVaryanti::from_str("other"),
            GrupKutusuVaryanti::Normal
        );

        assert_eq!(
            GrupKutusuVaryanti::from_str("FILL"),
            GrupKutusuVaryanti::Fill
        );
        assert_eq!(
            GrupKutusuVaryanti::from_str("OutLine"),
            GrupKutusuVaryanti::Outline
        );

        assert_eq!(GrupKutusuVaryanti::Normal.as_str(), "normal");
        assert_eq!(GrupKutusuVaryanti::Fill.as_str(), "fill");
        assert_eq!(GrupKutusuVaryanti::Outline.as_str(), "outline");
    }
}
