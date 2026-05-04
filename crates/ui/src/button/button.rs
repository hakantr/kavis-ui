use std::rc::Rc;

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, EtkinTema,
    OdaklanabilirUzantisi as _, Renklendir as _, Secilebilir, Simge, SimgeAdi, StilBoyutlandirma,
    StilUzantisi,
    button::DugmeSimgesi,
    h_flex,
    tooltip::{AracIpucu, YonetilenAracIpucuUzantisi as _},
};
use crate::ham_gpui::{
    AnyElement, App, ClickEvent, Corners, Div, Edges, ElementId, Hsla, InteractiveElement,
    Interactivity, IntoElement, MouseButton, ParentElement, Pixels, RenderOnce, SharedString,
    Stateful, StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative, transparent_white,
};

#[derive(Default, Clone, Copy)]
pub enum DugmeYuvarlakligi {
    None,
    Small,
    #[default]
    Medium,
    Large,
    Size(Pixels),
}

impl From<Pixels> for DugmeYuvarlakligi {
    fn from(px: Pixels) -> Self {
        DugmeYuvarlakligi::Size(px)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DugmeOzelVaryanti {
    color: Hsla,
    foreground: Hsla,
    shadow: bool,
    hover: Hsla,
    active: Hsla,
}

pub trait DugmeVaryantlari: Sized {
    fn with_variant(self, variant: DugmeVaryanti) -> Self;

    /// İle birincil stil için Dugme.
    fn primary(self) -> Self {
        self.with_variant(DugmeVaryanti::Primary)
    }

    /// İle ikincil stil için Dugme.
    fn secondary(self) -> Self {
        self.with_variant(DugmeVaryanti::Secondary)
    }

    /// İle danger stil için Dugme.
    fn danger(self) -> Self {
        self.with_variant(DugmeVaryanti::Danger)
    }

    /// İle uyarı stil için Dugme.
    fn warning(self) -> Self {
        self.with_variant(DugmeVaryanti::Warning)
    }

    /// İle başarı stil için Dugme.
    fn success(self) -> Self {
        self.with_variant(DugmeVaryanti::Success)
    }

    /// İle bilgi stil için Dugme.
    fn info(self) -> Self {
        self.with_variant(DugmeVaryanti::Info)
    }

    /// İle ghost stil için Dugme.
    fn ghost(self) -> Self {
        self.with_variant(DugmeVaryanti::Ghost)
    }

    /// İle link stil için Dugme.
    fn link(self) -> Self {
        self.with_variant(DugmeVaryanti::Link)
    }

    /// Metin stiliyle Düğme dolgusuz hale gelir ve normal metin gibi görünür.
    fn text(self) -> Self {
        self.with_variant(DugmeVaryanti::Text)
    }

    /// İle özel stil için Dugme.
    fn custom(self, style: DugmeOzelVaryanti) -> Self {
        self.with_variant(DugmeVaryanti::Custom(style))
    }
}

impl DugmeOzelVaryanti {
    pub fn new(cx: &App) -> Self {
        Self {
            color: cx.theme().transparent,
            foreground: cx.theme().foreground,
            hover: cx.theme().transparent,
            active: cx.theme().transparent,
            shadow: false,
        }
    }

    /// Arka plan rengini ayarlar. Varsayılan transparent değeridir.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    /// Ön plan rengini ayarlar. Varsayılan tema ön plan rengidir.
    pub fn foreground(mut self, color: Hsla) -> Self {
        self.foreground = color;
        self
    }

    /// üzerine gelme Arka plan rengini ayarlar. Varsayılan transparent değeridir.
    pub fn hover(mut self, color: Hsla) -> Self {
        self.hover = color;
        self
    }

    /// etkin Arka plan rengini ayarlar. Varsayılan transparent değeridir.
    pub fn active(mut self, color: Hsla) -> Self {
        self.active = color;
        self
    }

    /// Gölgeyi ayarlar. Varsayılan false değeridir.
    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }
}

/// varyant Dugme.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum DugmeVaryanti {
    #[default]
    Default,
    Primary,
    Secondary,
    Danger,
    Info,
    Success,
    Warning,
    Ghost,
    Link,
    Text,
    Custom(DugmeOzelVaryanti),
}

impl DugmeVaryanti {
    #[inline]
    pub fn is_link(&self) -> bool {
        matches!(self, Self::Link)
    }

