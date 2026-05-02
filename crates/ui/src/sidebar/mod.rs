use crate::{
    Daraltilabilir, EtkinTema, Side, Simge, SimgeAdi, Sizable, StyledExt,
    button::{Dugme, DugmeVaryantlari},
    h_flex,
    scroll::KaydirilabilirOge,
    v_flex,
};
use gpui::{
    AbsoluteLength, AnyElement, App, ClickEvent, DefiniteLength, EdgesRefinement, ElementId,
    InteractiveElement as _, IntoElement, Length, ListAlignment, ListState as ListeDurumu,
    ParentElement, Pixels, RenderOnce, SharedString, StyleRefinement, Styled, Window, div, list,
    prelude::FluentBuilder, px,
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
    collapsible: bool,
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
            collapsible: true,
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

    /// Yan çubuğun daraltılabilir olup olmadığını ayarlar. Varsayılan: true.
    pub fn collapsible(mut self, collapsible: bool) -> Self {
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

        let collapsed = self.collapsed;

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
            .when(self.collapsed, |this| this.w(COLLAPSED_WIDTH).gap_2())
            .when_some(self.header.take(), |this, header| {
                this.child(
                    h_flex()
                        .id("header")
                        .pt_3()
                        .px_3()
                        .gap_2()
                        .when(self.collapsed, |this| this.pt_2().px_2())
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
                        .when(self.collapsed, |this| this.p_2())
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
                                                    .collapsed(self.collapsed)
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
                        .when(self.collapsed, |this| this.pt_2().px_2())
                        .child(footer),
                )
            });

        if !self.collapsible {
            return sidebar.into_any_element();
        }

        // Determine effective expanded width from user's custom style or default.
        let expanded_width = match self.style.size.width {
            Some(Length::Definite(DefiniteLength::Absolute(AbsoluteLength::Pixels(px)))) => px,
            Some(_) => {
                return sidebar.into_any_element();
            }
            None => DEFAULT_WIDTH,
        };

        // Store animation widths in keyed state so they remain stable across
        // re-renders (GPUI re-renders the whole tree on each animation frame).
        // Only update when `collapsed` actually changes.
        let prev_collapsed =
            window.use_keyed_state(format!("{}-prev-col", id), cx, |_, _| collapsed);
        let anim_widths = window.use_keyed_state(format!("{}-anim-w", id), cx, |_, _| {
            // First render: from == to, no visible animation
            let w = if collapsed {
                COLLAPSED_WIDTH
            } else {
                expanded_width
            };
            (w, w)
        });

        if *prev_collapsed.read(cx) != collapsed {
            let (new_from, new_to) = if collapsed {
                (expanded_width, COLLAPSED_WIDTH)
            } else {
                (COLLAPSED_WIDTH, expanded_width)
            };
            anim_widths.update(cx, |v, _| *v = (new_from, new_to));
            prev_collapsed.update(cx, |v, _| *v = collapsed);
        }
        let (from_w, to_w) = *anim_widths.read(cx);

        let wrapper = div()
            .id(format!("{}-anim", id))
            .h_full()
            .flex_shrink_0()
            .overflow_hidden()
            .child(sidebar);

        Transition::new(Duration::from_millis(200))
            .ease(ease_in_out_cubic)
            .width(from_w, to_w)
            .apply(
                wrapper,
                ElementId::NamedInteger("sidebar-w".into(), collapsed as u64),
            )
            .into_any_element()
    }
}
