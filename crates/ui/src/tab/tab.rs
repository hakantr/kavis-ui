use std::rc::Rc;

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, EtkinTema, Secilebilir, Simge, SimgeAdi, StilUzantisi,
    h_flex,
};
use gpui::prelude::FluentBuilder as _;
use gpui::{
    AnyElement, App, ClickEvent, Div, Edges, Hsla, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Pixels, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window,
    div, px, relative,
};

/// Sekme variants.
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum SekmeVaryanti {
    #[default]
    Sekme,
    Outline,
    Pill,
    Segmented,
    Underline,
}

impl SekmeVaryanti {
    fn height(&self, size: BilesenBoyutu) -> Pixels {
        match size {
            BilesenBoyutu::CokKucuk => match self {
                SekmeVaryanti::Underline => px(26.),
                _ => px(20.),
            },
            BilesenBoyutu::Kucuk => match self {
                SekmeVaryanti::Underline => px(30.),
                _ => px(24.),
            },
            BilesenBoyutu::Buyuk => match self {
                SekmeVaryanti::Underline => px(44.),
                _ => px(36.),
            },
            _ => match self {
                SekmeVaryanti::Underline => px(36.),
                _ => px(32.),
            },
        }
    }

    pub(super) fn inner_height(&self, size: BilesenBoyutu) -> Pixels {
        match size {
            BilesenBoyutu::CokKucuk => match self {
                SekmeVaryanti::Sekme | SekmeVaryanti::Outline | SekmeVaryanti::Pill => px(18.),
                SekmeVaryanti::Segmented => px(16.),
                SekmeVaryanti::Underline => px(20.),
            },
            BilesenBoyutu::Kucuk => match self {
                SekmeVaryanti::Sekme | SekmeVaryanti::Outline | SekmeVaryanti::Pill => px(22.),
                SekmeVaryanti::Segmented => px(18.),
                SekmeVaryanti::Underline => px(22.),
            },
            BilesenBoyutu::Buyuk => match self {
                SekmeVaryanti::Sekme | SekmeVaryanti::Outline | SekmeVaryanti::Pill => px(36.),
                SekmeVaryanti::Segmented => px(28.),
                SekmeVaryanti::Underline => px(32.),
            },
            _ => match self {
                SekmeVaryanti::Sekme => px(30.),
                SekmeVaryanti::Outline | SekmeVaryanti::Pill => px(26.),
                SekmeVaryanti::Segmented => px(24.),
                SekmeVaryanti::Underline => px(26.),
            },
        }
    }

    /// Varsayılan px(12) değeri paneldeki px_3 ile eşleşir; bakınız [`crate::dock::SekmePaneli`].
    fn inner_paddings(&self, size: BilesenBoyutu) -> Edges<Pixels> {
        let mut padding_x = match size {
            BilesenBoyutu::CokKucuk => px(8.),
            BilesenBoyutu::Kucuk => px(10.),
            BilesenBoyutu::Buyuk => px(16.),
            _ => px(12.),
        };

        if matches!(self, SekmeVaryanti::Underline) {
            padding_x = px(0.);
        }

        Edges {
            left: padding_x,
            right: padding_x,
            ..Default::default()
        }
    }

    fn inner_margins(&self, size: BilesenBoyutu) -> Edges<Pixels> {
        match size {
            BilesenBoyutu::CokKucuk => match self {
                SekmeVaryanti::Underline => Edges {
                    top: px(1.),
                    bottom: px(2.),
                    ..Default::default()
                },
                _ => Edges::all(px(0.)),
            },
            BilesenBoyutu::Kucuk => match self {
                SekmeVaryanti::Underline => Edges {
                    top: px(2.),
                    bottom: px(3.),
                    ..Default::default()
                },
                _ => Edges::all(px(0.)),
            },
            BilesenBoyutu::Buyuk => match self {
                SekmeVaryanti::Underline => Edges {
                    top: px(5.),
                    bottom: px(6.),
                    ..Default::default()
                },
                _ => Edges::all(px(0.)),
            },
            _ => match self {
                SekmeVaryanti::Underline => Edges {
                    top: px(3.),
                    bottom: px(4.),
                    ..Default::default()
                },
                _ => Edges::all(px(0.)),
            },
        }
    }