    #[inline]
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text)
    }

    #[inline]
    pub fn is_ghost(&self) -> bool {
        matches!(self, Self::Ghost)
    }

    #[inline]
    fn no_padding(&self) -> bool {
        self.is_link() || self.is_text()
    }

    #[inline]
    fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }
}

/// Bir Dugme öğe.
#[derive(IntoElement)]
pub struct Dugme {
    id: ElementId,
    base: Stateful<Div>,
    style: StyleRefinement,
    icon: Option<DugmeSimgesi>,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    disabled: bool,
    pub(crate) selected: bool,
    variant: DugmeVaryanti,
    rounded: DugmeYuvarlakligi,
    outline: bool,
    border_corners: Corners<bool>,
    border_edges: Edges<bool>,
    dropdown_caret: bool,
    size: BilesenBoyutu,
    compact: bool,
    tooltip: Option<(
        SharedString,
        Option<(Rc<Box<dyn crate::ham_gpui::Action>>, Option<SharedString>)>,
    )>,
    tooltip_builder: Option<Rc<dyn Fn(&mut Window, &mut App) -> crate::ham_gpui::AnyView>>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    on_hover: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    loading: bool,
    loading_icon: Option<Simge>,
    group_hover_name: Option<SharedString>,

    tab_index: isize,
    tab_stop: bool,
}

impl From<Dugme> for AnyElement {
    fn from(button: Dugme) -> Self {
        button.into_any_element()
    }
}

impl Dugme {
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();

        Self {
            id: id.clone(),
            // ID must be set after div is created;
            // `acilir_menu` uses this id to create the popup menu.
            base: div().flex_shrink_0().id(id),
            style: StyleRefinement::default(),
            icon: None,
            label: None,
            disabled: false,
            selected: false,
            variant: DugmeVaryanti::default(),
            rounded: DugmeYuvarlakligi::Medium,
            border_corners: Corners {
                top_left: true,
                top_right: true,
                bottom_right: true,
                bottom_left: true,
            },
            border_edges: Edges::all(true),
            size: BilesenBoyutu::Orta,
            tooltip: None,
            tooltip_builder: None,
            on_click: None,
            on_hover: None,
            loading: false,
            compact: false,
            outline: false,
            children: Vec::new(),
            loading_icon: None,
            group_hover_name: None,
            dropdown_caret: false,
            tab_index: 0,
            tab_stop: true,
        }
    }

    /// çerçeve stil Dugme ayarlar.
    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    /// kenarlık yarıçap Dugme ayarlar.
    pub fn rounded(mut self, rounded: impl Into<DugmeYuvarlakligi>) -> Self {
        self.rounded = rounded.into();
        self
    }

    /// kenarlık corners taraf Dugme ayarlar.
    pub(crate) fn border_corners(mut self, corners: impl Into<Corners<bool>>) -> Self {
        self.border_corners = corners.into();
        self
    }

    /// kenarlık edges Dugme ayarlar.
    pub(crate) fn border_edges(mut self, edges: impl Into<Edges<bool>>) -> Self {
        self.border_edges = edges.into();
        self
    }

    /// Düğme etiketini ayarlar; etiket yoksa düğme Simge Düğme moduna geçer.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// simge düğme, ise Dugme sahip yok etiket, düğme well içinde Simge Dugme mod ayarlar.
    pub fn icon(mut self, icon: impl Into<DugmeSimgesi>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// araç ipucu düğme ayarlar.
    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some((tooltip.into(), None));
        self
    }

    /// araç ipucu düğme ile eylem göstermek için keybinding ayarlar.
    pub fn tooltip_with_action(
        mut self,
        tooltip: impl Into<SharedString>,
        action: &dyn crate::ham_gpui::Action,
        context: Option<&str>,
    ) -> Self {
        self.tooltip = Some((
            tooltip.into(),
            Some((
                Rc::new(action.boxed_clone()),
                context.map(|c| c.to_string().into()),
            )),
        ));
        self
    }

    /// true göstermek için yükleme gösterge ayarlar.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Düğmenin kompakt modunu ayarlar; bu modda dolgu azaltılır.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// tıklama işleyici. ekler.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// Üzerine gelme işleyicisi ekler; bool parametresi farenin öğenin üzerinde olup olmadığını belirtir.
    pub fn on_hover(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_hover = Some(Rc::new(handler));
        self
    }

    /// Yükleme true olduğunda kullanılacak düğme yükleme simgesini ayarlar.
    ///
    /// Varsayılan bir spinner simgesidir.
    pub fn loading_icon(mut self, icon: impl Into<Simge>) -> Self {
        self.loading_icon = Some(icon.into());
        self
    }

    /// Düğmenin sekme indeksini ayarlar; sekme tuşuyla odaklanmada kullanılır.
    ///
    /// Varsayılan 0dır.
    pub fn tab_index(mut self, tab_index: isize) -> Self {
        self.tab_index = tab_index;
        self
    }

    /// Düğmenin sekme durağını ayarlar; true ise düğme sekme tuşuyla odaklanabilir.
    ///
    /// Varsayılan true değeridir.
    pub fn tab_stop(mut self, tab_stop: bool) -> Self {
        self.tab_stop = tab_stop;
        self
    }

    /// Düğmenin sonunda açılır caret simgesi gösterilip gösterilmeyeceğini ayarlar.
    pub fn dropdown_caret(mut self, dropdown_caret: bool) -> Self {
        self.dropdown_caret = dropdown_caret;
        self
    }

    /// Üzerine gelme stilinin, verilen ad ile işaretlenmiş üst grubun üzerine
    /// gelindiğinde de uygulanmasını sağlar; böylece kardeş düğmeler birlikte
    /// vurgulanabilir.
    pub(crate) fn group_hover_with(mut self, name: impl Into<SharedString>) -> Self {
        self.group_hover_name = Some(name.into());
        self
    }

    #[inline]
    fn clickable(&self) -> bool {
        !(self.disabled || self.loading) && self.on_click.is_some()
    }

    #[inline]
    fn hoverable(&self) -> bool {
        !(self.disabled || self.loading) && self.on_hover.is_some()
    }
}

