use std::rc::Rc;

use crate::ham_gpui::{
    App, ClickEvent, ElementId, Empty, Hsla, InteractiveElement, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px, rems, transparent_white,
};

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, EtkinTema as _, Renklendir, Simge, SimgeAdi, StilUzantisi,
    h_flex,
    text::{MetinGorunumuStili, Text},
};

/// varyant [`Uyari`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum UyariVaryanti {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Error,
}

impl UyariVaryanti {
    fn fg(&self, cx: &App) -> Hsla {
        match self {
            Self::Default => cx.theme().foreground,
            Self::Info => cx.theme().info,
            Self::Success => cx.theme().success,
            Self::Warning => cx.theme().warning,
            Self::Error => cx.theme().danger,
        }
    }

    fn bg(&self, cx: &App) -> Hsla {
        match self {
            Self::Default => cx.theme().background,
            Self::Info => cx.theme().info.mix_oklab(transparent_white(), 0.04),
            Self::Success => cx.theme().success.mix_oklab(transparent_white(), 0.04),
            Self::Warning => cx.theme().warning.mix_oklab(transparent_white(), 0.04),
            Self::Error => cx.theme().danger.mix_oklab(transparent_white(), 0.04),
        }
    }

    fn border_color(&self, cx: &App) -> Hsla {
        match self {
            Self::Default => cx.theme().border,
            Self::Info => cx.theme().info.mix_oklab(transparent_white(), 0.3),
            Self::Success => cx.theme().success.mix_oklab(transparent_white(), 0.3),
            Self::Warning => cx.theme().warning.mix_oklab(transparent_white(), 0.3),
            Self::Error => cx.theme().danger.mix_oklab(transparent_white(), 0.3),
        }
    }
}

/// Uyari için kullanılır gösterim bir mesaj için kullanıcı.
#[derive(IntoElement)]
pub struct Uyari {
    id: ElementId,
    style: StyleRefinement,
    variant: UyariVaryanti,
    icon: Simge,
    title: Option<SharedString>,
    message: Text,
    size: BilesenBoyutu,
    banner: bool,
    on_close: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    visible: bool,
}

impl Uyari {
    /// Yeni bir uyarı ile verilen mesaj oluşturur.
    pub fn new(id: impl Into<ElementId>, message: impl Into<Text>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            variant: UyariVaryanti::default(),
            icon: Simge::new(SimgeAdi::Info),
            title: None,
            message: message.into(),
            size: BilesenBoyutu::default(),
            banner: false,
            visible: true,
            on_close: None,
        }
    }

    /// Yeni bir bilgi [`UyariVaryanti::Info`] ile verilen mesaj oluşturur.
    pub fn info(id: impl Into<ElementId>, message: impl Into<Text>) -> Self {
        Self::new(id, message)
            .with_variant(UyariVaryanti::Info)
            .icon(SimgeAdi::Info)
    }

    /// Yeni bir [`UyariVaryanti::Success`] uyarı ile verilen mesaj oluşturur.
    pub fn success(id: impl Into<ElementId>, message: impl Into<Text>) -> Self {
        Self::new(id, message)
            .with_variant(UyariVaryanti::Success)
            .icon(SimgeAdi::CircleCheck)
    }

    /// Yeni bir [`UyariVaryanti::Warning`] uyarı ile verilen mesaj oluşturur.
    pub fn warning(id: impl Into<ElementId>, message: impl Into<Text>) -> Self {
        Self::new(id, message)
            .with_variant(UyariVaryanti::Warning)
            .icon(SimgeAdi::TriangleAlert)
    }

    /// Yeni bir [`UyariVaryanti::hata`] uyarı ile verilen mesaj oluşturur.
    pub fn error(id: impl Into<ElementId>, message: impl Into<Text>) -> Self {
        Self::new(id, message)
            .with_variant(UyariVaryanti::Error)
            .icon(SimgeAdi::CircleX)
    }

    /// [`UyariVaryanti`] uyarı ayarlar.
    pub fn with_variant(mut self, variant: UyariVaryanti) -> Self {
        self.variant = variant;
        self
    }

    /// simge için uyarı ayarlar.
    pub fn icon(mut self, icon: impl Into<Simge>) -> Self {
        self.icon = icon.into();
        self
    }

    /// başlık için uyarı ayarlar.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// uyarı olarak banner stil ayarlar.
    ///
    /// `banner` stili uyarının kapsayıcı genişliğinin tamamını kaplamasını sağlar; kenarlık ve yarıçap kullanmaz.
    /// Bu mod olmayacak gösterim `başlık`.
    pub fn banner(mut self) -> Self {
        self.banner = true;
        self
    }

    /// görünürlük uyarı ayarlar.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Uyarıyı kapatılabilir yapar; true ise kapatma simgesini gösterir.
    pub fn on_close(
        mut self,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Rc::new(on_close));
        self
    }
}

impl Boyutlandirilabilir for Uyari {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for Uyari {
    fn style(&mut self) -> &mut crate::ham_gpui::StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Uyari {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.visible {
            return Empty.into_any_element();
        }

        let (radius, padding_x, padding_y, gap) = match self.size {
            BilesenBoyutu::CokKucuk => (cx.theme().radius, px(12.), px(6.), px(6.)),
            BilesenBoyutu::Kucuk => (cx.theme().radius, px(12.), px(8.), px(6.)),
            BilesenBoyutu::Buyuk => (cx.theme().radius_lg, px(20.), px(14.), px(12.)),
            _ => (cx.theme().radius, px(16.), px(10.), px(12.)),
        };

        let bg = self.variant.bg(cx);
        let fg = self.variant.fg(cx);
        let border_color = self.variant.border_color(cx);

        h_flex()
            .id(self.id)
            .w_full()
            .text_color(fg)
            .bg(bg)
            .px(padding_x)
            .py(padding_y)
            .gap(gap)
            .justify_between()
            .text_sm()
            .border_1()
            .border_color(border_color)
            .when(!self.banner, |this| this.rounded(radius).items_start())
            .refine_style(&self.style)
            .child(
                div()
                    .flex()
                    .flex_1()
                    .when(self.banner, |this| this.items_center())
                    .overflow_hidden()
                    .gap(gap)
                    .child(
                        div()
                            .when(!self.banner, |this| this.mt(px(5.)))
                            .child(self.icon),
                    )
                    .child(
                        div()
                            .flex_1()
                            .overflow_hidden()
                            .gap_3()
                            .when(!self.banner, |this| {
                                this.when_some(self.title, |this, title| {
                                    this.child(
                                        div().w_full().truncate().font_semibold().child(title),
                                    )
                                })
                            })
                            .child(
                                self.message
                                    .style(MetinGorunumuStili::default().paragraph_gap(rems(0.2))),
                            ),
                    ),
            )
            .when_some(self.on_close, |this, on_close| {
                this.child(
                    div()
                        .id("close")
                        .p_0p5()
                        .rounded(cx.theme().radius)
                        .hover(|this| this.bg(bg.opacity(0.8)))
                        .active(|this| this.bg(bg.opacity(0.9)))
                        .on_click(move |ev, window, cx| {
                            on_close(ev, window, cx);
                        })
                        .child(
                            Simge::new(SimgeAdi::Close)
                                .with_size(self.size.max(BilesenBoyutu::Orta))
                                .flex_shrink_0(),
                        ),
                )
            })
            .into_any_element()
    }
}