    fn normal(&self, cx: &App) -> TabStyle {
        match self {
            SekmeVaryanti::Sekme => TabStyle {
                fg: cx.theme().tab_foreground,
                bg: cx.theme().transparent,
                borders: Edges {
                    left: px(1.),
                    right: px(1.),
                    ..Default::default()
                },
                border_color: cx.theme().transparent,
                ..Default::default()
            },
            SekmeVaryanti::Outline => TabStyle {
                fg: cx.theme().tab_foreground,
                bg: cx.theme().transparent,
                borders: Edges::all(px(1.)),
                border_color: cx.theme().border,
                ..Default::default()
            },
            SekmeVaryanti::Pill => TabStyle {
                fg: cx.theme().foreground,
                bg: cx.theme().transparent,
                ..Default::default()
            },
            SekmeVaryanti::Segmented => TabStyle {
                fg: cx.theme().tab_foreground,
                bg: cx.theme().transparent,
                ..Default::default()
            },
            SekmeVaryanti::Underline => TabStyle {
                fg: cx.theme().tab_foreground,
                bg: cx.theme().transparent,
                inner_bg: cx.theme().transparent,
                borders: Edges {
                    bottom: px(2.),
                    ..Default::default()
                },
                border_color: cx.theme().transparent,
                ..Default::default()
            },
        }
    }

    fn hovered(&self, selected: bool, cx: &App) -> TabStyle {
        match self {
            SekmeVaryanti::Sekme => TabStyle {
                fg: cx.theme().tab_active_foreground,
                bg: cx.theme().transparent,
                borders: Edges {
                    left: px(1.),
                    right: px(1.),
                    ..Default::default()
                },
                border_color: cx.theme().transparent,
                ..Default::default()
            },
            SekmeVaryanti::Outline => TabStyle {
                fg: cx.theme().secondary_foreground,
                bg: cx.theme().secondary_hover,
                borders: Edges::all(px(1.)),
                border_color: cx.theme().border,
                ..Default::default()
            },
            SekmeVaryanti::Pill => TabStyle {
                fg: cx.theme().secondary_foreground,
                bg: cx.theme().secondary,
                ..Default::default()
            },
            SekmeVaryanti::Segmented => TabStyle {
                fg: cx.theme().tab_active_foreground,
                bg: cx.theme().transparent,
                inner_bg: if selected {
                    cx.theme().background
                } else {
                    cx.theme().transparent
                },
                ..Default::default()
            },
            SekmeVaryanti::Underline => TabStyle {
                fg: cx.theme().tab_active_foreground,
                bg: cx.theme().transparent,
                inner_bg: cx.theme().transparent,
                borders: Edges {
                    bottom: px(2.),
                    ..Default::default()
                },
                border_color: cx.theme().transparent,
                ..Default::default()
            },
        }
    }

    fn selected(&self, cx: &App) -> TabStyle {
        match self {
            SekmeVaryanti::Sekme => TabStyle {
                fg: cx.theme().tab_active_foreground,
                bg: cx.theme().tab_active,
                borders: Edges {
                    left: px(1.),
                    right: px(1.),
                    ..Default::default()
                },
                border_color: cx.theme().border,
                ..Default::default()
            },
            SekmeVaryanti::Outline => TabStyle {
                fg: cx.theme().primary,
                bg: cx.theme().transparent,
                borders: Edges::all(px(1.)),
                border_color: cx.theme().primary,
                ..Default::default()
            },
            SekmeVaryanti::Pill => TabStyle {
                fg: cx.theme().primary_foreground,
                bg: cx.theme().primary,
                ..Default::default()
            },
            SekmeVaryanti::Segmented => TabStyle {
                fg: cx.theme().tab_active_foreground,
                bg: cx.theme().transparent,
                inner_bg: cx.theme().background,
                shadow: true,
                ..Default::default()
            },
            SekmeVaryanti::Underline => TabStyle {
                fg: cx.theme().tab_active_foreground,
                bg: cx.theme().transparent,
                borders: Edges {
                    bottom: px(2.),
                    ..Default::default()
                },
                border_color: cx.theme().primary,
                ..Default::default()
            },
        }
    }