impl DevreDisiBirakilabilir for Dugme {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Secilebilir for Dugme {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl Boyutlandirilabilir for Dugme {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl DugmeVaryantlari for Dugme {
    fn with_variant(mut self, variant: DugmeVaryanti) -> Self {
        self.variant = variant;
        self
    }
}

impl Styled for Dugme {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Dugme {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl InteractiveElement for Dugme {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl RenderOnce for Dugme {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let style: DugmeVaryanti = self.variant;
        let clickable = self.clickable();
        let is_disabled = self.disabled;
        let hoverable = self.hoverable();
        let normal_style = style.normal(self.outline, cx);
        let icon_size = match self.size {
            BilesenBoyutu::Ozel(v) => BilesenBoyutu::Ozel(v * 0.75),
            _ => self.size,
        };

        let focus_handle = window
            .use_keyed_state(self.id.clone(), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let is_focused = focus_handle.is_focused(window);

        let rounding = match self.rounded {
            DugmeYuvarlakligi::Small => cx.theme().radius * 0.5,
            DugmeYuvarlakligi::Medium => cx.theme().radius,
            DugmeYuvarlakligi::Large => cx.theme().radius * 2.0,
            DugmeYuvarlakligi::Size(px) => px,
            DugmeYuvarlakligi::None => Pixels::ZERO,
        };

        self.base
            .when(!self.disabled, |this| {
                this.track_focus(
                    &focus_handle
                        .tab_index(self.tab_index)
                        .tab_stop(self.tab_stop),
                )
            })
            .cursor_default()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .cursor_default()
            .when(self.variant.is_link(), |this| this.cursor_pointer())
            .when(cx.theme().shadow && normal_style.shadow, |this| {
                this.shadow_xs()
            })
            .when(!style.no_padding(), |this| {
                if self.label.is_none() && self.children.is_empty() {
                    // Simge Dugme
                    match self.size {
                        BilesenBoyutu::Ozel(px) => this.size(px),
                        BilesenBoyutu::CokKucuk => this.size_5(),
                        BilesenBoyutu::Kucuk => this.size_6(),
                        BilesenBoyutu::Buyuk | BilesenBoyutu::Orta => this.size_8(),
                    }
                } else {
                    // Normal Dugme
                    match self.size {
                        BilesenBoyutu::Ozel(size) => this.px(size * 0.2),
                        BilesenBoyutu::CokKucuk => {
                            this.h_5().px_1().when(self.compact, |this| this.min_w_5())
                        }
                        BilesenBoyutu::Kucuk => this
                            .h_6()
                            .px_3()
                            .when(self.compact, |this| this.min_w_6().px_1p5()),
                        _ => this
                            .h_8()
                            .px_4()
                            .when(self.compact, |this| this.min_w_8().px_2()),
                    }
                }
            })
            .when(self.border_corners.top_left, |this| {
                this.rounded_tl(rounding)
            })
            .when(self.border_corners.top_right, |this| {
                this.rounded_tr(rounding)
            })
            .when(self.border_corners.bottom_left, |this| {
                this.rounded_bl(rounding)
            })
            .when(self.border_corners.bottom_right, |this| {
                this.rounded_br(rounding)
            })
            .when(self.variant.is_default() || self.outline, |this| {
                this.when(self.border_edges.left, |this| this.border_l_1())
                    .when(self.border_edges.right, |this| this.border_r_1())
                    .when(self.border_edges.top, |this| this.border_t_1())
                    .when(self.border_edges.bottom, |this| this.border_b_1())
            })
            .text_color(normal_style.fg)
            .when(self.selected, |this| {
                let selected_style = style.selected(self.outline, cx);
                this.bg(selected_style.bg)
                    .border_color(selected_style.border)
                    .text_color(selected_style.fg)
            })
            .when(!self.disabled && !self.selected, |this| {
                let hover_style = style.hovered(self.outline, cx);
                this.border_color(normal_style.border)
                    .bg(normal_style.bg)
                    .when(normal_style.underline, |this| this.text_decoration_1())
                    .hover(|this| {
                        this.bg(hover_style.bg)
                            .border_color(hover_style.border)
                            .text_color(hover_style.fg)
                    })
                    .when_some(self.group_hover_name.clone(), |this, name| {
                        this.group_hover(name, |this| {
                            this.bg(hover_style.bg)
                                .border_color(hover_style.border)
                                .text_color(hover_style.fg)
                        })
                    })
                    .active(|this| {
                        let active_style = style.active(self.outline, cx);
                        this.bg(active_style.bg)
                            .border_color(active_style.border)
                            .text_color(active_style.fg)
                    })
            })
            .when(self.disabled, |this| {
                let disabled_style = style.disabled(self.outline, cx);
                this.bg(disabled_style.bg)
                    .text_color(disabled_style.fg)
                    .border_color(disabled_style.border)
                    .shadow_none()
            })
            .refine_style(&self.style)
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                // Stop handle any click event when disabled.
                // To avoid handle dropdown menu open when button is disabled.
                if is_disabled {
                    cx.stop_propagation();
                    return;
                }

                // Avoid focus on mouse down.
                window.prevent_default();
            })
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |event, window, cx| {
                    // Stop handle any click event when disabled.
                    // To avoid handle dropdown menu open when button is disabled.
                    if !clickable {
                        cx.stop_propagation();
                        return;
                    }

                    on_click(event, window, cx);
                })
            })
            .when_some(self.on_hover.filter(|_| hoverable), |this, on_hover| {
                this.on_hover(move |hovered, window, cx| {
                    on_hover(hovered, window, cx);
                })
            })
            .child({
                h_flex()
                    .id("label")
                    .size_full()
                    .items_center()
                    .justify_center()
                    .button_text_size(self.size)
                    .map(|this| match self.size {
                        BilesenBoyutu::CokKucuk => this.gap_1(),
                        BilesenBoyutu::Kucuk => this.gap_1(),
                        _ => this.gap_2(),
                    })
                    .when_some(self.icon, |this, icon| {
                        this.child(
                            icon.loading_icon(self.loading_icon)
                                .loading(self.loading)
                                .with_size(icon_size),
                        )
                    })
                    .when_some(self.label, |this, label| {
                        this.child(div().flex_none().line_height(relative(1.)).child(label))
                    })
                    .children(self.children)
                    .when(self.dropdown_caret, |this| {
                        this.justify_between().child(
                            Simge::new(SimgeAdi::ChevronDown).xsmall().text_color(
                                match self.disabled {
                                    true => normal_style.fg.opacity(0.3),
                                    false => normal_style.fg.opacity(0.5),
                                },
                            ),
                        )
                    })
            })
            .when(self.loading && !self.disabled, |this| {
                this.bg(normal_style.bg.opacity(0.8))
                    .border_color(normal_style.border.opacity(0.8))
                    .text_color(normal_style.fg.opacity(0.8))
            })
            .map(|this| {
                if let Some(builder) = self.tooltip_builder {
                    this.managed_tooltip(move |window, cx| builder(window, cx))
                } else if let Some((tooltip, action)) = self.tooltip {
                    this.managed_tooltip(move |window, cx| {
                        AracIpucu::new(tooltip.clone())
                            .when_some(action.clone(), |this, (action, context)| {
                                this.action(
                                    action.boxed_clone().as_ref(),
                                    context.as_ref().map(|c| c.as_ref()),
                                )
                            })
                            .build(window, cx)
                    })
                } else {
                    this
                }
            })
            .odak_halkasi(is_focused, px(0.), window, cx)
    }
}

struct ButtonVariantStyle {
    bg: Hsla,
    border: Hsla,
    fg: Hsla,
    underline: bool,
    shadow: bool,
}

impl DugmeVaryanti {
    fn bg_color(&self, outline: bool, cx: &mut App) -> Hsla {
        if outline {
            return cx.theme().input_background();
        }

        match self {
            Self::Default => cx.theme().input_background(),
            Self::Primary => cx.theme().button_primary,
            Self::Secondary => cx.theme().secondary,
            Self::Danger => cx.theme().danger.mix_oklab(cx.theme().transparent, 0.2),
            Self::Warning => cx.theme().warning.mix_oklab(cx.theme().transparent, 0.2),
            Self::Success => cx.theme().success.mix_oklab(cx.theme().transparent, 0.2),
            Self::Info => cx.theme().info.mix_oklab(cx.theme().transparent, 0.2),
            Self::Ghost | Self::Link | Self::Text => cx.theme().transparent,
            Self::Custom(colors) => colors.color.mix_oklab(cx.theme().transparent, 0.2),
        }
    }

