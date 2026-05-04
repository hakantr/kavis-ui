use crate::ham_gpui::{
    AbsoluteLength, AnyElement, App, ClickEvent, DefiniteLength, EdgesRefinement, ElementId,
    InteractiveElement as _, IntoElement, Length, ListAlignment, ListState as ListeDurumu,
    ParentElement, Pixels, RenderOnce, SharedString, StyleRefinement, Styled, Window, div, list,
    prelude::FluentBuilder, px,
};
use crate::{
    Boyutlandirilabilir, Daraltilabilir, EtkinTema, Side, Simge, SimgeAdi, StilUzantisi,
    button::{Dugme, DugmeVaryantlari},
    h_flex,
    scroll::KaydirilabilirOge,
    v_flex,
};
use std::{rc::Rc, time::Duration};

use crate::animation::{Transition, ease_in_out_cubic};

mod footer;
mod group;
mod header;
mod menu;
pub use footer::*;
pub use group::*;
pub use header::*;
pub use menu::*;

const DEFAULT_WIDTH: Pixels = px(255.);
const COLLAPSED_WIDTH: Pixels = px(48.);
const SIDEBAR_TRANSITION_DURATION: Duration = Duration::from_millis(200);

/// [`YanCubuk`] daraltıldığında nasıl davranacağını belirler.
///
/// - [`YanCubukDaralma::Icon`] yan çubuğu simge genişliğine indirir.
/// - [`YanCubukDaralma::Offcanvas`] yan çubuğu düzen dışına kaydırır ve genişliği serbest bırakır.
/// - [`YanCubukDaralma::None`] daraltma durumunu yok sayar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum YanCubukDaralma {
    /// Yan çubuğu simge genişliğine indir.
    #[default]
    Icon,
    /// Yan çubuğu tamamen düzen dışına kaydır.
    Offcanvas,
    /// Daraltmayı devre dışı bırak.
    None,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SidebarWrapperLayout {
    None,
    Static { width: Pixels },
    Animated { target_width: Pixels },
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SidebarLayout {
    icon_collapsed: bool,
    offcanvas_collapsed: bool,
    align_child_to_end: bool,
    wrapper: SidebarWrapperLayout,
}

impl SidebarLayout {
    fn new(
        collapsible: YanCubukDaralma,
        collapsed: bool,
        expanded_width: Option<Pixels>,
        side: Side,
    ) -> Self {
        let collapsed = collapsed && collapsible != YanCubukDaralma::None;
        let wrapper = match collapsible {
            YanCubukDaralma::None => SidebarWrapperLayout::None,
            YanCubukDaralma::Icon => match expanded_width {
                Some(expanded_width) => SidebarWrapperLayout::Animated {
                    target_width: if collapsed {
                        COLLAPSED_WIDTH
                    } else {
                        expanded_width
                    },
                },
                None => SidebarWrapperLayout::None,
            },
            YanCubukDaralma::Offcanvas => match (expanded_width, collapsed) {
                (Some(_), true) => SidebarWrapperLayout::Animated {
                    target_width: px(0.),
                },
                (Some(expanded_width), false) => SidebarWrapperLayout::Animated {
                    target_width: expanded_width,
                },
                (None, true) => SidebarWrapperLayout::Static { width: px(0.) },
                (None, false) => SidebarWrapperLayout::None,
            },
        };
        let align_child_to_end = match collapsible {
            YanCubukDaralma::Offcanvas => side.is_left(),
            _ => side.is_right(),
        };

        Self {
            icon_collapsed: collapsed && collapsible == YanCubukDaralma::Icon,
            offcanvas_collapsed: collapsed && collapsible == YanCubukDaralma::Offcanvas,
            align_child_to_end,
            wrapper,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SidebarAnimationState {
    from_width: Pixels,
    target_width: Pixels,
    render_child: bool,
    hide_scheduled: bool,
    hide_request: u64,
}

impl SidebarAnimationState {
    fn new(target_width: Pixels, render_child: bool) -> Self {
        Self {
            from_width: target_width,
            target_width,
            render_child,
            hide_scheduled: false,
            hide_request: 0,
        }
    }

    fn needs_update(&self, target_width: Pixels, offcanvas_collapsed: bool) -> bool {
        let child_state_changed = if offcanvas_collapsed {
            self.render_child && !self.hide_scheduled
        } else {
            !self.render_child || self.hide_scheduled
        };

        self.target_width != target_width || child_state_changed
    }

    fn update_target(&mut self, target_width: Pixels, offcanvas_collapsed: bool) -> Option<u64> {
        if self.target_width != target_width {
            self.from_width = self.target_width;
            self.target_width = target_width;
        }

        if offcanvas_collapsed {
            if self.render_child && !self.hide_scheduled {
                self.hide_scheduled = true;
                self.hide_request = self.hide_request.wrapping_add(1);
                Some(self.hide_request)
            } else {
                None
            }
        } else {
            self.render_child = true;
            if self.hide_scheduled {
                self.hide_request = self.hide_request.wrapping_add(1);
            }
            self.hide_scheduled = false;
            None
        }
    }

    fn finish_hide(&mut self, request: u64) -> bool {
        if self.render_child
            && self.hide_scheduled
            && self.hide_request == request
            && self.target_width == px(0.)
        {
            self.render_child = false;
            self.hide_scheduled = false;
            true
        } else {
            false
        }
    }
}

fn sidebar_wrapper(
    id: impl Into<ElementId>,
    align_child_to_end: bool,
) -> impl ParentElement + IntoElement + Styled {
    div()
        .id(id)
        .flex()
        .h_full()
        .flex_shrink_0()
        .overflow_hidden()
        .when(align_child_to_end, |this| this.justify_end())
}

fn sidebar_expanded_width(style: &StyleRefinement) -> Option<Pixels> {
    match style.size.width {
        Some(Length::Definite(DefiniteLength::Absolute(AbsoluteLength::Pixels(px)))) => Some(px),
        Some(_) => None,
        None => Some(DEFAULT_WIDTH),
    }
}

fn sidebar_animation_id(id: &ElementId, from: Pixels, to: Pixels) -> ElementId {
    ElementId::NamedInteger(
        format!("{id}-anim-w").into(),
        (from.as_f32().to_bits() as u64) << 32 | to.as_f32().to_bits() as u64,
    )
}

pub trait YanCubukOgesi: Daraltilabilir + Clone {
    fn render(
        self,
        id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement;
}

/// Daraltılabilir alt öğeler içerebilen bir YanCubuk öğesi.
#[derive(IntoElement)]
pub struct YanCubuk<E: YanCubukOgesi + 'static> {
    id: ElementId,
    style: StyleRefinement,
    content: Vec<E>,
    /// başlık görünüm
    header: Option<AnyElement>,
    /// alt bilgi görünüm
    footer: Option<AnyElement>,
    /// taraf sidebar
    side: Side,
    collapsible: YanCubukDaralma,
    collapsed: bool,
    ust_bosluk: Pixels,
    alt_bosluk: Pixels,
}

impl<E: YanCubukOgesi> YanCubuk<E> {
    /// Yeni bir YanCubuk ile verilen ID oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            content: vec![],
            header: None,
            footer: None,
            side: Side::Left,
            collapsible: YanCubukDaralma::Icon,
            collapsed: false,
            ust_bosluk: px(0.0),
            alt_bosluk: px(0.0),
        }
    }

    /// taraf sidebar ayarlar.
    ///
    /// varsayılandır `Side::Left`.
    pub fn ust_bosluk(mut self, ust_bosluk: Pixels) -> Self {
        self.ust_bosluk = ust_bosluk;
        self
    }

    pub fn alt_bosluk(mut self, alt_bosluk: Pixels) -> Self {
        self.alt_bosluk = alt_bosluk;
        self
    }
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// Yan çubuğun nasıl daralacağını ayarlar.
    pub fn collapsible(mut self, collapsible: YanCubukDaralma) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// sidebar olmak için daraltılmış ayarlar.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// başlık sidebar ayarlar.
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// alt bilgi sidebar ayarlar.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// bir alt öğe için sidebar, alt olmalıdır implement `Daraltilabilir` ekler.
    pub fn child(mut self, child: E) -> Self {
        self.content.push(child);
        self
    }

    /// çoklu alt öğeler için sidebar, alt öğeler olmalıdır implement `Daraltilabilir` ekler.
    pub fn children(mut self, children: impl IntoIterator<Item = E>) -> Self {
        self.content.extend(children);
        self
    }
}

/// Gecis düğme için collapse/expand [`YanCubuk`].
#[derive(IntoElement)]
pub struct YanCubukGecisDugmesi {
    btn: Dugme,
    collapsed: bool,
    side: Side,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl YanCubukGecisDugmesi {
    /// Yeni bir YanCubukGecisDugmesi oluşturur.
    pub fn new() -> Self {
        Self {
            btn: Dugme::new("collapse").ghost().small(),
            collapsed: false,
            side: Side::Left,
            on_click: None,
        }
    }

    /// taraf açıp kapatma düğme ayarlar.
    ///
    /// varsayılandır `Side::Left`.
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// daraltılmış durum açıp kapatma düğme ayarlar.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Bir tıklama işleyici için açıp kapatma düğme ekler.
    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }
}

impl RenderOnce for YanCubukGecisDugmesi {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let collapsed = self.collapsed;
        let on_click = self.on_click.clone();

        let icon = if collapsed {
            if self.side.is_left() {
                SimgeAdi::PanelLeftOpen
            } else {
                SimgeAdi::PanelRightOpen
            }
        } else {
            if self.side.is_left() {
                SimgeAdi::PanelLeftClose
            } else {
                SimgeAdi::PanelRightClose
            }
        };

        self.btn
            .when_some(on_click, |this, on_click| {
                this.on_click(move |ev, window, cx| {
                    on_click(ev, window, cx);
                })
            })
            .icon(Simge::new(icon).size_4())
    }
}

impl<E: YanCubukOgesi> Styled for YanCubuk<E> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl<E: YanCubukOgesi> RenderOnce for YanCubuk<E> {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.style.padding = EdgesRefinement::default();

        let id = self.id;
        let content_len = self.content.len();
        let overdraw = px(window.viewport_size().height.as_f32() * 0.3);
        let list_state = window
            .use_keyed_state(
                SharedString::from(format!("{}-list-state", id)),
                cx,
                |_, _| ListeDurumu::new(content_len, ListAlignment::Top, overdraw),
            )
            .read(cx)
            .clone();
        if list_state.item_count() != content_len {
            list_state.reset(content_len);
        }

        // Özel width piksel değilse animasyon yerine özgün yerleşim korunur.
        let expanded_width = sidebar_expanded_width(&self.style);
        let layout =
            SidebarLayout::new(self.collapsible, self.collapsed, expanded_width, self.side);

        // YanCubuk content renders at its target width immediately.
        // A wrapper div animates clip-width for smooth transitions
        // without re-laying out sidebar content each animation frame.
        let sidebar = v_flex()
            .id(id.clone())
            .flex_shrink_0()
            .pt(self.ust_bosluk)
            .pb(self.alt_bosluk)
            .h_full()
            .overflow_hidden()
            .relative()
            .bg(cx.theme().sidebar)
            .text_color(cx.theme().sidebar_foreground)
            .border_color(cx.theme().sidebar_border)
            .map(|this| match self.side {
                Side::Left => this.border_r_1(),
                Side::Right => this.border_l_1(),
            })
            .when(self.style.size.width.is_none(), |this| {
                this.w(DEFAULT_WIDTH)
            })
            .refine_style(&self.style)
            .when(layout.icon_collapsed, |this| {
                this.w(COLLAPSED_WIDTH).gap_2()
            })
            .when_some(self.header.take(), |this, header| {
                this.child(
                    h_flex()
                        .id("header")
                        .pt_3()
                        .px_3()
                        .gap_2()
                        .when(layout.icon_collapsed, |this| this.pt_2().px_2())
                        .child(header),
                )
            })
            .child(
                v_flex().id("content").flex_1().min_h_0().child(
                    v_flex()
                        .id("inner")
                        .size_full()
                        .px_3()
                        .gap_y_3()
                        .when(layout.icon_collapsed, |this| this.p_2())
                        .child(
                            list(list_state.clone(), {
                                move |ix, window, cx| {
                                    let group = self.content.get(ix).cloned();
                                    let is_first = ix == 0;
                                    let is_last =
                                        content_len > 0 && ix == content_len.saturating_sub(1);
                                    div()
                                        .id(ix)
                                        .when_some(group, |this, group| {
                                            this.child(
                                                group
                                                    .collapsed(layout.icon_collapsed)
                                                    .render(ix, window, cx)
                                                    .into_any_element(),
                                            )
                                        })
                                        .when(is_first, |this| this.pt_3())
                                        .when(is_last, |this| this.pb_3())
                                        .into_any_element()
                                }
                            })
                            .size_full(),
                        )
                        .vertical_scrollbar(&list_state),
                ),
            )
            .when_some(self.footer.take(), |this, footer| {
                this.child(
                    h_flex()
                        .id("footer")
                        .pb_3()
                        .px_3()
                        .gap_2()
                        .when(layout.icon_collapsed, |this| this.pt_2().px_2())
                        .child(footer),
                )
            });

        let target_width = match layout.wrapper {
            SidebarWrapperLayout::None => return sidebar.into_any_element(),
            SidebarWrapperLayout::Static { width } => {
                return sidebar_wrapper(format!("{}-anim", id), layout.align_child_to_end)
                    .w(width)
                    .when(!layout.offcanvas_collapsed, |this| this.child(sidebar))
                    .into_any_element();
            }
            SidebarWrapperLayout::Animated { target_width } => target_width,
        };

        // Hedef width artık collapse modundan ve özel width'ten türediği için
        // sadece collapsed değişimini izlemek yeterli değil. Offcanvas kapanırken
        // içerik animasyon sonuna kadar mount kalır, sonra tab sırasından çıkar.
        let animation_state = window.use_keyed_state(format!("{}-anim-w", id), cx, |_, _| {
            SidebarAnimationState::new(target_width, !layout.offcanvas_collapsed)
        });

        let hide_request = if animation_state
            .read(cx)
            .needs_update(target_width, layout.offcanvas_collapsed)
        {
            animation_state.update(cx, |state, _| {
                state.update_target(target_width, layout.offcanvas_collapsed)
            })
        } else {
            None
        };
        if let Some(hide_request) = hide_request {
            cx.spawn({
                let animation_state = animation_state.clone();
                async move |cx| {
                    cx.background_executor()
                        .timer(SIDEBAR_TRANSITION_DURATION)
                        .await;
                    _ = animation_state.update(cx, |state, cx| {
                        if state.finish_hide(hide_request) {
                            cx.notify();
                        }
                    });
                }
            })
            .detach();
        }
        let animation_state = *animation_state.read(cx);
        let from_w = animation_state.from_width;
        let to_w = animation_state.target_width;

        let wrapper = sidebar_wrapper(format!("{}-anim", id), layout.align_child_to_end)
            .when(animation_state.render_child, |this| this.child(sidebar));

        Transition::new(SIDEBAR_TRANSITION_DURATION)
            .ease(ease_in_out_cubic)
            .width(from_w, to_w)
            .apply(wrapper, sidebar_animation_id(&id, from_w, to_w))
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn layout(
        collapsible: YanCubukDaralma,
        collapsed: bool,
        expanded_width: Option<Pixels>,
        side: Side,
    ) -> SidebarLayout {
        SidebarLayout::new(collapsible, collapsed, expanded_width, side)
    }

    #[test]
    fn icon_modu_daralinca_simge_genisligi_kullanir() {
        let layout = layout(YanCubukDaralma::Icon, true, Some(px(240.)), Side::Left);

        assert!(layout.icon_collapsed);
        assert!(!layout.offcanvas_collapsed);
        assert!(!layout.align_child_to_end);
        assert_eq!(
            layout.wrapper,
            SidebarWrapperLayout::Animated {
                target_width: COLLAPSED_WIDTH,
            }
        );
    }

    #[test]
    fn icon_modu_acikken_genisleyen_genisligi_kullanir() {
        let layout = layout(YanCubukDaralma::Icon, false, Some(px(240.)), Side::Left);

        assert!(!layout.icon_collapsed);
        assert!(!layout.offcanvas_collapsed);
        assert_eq!(
            layout.wrapper,
            SidebarWrapperLayout::Animated {
                target_width: px(240.),
            }
        );
    }

    #[test]
    fn icon_modu_acikken_piksel_olmayan_genislikte_ozgun_yerlesimi_korur() {
        let layout = layout(YanCubukDaralma::Icon, false, None, Side::Left);

        assert!(!layout.icon_collapsed);
        assert!(!layout.offcanvas_collapsed);
        assert_eq!(layout.wrapper, SidebarWrapperLayout::None);
    }

    #[test]
    fn offcanvas_daralinca_genisligi_sifira_animasyonlar() {
        let layout = layout(YanCubukDaralma::Offcanvas, true, Some(px(240.)), Side::Left);

        assert!(!layout.icon_collapsed);
        assert!(layout.offcanvas_collapsed);
        assert!(layout.align_child_to_end);
        assert_eq!(
            layout.wrapper,
            SidebarWrapperLayout::Animated {
                target_width: px(0.),
            }
        );
    }

    #[test]
    fn offcanvas_acikken_piksel_genisligi_kullanir() {
        let layout = layout(
            YanCubukDaralma::Offcanvas,
            false,
            Some(px(240.)),
            Side::Left,
        );

        assert!(!layout.icon_collapsed);
        assert!(!layout.offcanvas_collapsed);
        assert_eq!(
            layout.wrapper,
            SidebarWrapperLayout::Animated {
                target_width: px(240.),
            }
        );
    }

    #[test]
    fn offcanvas_daralinca_piksel_olmayan_genislik_durumunda_yerlesimi_statik_birakir() {
        let layout = layout(YanCubukDaralma::Offcanvas, true, None, Side::Left);

        assert!(!layout.icon_collapsed);
        assert!(layout.offcanvas_collapsed);
        assert_eq!(
            layout.wrapper,
            SidebarWrapperLayout::Static { width: px(0.) }
        );
    }

    #[test]
    fn offcanvas_acikken_piksel_olmayan_genislikte_ozgun_yerlesimi_korur() {
        let layout = layout(YanCubukDaralma::Offcanvas, false, None, Side::Left);

        assert!(!layout.icon_collapsed);
        assert!(!layout.offcanvas_collapsed);
        assert_eq!(layout.wrapper, SidebarWrapperLayout::None);
    }

    #[test]
    fn offcanvas_alt_ogeyi_icerik_kenarina_yaslamali() {
        let left = layout(YanCubukDaralma::Offcanvas, true, Some(px(240.)), Side::Left);
        let right = layout(
            YanCubukDaralma::Offcanvas,
            true,
            Some(px(240.)),
            Side::Right,
        );

        assert!(left.align_child_to_end);
        assert!(!right.align_child_to_end);
    }

    #[test]
    fn none_modu_daralma_durumunu_yok_sayar() {
        let layout = layout(YanCubukDaralma::None, true, Some(px(240.)), Side::Right);

        assert!(!layout.icon_collapsed);
        assert!(!layout.offcanvas_collapsed);
        assert!(layout.align_child_to_end);
        assert_eq!(layout.wrapper, SidebarWrapperLayout::None);
    }

    #[test]
    fn animasyon_kimligi_yan_cubuk_kimligine_baglanmali() {
        let from = px(240.);
        let to = COLLAPSED_WIDTH;

        assert_ne!(
            sidebar_animation_id(&ElementId::Name("sidebar-a".into()), from, to),
            sidebar_animation_id(&ElementId::Name("sidebar-b".into()), from, to)
        );
    }

    #[test]
    fn animasyon_durumu_offcanvas_kapanisi_bitene_kadar_icerigi_tutar() {
        let mut state = SidebarAnimationState::new(px(240.), true);

        let request = state.update_target(px(0.), true);

        assert_eq!(request, Some(1));
        assert_eq!(state.from_width, px(240.));
        assert_eq!(state.target_width, px(0.));
        assert!(state.render_child);

        assert!(state.finish_hide(1));

        assert!(!state.render_child);
        assert!(!state.hide_scheduled);
    }

    #[test]
    fn animasyon_durumu_bekleyen_offcanvas_gizlemesini_yeniden_planlamamali() {
        let mut state = SidebarAnimationState::new(px(240.), true);

        let request = state.update_target(px(0.), true);

        assert_eq!(request, Some(1));
        assert!(!state.needs_update(px(0.), true));
        assert_eq!(state.update_target(px(0.), true), None);
        assert_eq!(state.hide_request, 1);
    }

    #[test]
    fn animasyon_durumu_yeniden_acilinca_bekleyen_gizlemeyi_iptal_etmeli() {
        let mut state = SidebarAnimationState::new(px(240.), true);

        let request = state.update_target(px(0.), true).unwrap();
        state.update_target(px(240.), false);

        assert!(!state.finish_hide(request));
        assert!(state.render_child);
        assert!(!state.hide_scheduled);
        assert_eq!(state.from_width, px(0.));
        assert_eq!(state.target_width, px(240.));
    }

    #[test]
    fn animasyon_durumu_eski_gizleme_isteklerini_yok_saymali() {
        let mut state = SidebarAnimationState::new(px(240.), true);

        let request = state.update_target(px(0.), true).unwrap();
        state.update_target(px(240.), false);
        state.update_target(px(0.), true);

        assert!(!state.finish_hide(request));
        assert!(state.render_child);
        assert!(state.hide_scheduled);
    }

    #[test]
    fn animasyon_durumu_baslangicta_offcanvas_kapaliysa_gizli_baslamali() {
        let state = SidebarAnimationState::new(px(0.), false);

        assert!(!state.render_child);
        assert_eq!(state.from_width, px(0.));
        assert_eq!(state.target_width, px(0.));
    }
}