    fn disabled(&self, selected: bool, cx: &App) -> TabStyle {
        match self {
            SekmeVaryanti::Sekme => TabStyle {
                fg: cx.theme().muted_foreground,
                bg: cx.theme().transparent,
                border_color: if selected {
                    cx.theme().border
                } else {
                    cx.theme().transparent
                },
                borders: Edges {
                    left: px(1.),
                    right: px(1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            SekmeVaryanti::Outline => TabStyle {
                fg: cx.theme().muted_foreground,
                bg: cx.theme().transparent,
                borders: Edges::all(px(1.)),
                border_color: if selected {
                    cx.theme().primary
                } else {
                    cx.theme().border
                },
                ..Default::default()
            },
            SekmeVaryanti::Pill => TabStyle {
                fg: if selected {
                    cx.theme().primary_foreground.opacity(0.5)
                } else {
                    cx.theme().muted_foreground
                },
                bg: if selected {
                    cx.theme().primary.opacity(0.5)
                } else {
                    cx.theme().transparent
                },
                ..Default::default()
            },
            SekmeVaryanti::Segmented => TabStyle {
                fg: cx.theme().muted_foreground,
                bg: cx.theme().tab_bar,
                inner_bg: if selected {
                    cx.theme().background
                } else {
                    cx.theme().transparent
                },
                ..Default::default()
            },
            SekmeVaryanti::Underline => TabStyle {
                fg: cx.theme().muted_foreground,
                bg: cx.theme().transparent,
                border_color: if selected {
                    cx.theme().border
                } else {
                    cx.theme().transparent
                },
                borders: Edges {
                    bottom: px(2.),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    pub(super) fn tab_bar_radius(&self, size: BilesenBoyutu, cx: &App) -> Pixels {
        if *self != SekmeVaryanti::Segmented {
            return px(0.);
        }

        match size {
            BilesenBoyutu::CokKucuk | BilesenBoyutu::Kucuk => cx.theme().radius,
            BilesenBoyutu::Buyuk => cx.theme().radius_lg,
            _ => cx.theme().radius_lg,
        }
    }

    fn radius(&self, size: BilesenBoyutu, cx: &App) -> Pixels {
        match self {
            SekmeVaryanti::Outline | SekmeVaryanti::Pill => px(99.),
            SekmeVaryanti::Segmented => match size {
                BilesenBoyutu::CokKucuk | BilesenBoyutu::Kucuk => cx.theme().radius,
                BilesenBoyutu::Buyuk => cx.theme().radius_lg,
                _ => cx.theme().radius_lg,
            },
            _ => px(0.),
        }
    }

    pub(super) fn inner_radius(&self, size: BilesenBoyutu, cx: &App) -> Pixels {
        match self {
            SekmeVaryanti::Segmented => match size {
                BilesenBoyutu::Buyuk => self.tab_bar_radius(size, cx) - px(3.),
                _ => self.tab_bar_radius(size, cx) - px(2.),
            },
            _ => px(0.),
        }
    }
}

#[allow(dead_code)]
struct TabStyle {
    borders: Edges<Pixels>,
    border_color: Hsla,
    bg: Hsla,
    fg: Hsla,
    shadow: bool,
    inner_bg: Hsla,
}

impl Default for TabStyle {
    fn default() -> Self {
        TabStyle {
            borders: Edges::all(px(0.)),
            border_color: gpui::transparent_white(),
            bg: gpui::transparent_white(),
            fg: gpui::transparent_white(),
            shadow: false,
            inner_bg: gpui::transparent_white(),
        }
    }
}

/// Bir Sekme öğe için [`super::SekmeCubugu`].
#[derive(IntoElement)]
pub struct Sekme {
    ix: usize,
    base: Div,
    pub(super) label: Option<SharedString>,
    pub(super) icon: Option<Simge>,
    prefix: Option<AnyElement>,
    pub(super) tab_bar_prefix: Option<bool>,
    suffix: Option<AnyElement>,
    children: Vec<AnyElement>,
    variant: SekmeVaryanti,
    size: BilesenBoyutu,
    pub(super) disabled: bool,
    pub(super) selected: bool,
    pub(super) indicator_active: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl From<&'static str> for Sekme {
    fn from(label: &'static str) -> Self {
        Self::new().label(label)
    }
}

impl From<String> for Sekme {
    fn from(label: String) -> Self {
        Self::new().label(label)
    }
}

impl From<SharedString> for Sekme {
    fn from(label: SharedString) -> Self {
        Self::new().label(label)
    }
}

impl From<Simge> for Sekme {
    fn from(icon: Simge) -> Self {
        Self::default().icon(icon)
    }
}

impl From<SimgeAdi> for Sekme {
    fn from(icon_name: SimgeAdi) -> Self {
        Self::default().icon(Simge::new(icon_name))
    }
}

impl Default for Sekme {
    fn default() -> Self {
        Self {
            ix: 0,
            base: div(),
            label: None,
            icon: None,
            tab_bar_prefix: None,
            children: Vec::new(),
            disabled: false,
            selected: false,
            indicator_active: false,
            prefix: None,
            suffix: None,
            variant: SekmeVaryanti::default(),
            size: BilesenBoyutu::default(),
            on_click: None,
        }
    }
}

impl Sekme {
    /// Yeni bir sekme ile bir etiket oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// etiket için sekme ayarlar.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// simge için sekme ayarlar.
    pub fn icon(mut self, icon: impl Into<Simge>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sekme Varyant ayarlar.
    pub fn with_variant(mut self, variant: SekmeVaryanti) -> Self {
        self.variant = variant;
        self
    }

    /// Kullanım Pill varyant.
    pub fn pill(mut self) -> Self {
        self.variant = SekmeVaryanti::Pill;
        self
    }

    /// Kullanım çerçeve varyant.
    pub fn outline(mut self) -> Self {
        self.variant = SekmeVaryanti::Outline;
        self
    }

    /// Kullanım Segmented varyant.
    pub fn segmented(mut self) -> Self {
        self.variant = SekmeVaryanti::Segmented;
        self
    }

    /// Kullanım Underline varyant.
    pub fn underline(mut self) -> Self {
        self.variant = SekmeVaryanti::Underline;
        self
    }

    /// sol taraf sekme ayarlar.
    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.prefix = Some(prefix.into_any_element());
        self
    }

    /// sağ taraf sekme ayarlar.
    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_any_element());
        self
    }

    /// devre dışı durum için sekme, varsayılan false ayarlar.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// tıklama işleyici için sekme ayarlar.
    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    /// indeks için sekme ayarlar.
    pub(crate) fn ix(mut self, ix: usize) -> Self {
        self.ix = ix;
        self
    }

    /// ise sekme çubuk sahip bir ön ek ayarlar.
    pub(crate) fn tab_bar_prefix(mut self, tab_bar_prefix: bool) -> Self {
        self.tab_bar_prefix = Some(tab_bar_prefix);
        self
    }
}

impl ParentElement for Sekme {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Secilebilir for Sekme {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl InteractiveElement for Sekme {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Sekme {}

impl Styled for Sekme {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl Boyutlandirilabilir for Sekme {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for Sekme {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut tab_style = if self.selected {
            self.variant.selected(cx)
        } else {
            self.variant.normal(cx)
        };
        let mut hover_style = self.variant.hovered(self.selected, cx);
        if self.disabled {
            tab_style = self.variant.disabled(self.selected, cx);
            hover_style = self.variant.disabled(self.selected, cx);
        }
        let tab_bar_prefix = self.tab_bar_prefix.unwrap_or_default();
        if !tab_bar_prefix {
            if self.ix == 0 && self.variant == SekmeVaryanti::Sekme {
                tab_style.borders.left = px(0.);
                hover_style.borders.left = px(0.);
            }
        }
        let radius = self.variant.radius(self.size, cx);
        let inner_radius = self.variant.inner_radius(self.size, cx);
        let inner_paddings = self.variant.inner_paddings(self.size);
        let inner_margins = self.variant.inner_margins(self.size);
        let inner_height = self.variant.inner_height(self.size);
        let height = self.variant.height(self.size);

        self.base
            .id(self.ix)
            .flex()
            .flex_wrap()
            .gap_1()
            .items_center()
            .flex_shrink_0()
            .h(height)
            .overflow_hidden()
            .text_color(tab_style.fg)
            .map(|this| match self.size {
                BilesenBoyutu::CokKucuk => this.text_xs(),
                BilesenBoyutu::Buyuk => this.text_base(),
                _ => this.text_sm(),
            })
            .bg(tab_style.bg)
            .border_l(tab_style.borders.left)
            .border_r(tab_style.borders.right)
            .border_t(tab_style.borders.top)
            .border_b(tab_style.borders.bottom)
            .border_color(tab_style.border_color)
            .rounded(radius)
            .when(!self.selected && !self.disabled, |this| {
                this.hover(|this| {
                    this.text_color(hover_style.fg)
                        .bg(hover_style.bg)
                        .border_l(hover_style.borders.left)
                        .border_r(hover_style.borders.right)
                        .border_t(hover_style.borders.top)
                        .border_b(hover_style.borders.bottom)
                        .border_color(hover_style.border_color)
                        .rounded(radius)
                })
            })
            .when_some(self.prefix, |this, prefix| this.child(prefix))
            .child(
                h_flex()
                    .flex_1()
                    .h(inner_height)
                    .line_height(relative(1.))
                    .whitespace_nowrap()
                    .items_center()
                    .justify_center()
                    .overflow_hidden()
                    .margins(inner_margins)
                    .flex_shrink_0()
                    .map(|this| match self.icon {
                        Some(icon) => {
                            this.w(inner_height * 1.25)
                                .child(icon.map(|this| match self.size {
                                    BilesenBoyutu::CokKucuk => this.size_2p5(),
                                    BilesenBoyutu::Kucuk => this.size_3p5(),
                                    BilesenBoyutu::Buyuk => this.size_4(),
                                    _ => this.size_4(),
                                }))
                        }
                        None => this
                            .paddings(inner_paddings)
                            .map(|this| match self.label {
                                Some(label) => this.child(label),
                                None => this,
                            })
                            .children(self.children),
                    })
                    .bg(tab_style.inner_bg)
                    .rounded(inner_radius)
                    .when(tab_style.shadow, |this| this.shadow_xs())
                    .hover(|this| this.bg(hover_style.inner_bg).rounded(inner_radius)),
            )
            .when_some(self.suffix, |this, suffix| this.child(suffix))
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                // Stop propagation behavior, for works on BaslikCubugu.
                // https://github.com/hakantr/kavis-ui/issues/1836
                cx.stop_propagation();
            })
            .when(!self.disabled, |this| {
                this.when_some(self.on_click.clone(), |this, on_click| {
                    this.on_click(move |event, window, cx| on_click(event, window, cx))
                })
            })
    }
}