    fn text_color(&self, outline: bool, cx: &mut App) -> Hsla {
        match self {
            Self::Default => cx.theme().foreground,
            Self::Primary => {
                if outline {
                    cx.theme().button_primary
                } else {
                    cx.theme().button_primary_foreground
                }
            }
            Self::Secondary | Self::Ghost => cx.theme().secondary_foreground,
            Self::Danger => cx.theme().danger,
            Self::Warning => cx.theme().warning,
            Self::Success => cx.theme().success,
            Self::Info => cx.theme().info,
            Self::Link => cx.theme().link,
            Self::Text => cx.theme().foreground,
            Self::Custom(colors) => colors.color,
        }
    }

    fn border_color(&self, _bg: Hsla, outline: bool, cx: &mut App) -> Hsla {
        match self {
            Self::Default => cx.theme().input,
            Self::Secondary => cx.theme().border,
            Self::Primary => cx.theme().button_primary,
            Self::Danger => {
                if outline {
                    cx.theme().danger.mix_oklab(transparent_white(), 0.4)
                } else {
                    cx.theme().danger
                }
            }
            Self::Info => {
                if outline {
                    cx.theme().info.mix_oklab(transparent_white(), 0.4)
                } else {
                    cx.theme().info
                }
            }
            Self::Warning => {
                if outline {
                    cx.theme().warning.mix_oklab(transparent_white(), 0.4)
                } else {
                    cx.theme().warning
                }
            }
            Self::Success => {
                if outline {
                    cx.theme().success.mix_oklab(transparent_white(), 0.4)
                } else {
                    cx.theme().success
                }
            }
            Self::Ghost | Self::Link | Self::Text => cx.theme().transparent,
            Self::Custom(colors) => {
                if outline {
                    colors.color.mix_oklab(transparent_white(), 0.4)
                } else {
                    colors.color
                }
            }
        }
    }

    fn underline(&self, _: &App) -> bool {
        match self {
            Self::Link => true,
            _ => false,
        }
    }

    fn shadow(&self, _: bool, _: &App) -> bool {
        match self {
            Self::Default
            | Self::Primary
            | Self::Secondary
            | Self::Danger
            | Self::Info
            | Self::Success
            | Self::Warning => true,
            Self::Custom(c) => c.shadow,
            _ => false,
        }
    }

    fn normal(&self, outline: bool, cx: &mut App) -> ButtonVariantStyle {
        let bg = self.bg_color(outline, cx);
        let border = self.border_color(bg, outline, cx);
        let fg = self.text_color(outline, cx);
        let underline = self.underline(cx);
        let shadow = self.shadow(outline, cx);

        ButtonVariantStyle {
            bg,
            border,
            fg,
            underline,
            shadow,
        }
    }

    fn hovered(&self, outline: bool, cx: &mut App) -> ButtonVariantStyle {
        let bg = match self {
            Self::Default => cx.theme().input.mix_oklab(cx.theme().transparent, 0.5),
            Self::Primary => {
                if outline {
                    cx.theme()
                        .button_primary
                        .mix_oklab(cx.theme().transparent, 0.2)
                } else {
                    cx.theme().button_primary_hover
                }
            }
            Self::Secondary => cx.theme().secondary_hover,
            Self::Danger => {
                if outline {
                    cx.theme().danger.mix_oklab(cx.theme().transparent, 0.2)
                } else {
                    cx.theme().danger.mix_oklab(cx.theme().transparent, 0.3)
                }
            }
            Self::Warning => {
                if outline {
                    cx.theme().warning.mix_oklab(cx.theme().transparent, 0.2)
                } else {
                    cx.theme().warning.mix_oklab(cx.theme().transparent, 0.3)
                }
            }
            Self::Success => {
                if outline {
                    cx.theme().success.mix_oklab(cx.theme().transparent, 0.2)
                } else {
                    cx.theme().success.mix_oklab(cx.theme().transparent, 0.3)
                }
            }
            Self::Info => {
                if outline {
                    cx.theme().info.mix_oklab(cx.theme().transparent, 0.2)
                } else {
                    cx.theme().info.mix_oklab(cx.theme().transparent, 0.3)
                }
            }
            Self::Custom(colors) => {
                if outline {
                    colors.color.mix_oklab(cx.theme().transparent, 0.2)
                } else {
                    colors.color.mix_oklab(cx.theme().transparent, 0.3)
                }
            }
            Self::Ghost => {
                if cx.theme().mode.is_dark() {
                    cx.theme().secondary.lighten(0.1).opacity(0.8)
                } else {
                    cx.theme().secondary.darken(0.1).opacity(0.8)
                }
            }
            Self::Link => cx.theme().transparent,
            Self::Text => cx.theme().transparent,
        };

        let border = self.border_color(bg, outline, cx);
        let fg = match self {
            Self::Link => cx.theme().link_hover,
            _ => self.text_color(outline, cx),
        };

        let underline = self.underline(cx);
        let shadow = self.shadow(outline, cx);

        ButtonVariantStyle {
            bg,
            border,
            fg,
            underline,
            shadow,
        }
    }

    fn active(&self, outline: bool, cx: &mut App) -> ButtonVariantStyle {
        let bg = match self {
            Self::Default => cx.theme().input.mix_oklab(cx.theme().transparent, 0.7),
            Self::Primary => {
                if outline {
                    cx.theme()
                        .button_primary
                        .mix_oklab(cx.theme().transparent, 0.4)
                } else {
                    cx.theme().button_primary_active
                }
            }
            Self::Secondary => cx.theme().secondary_active,
            Self::Ghost => {
                if cx.theme().mode.is_dark() {
                    cx.theme().secondary.lighten(0.2).opacity(0.8)
                } else {
                    cx.theme().secondary.darken(0.2).opacity(0.8)
                }
            }
            Self::Danger => cx.theme().danger.mix_oklab(cx.theme().transparent, 0.4),
            Self::Warning => cx.theme().warning.mix_oklab(cx.theme().transparent, 0.4),
            Self::Success => cx.theme().success.mix_oklab(cx.theme().transparent, 0.4),
            Self::Info => cx.theme().info.mix_oklab(cx.theme().transparent, 0.4),
            Self::Custom(colors) => colors.color.mix_oklab(cx.theme().transparent, 0.4),
            Self::Link => cx.theme().transparent,
            Self::Text => cx.theme().transparent,
        };
        let border = self.border_color(bg, outline, cx);
        let fg = match self {
            Self::Link => cx.theme().link_active,
            Self::Text => cx.theme().foreground.opacity(0.7),
            _ => self.text_color(outline, cx),
        };
        let underline = self.underline(cx);
        let shadow = self.shadow(outline, cx);

        ButtonVariantStyle {
            bg,
            border,
            fg,
            underline,
            shadow,
        }
    }

    fn selected(&self, outline: bool, cx: &mut App) -> ButtonVariantStyle {
        let bg = match self {
            Self::Default => cx.theme().input.mix_oklab(cx.theme().transparent, 0.7),
            Self::Primary => cx.theme().button_primary_active,
            Self::Secondary | Self::Ghost => cx.theme().secondary_active,
            Self::Danger => cx.theme().danger_active,
            Self::Warning => cx.theme().warning_active,
            Self::Success => cx.theme().success_active,
            Self::Info => cx.theme().info_active,
            Self::Link => cx.theme().transparent,
            Self::Text => cx.theme().transparent,
            Self::Custom(colors) => colors.active,
        };

        let border = self.border_color(bg, outline, cx);
        let fg = match self {
            Self::Link => cx.theme().link_active,
            Self::Text => cx.theme().foreground.opacity(0.7),
            _ => self.text_color(false, cx),
        };
        let underline = self.underline(cx);
        let shadow = self.shadow(outline, cx);

        ButtonVariantStyle {
            bg,
            border,
            fg,
            underline,
            shadow,
        }
    }

    fn disabled(&self, outline: bool, cx: &mut App) -> ButtonVariantStyle {
        let bg = match self {
            Self::Default | Self::Link | Self::Ghost | Self::Text => cx.theme().transparent,
            Self::Primary => cx.theme().button_primary.opacity(0.15),
            Self::Danger => cx.theme().danger.opacity(0.15),
            Self::Warning => cx.theme().warning.opacity(0.15),
            Self::Success => cx.theme().success.opacity(0.15),
            Self::Info => cx.theme().info.opacity(0.15),
            Self::Secondary => cx.theme().secondary.opacity(1.5),
            Self::Custom(style) => style.color.opacity(0.15),
        };
        let fg = cx.theme().muted_foreground.opacity(0.5);
        let (bg, border) = if outline {
            (
                cx.theme().input_background().opacity(0.5),
                cx.theme().border.opacity(0.5),
            )
        } else if let Self::Default = self {
            (
                cx.theme().input_background().opacity(0.5),
                cx.theme().input.opacity(0.5),
            )
        } else {
            (bg, bg)
        };

        let underline = self.underline(cx);
        let shadow = false;

        ButtonVariantStyle {
            bg,
            border,
            fg,
            underline,
            shadow,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[crate::ham_gpui::test]
    fn test_button_builder(_cx: &mut crate::ham_gpui::TestAppContext) {
        let button = Dugme::new("complex-button")
            .label("Save Changes")
            .primary()
            .outline()
            .large()
            .tooltip("Click to save")
            .compact()
            .loading(false)
            .disabled(false)
            .selected(false)
            .tab_index(1)
            .tab_stop(true)
            .dropdown_caret(false)
            .rounded(DugmeYuvarlakligi::Medium)
            .on_click(|_, _, _| {});

        assert_eq!(button.label, Some("Save Changes".into()));
        assert_eq!(button.variant, DugmeVaryanti::Primary);
        assert!(button.outline);
        assert_eq!(button.size, BilesenBoyutu::Buyuk);
        assert!(button.tooltip.is_some());
        assert!(button.compact);
        assert!(!button.loading);
        assert!(!button.disabled);
        assert!(!button.selected);
        assert_eq!(button.tab_index, 1);
        assert!(button.tab_stop);
        assert!(!button.dropdown_caret);
        assert!(matches!(button.rounded, DugmeYuvarlakligi::Medium));
    }

    #[crate::ham_gpui::test]
    fn test_button_clickable_logic(_cx: &mut crate::ham_gpui::TestAppContext) {
        // Dugme with click handler should be clickable
        let clickable = Dugme::new("test").on_click(|_, _, _| {});
        assert!(clickable.clickable());

        // Disabled button should not be clickable
        let disabled = Dugme::new("test").disabled(true).on_click(|_, _, _| {});
        assert!(!disabled.clickable());

        // Loading button should not be clickable
        let loading = Dugme::new("test").loading(true).on_click(|_, _, _| {});
        assert!(!loading.clickable());
    }

    #[crate::ham_gpui::test]
    fn test_button_variant_methods(_cx: &mut crate::ham_gpui::TestAppContext) {
        // Test variant check methods
        assert!(DugmeVaryanti::Link.is_link());
        assert!(DugmeVaryanti::Text.is_text());
        assert!(DugmeVaryanti::Ghost.is_ghost());

        // Test no_padding logic
        assert!(DugmeVaryanti::Link.no_padding());
        assert!(DugmeVaryanti::Text.no_padding());
        assert!(!DugmeVaryanti::Ghost.no_padding());
    }
}
