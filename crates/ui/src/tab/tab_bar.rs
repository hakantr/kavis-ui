use std::{cell::RefCell, rc::Rc, time::Duration};

use gpui::{
    Anchor, Animation, AnimationExt as _, AnyElement, App, Bounds, Div, Edges, ElementId,
    InteractiveElement, IntoElement, ParentElement, Pixels, RenderOnce, ScrollHandle, SharedString,
    Stateful, StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};
use rust_i18n::t;
use smallvec::SmallVec;

use super::{Sekme, SekmeVaryanti};
use crate::animation::{Lerp, ease_in_out_cubic};
use crate::button::{Dugme, DugmeVaryantlari as _};
use crate::menu::{DropdownMenu as _, PopupMenuItem};
use crate::{
    BilesenBoyutu, Boyutlandirilabilir, ElementExt, EtkinTema, Secilebilir, Simge, SimgeAdi,
    StilUzantisi, h_flex,
};

struct TabIndicatorBounds {
    container: Bounds<Pixels>,
    tabs: Vec<Bounds<Pixels>>,
}

impl TabIndicatorBounds {
    fn new(num_tabs: usize) -> Self {
        Self {
            container: Bounds::default(),
            tabs: vec![Bounds::default(); num_tabs],
        }
    }

    fn resize(&mut self, num_tabs: usize) {
        self.tabs.resize(num_tabs, Bounds::default());
    }
}

/// Bir SekmeCubugu öğe olan içerir çoklu [`Sekme`] öğeler.
#[derive(IntoElement)]
pub struct SekmeCubugu {
    id: ElementId,
    base: Stateful<Div>,
    style: StyleRefinement,
    scroll_handle: Option<ScrollHandle>,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    children: SmallVec<[Sekme; 2]>,
    last_empty_space: AnyElement,
    selected_index: Option<usize>,
    variant: SekmeVaryanti,
    size: BilesenBoyutu,
    menu: bool,
    on_click: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl SekmeCubugu {
    /// Yeni bir SekmeCubugu oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        let id = id.into();
        Self {
            id: id.clone(),
            base: div().id(id).px(px(-1.)),
            style: StyleRefinement::default(),
            children: SmallVec::new(),
            scroll_handle: None,
            prefix: None,
            suffix: None,
            variant: SekmeVaryanti::default(),
            size: BilesenBoyutu::default(),
            last_empty_space: div().w_3().into_any_element(),
            selected_index: None,
            on_click: None,
            menu: false,
        }
    }

    /// Sekme varyantını ayarlar; tüm alt öğeler bu varyantı devralır.
    pub fn with_variant(mut self, variant: SekmeVaryanti) -> Self {
        self.variant = variant;
        self
    }

    /// Sekme varyantını Pill olarak ayarlar; tüm alt öğeler bu varyantı devralır.
    pub fn pill(mut self) -> Self {
        self.variant = SekmeVaryanti::Pill;
        self
    }

    /// Sekme varyantını çerçeve olarak ayarlar; tüm alt öğeler bu varyantı devralır.
    pub fn outline(mut self) -> Self {
        self.variant = SekmeVaryanti::Outline;
        self
    }

    /// Sekme varyantını Segmented olarak ayarlar; tüm alt öğeler bu varyantı devralır.
    pub fn segmented(mut self) -> Self {
        self.variant = SekmeVaryanti::Segmented;
        self
    }

    /// Sekme varyantını Underline olarak ayarlar; tüm alt öğeler bu varyantı devralır.
    pub fn underline(mut self) -> Self {
        self.variant = SekmeVaryanti::Underline;
        self
    }

    /// Sekmeler taştığında menü düğmesinin gösterilip gösterilmeyeceğini ayarlar. Varsayılan false.
    pub fn menu(mut self, menu: bool) -> Self {
        self.menu = menu;
        self
    }

    /// Track kaydırma SekmeCubugu.
    pub fn track_scroll(mut self, scroll_handle: &ScrollHandle) -> Self {
        self.scroll_handle = Some(scroll_handle.clone());
        self
    }

    /// ön ek öğe SekmeCubugu ayarlar.
    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.prefix = Some(prefix.into_any_element());
        self
    }

    /// son ek öğe SekmeCubugu ayarlar.
    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_any_element());
        self
    }

    /// SekmeCubugu alt öğeleri ekler; tüm alt öğeler varyantı devralır.
    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<Sekme>>) -> Self {
        self.children.extend(children.into_iter().map(Into::into));
        self
    }

    /// SekmeCubugu alt öğesi ekler; sekme varyantı devralır.
    pub fn child(mut self, child: impl Into<Sekme>) -> Self {
        self.children.push(child.into());
        self
    }

    /// seçili indeks SekmeCubugu ayarlar.
    pub fn selected_index(mut self, index: usize) -> Self {
        self.selected_index = Some(index);
        self
    }

    /// son boş boşluk öğe SekmeCubugu ayarlar.
    pub fn last_empty_space(mut self, last_empty_space: impl IntoElement) -> Self {
        self.last_empty_space = last_empty_space.into_any_element();
        self
    }

    /// SekmeCubugu on_click geri çağrısını ayarlar; ilk parametre tıklanan sekmenin indeksidir.
    ///
    /// Bu ayar yapıldığında alt öğelerin on_click işleyicileri yok sayılır.
    pub fn on_click<F>(mut self, on_click: F) -> Self
    where
        F: Fn(&usize, &mut Window, &mut App) + 'static,
    {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    /// sliding gösterge öğe için animasyonlu sekme switching. çizer.
    fn render_indicator(
        &self,
        bounds_rc: &Option<Rc<RefCell<TabIndicatorBounds>>>,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<AnyElement> {
        let has_indicator = matches!(
            self.variant,
            SekmeVaryanti::Segmented | SekmeVaryanti::Pill | SekmeVaryanti::Underline
        );
        let num_tabs = self.children.len();
        let selected_ix = self.selected_index.unwrap_or(usize::MAX);

        if !(has_indicator && num_tabs > 0 && selected_ix < num_tabs) {
            return None;
        }

        let prev_key = format!("{}-tab-prev", self.id);
        let anim_key = format!("{}-tab-anim", self.id);
        let init_key = format!("{}-tab-init", self.id);

        let prev_selected = window.use_keyed_state(prev_key, cx, |_, _| selected_ix);
        // (from_left, from_width, to_left, to_width, epoch)
        let anim_params =
            window.use_keyed_state(anim_key, cx, |_, _| (px(0.), px(0.), px(0.), px(0.), 0u64));
        let initialized = window.use_keyed_state(init_key, cx, |_, _| false);

        // First frame: trigger re-render to capture bounds via on_prepaint
        if !*initialized.read(cx) {
            initialized.update(cx, |v, _| *v = true);
        }

        self.update_anim_params(selected_ix, bounds_rc, &prev_selected, &anim_params, cx);

        let (from_left, from_width, to_left, to_width, epoch) = *anim_params.read(cx);
        if to_width <= px(0.) {
            return None;
        }

        let variant = self.variant;
        let size = self.size;
        let inner_height = variant.inner_height(size);
        let inner_radius = variant.inner_radius(size, cx);

        let indicator = div()
            .absolute()
            .top_0()
            .bottom_0()
            .map(|el| match variant {
                SekmeVaryanti::Segmented => el.flex().items_center().child(
                    div()
                        .w_full()
                        .h(inner_height)
                        .bg(cx.theme().background)
                        .rounded(inner_radius)
                        .shadow_xs(),
                ),
                SekmeVaryanti::Pill => el
                    .flex()
                    .items_center()
                    .child(div().size_full().bg(cx.theme().primary).rounded(px(99.))),
                SekmeVaryanti::Underline => el.child(
                    div()
                        .absolute()
                        .left_0()
                        .right_0()
                        .bottom_0()
                        .h(px(2.))
                        .bg(cx.theme().primary),
                ),
                _ => el,
            })
            .with_animation(
                ElementId::NamedInteger("tab-ind".into(), epoch),
                Animation::new(Duration::from_millis(200)).with_easing(ease_in_out_cubic),
                move |el, delta| {
                    let left = Lerp::lerp(&from_left, &to_left, delta);
                    let width = Lerp::lerp(&from_width, &to_width, delta);
                    el.left(left).w(width)
                },
            );

        Some(indicator.into_any_element())
    }

    /// animasyon parametreler temelli üzerinde geçerli ve önceki seçim. günceller.
    fn update_anim_params(
        &self,
        selected_ix: usize,
        bounds_rc: &Option<Rc<RefCell<TabIndicatorBounds>>>,
        prev_selected: &gpui::Entity<usize>,
        anim_params: &gpui::Entity<(Pixels, Pixels, Pixels, Pixels, u64)>,
        cx: &mut App,
    ) {
        let rc = match bounds_rc {
            Some(rc) => rc,
            None => return,
        };

        let prev_ix = *prev_selected.read(cx);
        let bounds = rc.borrow();
        let container = bounds.container;

        if container.size.width == px(0.) {
            if prev_ix != selected_ix {
                prev_selected.update(cx, |v, _| *v = selected_ix);
            }
            return;
        }

        if prev_ix != selected_ix {
            let from_b = bounds.tabs.get(prev_ix);
            let to_b = bounds.tabs.get(selected_ix);
            match (from_b, to_b) {
                (Some(from_b), Some(to_b)) => {
                    let from_left = from_b.origin.x - container.origin.x;
                    let from_width = from_b.size.width;
                    let to_left = to_b.origin.x - container.origin.x;
                    let to_width = to_b.size.width;
                    let epoch = anim_params.read(cx).4 + 1;
                    anim_params.update(cx, |v, _| {
                        *v = (from_left, from_width, to_left, to_width, epoch)
                    });
                }
                (None, Some(to_b)) => {
                    let left = to_b.origin.x - container.origin.x;
                    let width = to_b.size.width;
                    anim_params.update(cx, |v, _| *v = (left, width, left, width, v.4));
                }
                _ => {}
            }
            drop(bounds);
            prev_selected.update(cx, |v, _| *v = selected_ix);
            return;
        }

        // Same selection, no bounds yet: initialize position
        if anim_params.read(cx).3 != px(0.) {
            return;
        }

        if let Some(to_b) = bounds.tabs.get(selected_ix) {
            let left = to_b.origin.x - container.origin.x;
            let width = to_b.size.width;
            anim_params.update(cx, |v, _| *v = (left, width, left, width, v.4));
        }
    }
}

impl Styled for SekmeCubugu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Boyutlandirilabilir for SekmeCubugu {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for SekmeCubugu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let default_gap = match self.size {
            BilesenBoyutu::Kucuk | BilesenBoyutu::CokKucuk => px(8.),
            BilesenBoyutu::Buyuk => px(16.),
            _ => px(12.),
        };
        let (bg, paddings, gap) = match self.variant {
            SekmeVaryanti::Sekme => {
                let padding = Edges::all(px(0.));
                (cx.theme().tab_bar, padding, px(0.))
            }
            SekmeVaryanti::Outline => {
                let padding = Edges::all(px(0.));
                (cx.theme().transparent, padding, default_gap)
            }
            SekmeVaryanti::Pill => {
                let padding = Edges::all(px(0.));
                (cx.theme().transparent, padding, px(4.))
            }
            SekmeVaryanti::Segmented => {
                let padding_x = match self.size {
                    BilesenBoyutu::CokKucuk => px(2.),
                    BilesenBoyutu::Kucuk => px(3.),
                    _ => px(4.),
                };
                let padding = Edges {
                    left: padding_x,
                    right: padding_x,
                    ..Default::default()
                };

                (cx.theme().tab_bar_segmented, padding, px(2.))
            }
            SekmeVaryanti::Underline => {
                // This gap is same as the tab inner_paddings
                let gap = match self.size {
                    BilesenBoyutu::CokKucuk => px(10.),
                    BilesenBoyutu::Kucuk => px(12.),
                    BilesenBoyutu::Buyuk => px(20.),
                    _ => px(16.),
                };

                (cx.theme().transparent, Edges::all(px(0.)), gap)
            }
        };

        let has_indicator = matches!(
            self.variant,
            SekmeVaryanti::Segmented | SekmeVaryanti::Pill | SekmeVaryanti::Underline
        );
        let num_tabs = self.children.len();

        // Bounds tracking for tab indicator animation.
        // Uses Rc<RefCell> to avoid triggering re-renders from prepaint writes.
        let bounds_rc = if has_indicator && num_tabs > 0 {
            let rc: Rc<RefCell<TabIndicatorBounds>> = window
                .use_keyed_state(format!("{}-tab-bounds", self.id), cx, |_, _| {
                    Rc::new(RefCell::new(TabIndicatorBounds::new(num_tabs)))
                })
                .read(cx)
                .clone();
            rc.borrow_mut().resize(num_tabs);
            Some(rc)
        } else {
            None
        };

        let indicator_element = self.render_indicator(&bounds_rc, window, cx);

        let has_suffix_or_menu = self.suffix.is_some() || self.menu;
        let mut item_metas: Vec<(Option<SharedString>, Option<Simge>, bool)> = Vec::new();
        let selected_index = self.selected_index;
        let on_click = self.on_click.clone();

        self.base
            .group("tab-bar")
            .relative()
            .flex()
            .items_center()
            .bg(bg)
            .text_color(cx.theme().tab_foreground)
            .when(
                self.variant == SekmeVaryanti::Underline || self.variant == SekmeVaryanti::Sekme,
                |this| {
                    this.child(
                        div()
                            .id("border-b")
                            .absolute()
                            .left_0()
                            .bottom_0()
                            .size_full()
                            .border_b_1()
                            .border_color(cx.theme().border),
                    )
                },
            )
            .rounded(self.variant.tab_bar_radius(self.size, cx))
            .paddings(paddings)
            .refine_style(&self.style)
            .when_some(self.prefix, |this, prefix| this.child(prefix))
            .child(
                h_flex().id("tabs").flex_1().overflow_x_hidden().child(
                    h_flex()
                        .id("tabs-inner")
                        .relative()
                        .gap(gap)
                        .overflow_x_scroll()
                        .when_some(self.scroll_handle, |this, scroll_handle| {
                            this.track_scroll(&scroll_handle)
                        })
                        .when_some(bounds_rc.clone(), |this, rc| {
                            this.on_prepaint(move |bounds, _, _| {
                                rc.borrow_mut().container = bounds;
                            })
                        })
                        .when_some(indicator_element, |this, ind| this.child(ind))
                        .children(self.children.into_iter().enumerate().map(|(ix, child)| {
                            item_metas.push((
                                child.label.clone(),
                                child.icon.clone(),
                                child.disabled,
                            ));
                            let tab_bar_prefix = child.tab_bar_prefix.unwrap_or(true);
                            let mut tab = child
                                .ix(ix)
                                .tab_bar_prefix(tab_bar_prefix)
                                .with_variant(self.variant)
                                .with_size(self.size);
                            tab.indicator_active = has_indicator;
                            let tab = tab
                                .when_some(self.selected_index, |this, selected_ix| {
                                    this.selected(selected_ix == ix)
                                })
                                .when_some(self.on_click.clone(), move |this, on_click| {
                                    this.on_click(move |_, window, cx| on_click(&ix, window, cx))
                                });

                            if let Some(ref rc) = bounds_rc {
                                let rc = rc.clone();
                                div()
                                    .on_prepaint(move |bounds, _, _| {
                                        if let Some(slot) = rc.borrow_mut().tabs.get_mut(ix) {
                                            *slot = bounds;
                                        }
                                    })
                                    .child(tab)
                                    .into_any_element()
                            } else {
                                tab.into_any_element()
                            }
                        }))
                        .when(has_suffix_or_menu, |this| this.child(self.last_empty_space)),
                ),
            )
            .when(self.menu, |this| {
                this.child(
                    Dugme::new("more")
                        .xsmall()
                        .ghost()
                        .icon(SimgeAdi::ChevronDown)
                        .dropdown_menu(move |mut this, _, _| {
                            this = this.scrollable(true);
                            for (ix, (label, icon, disabled)) in item_metas.iter().enumerate() {
                                let base = if let Some(label) = label.clone() {
                                    PopupMenuItem::new(label)
                                } else if let Some(icon) = icon.clone() {
                                    PopupMenuItem::element(move |_, _| icon.clone())
                                } else {
                                    PopupMenuItem::new(t!("Yerlesim.Unnamed"))
                                };
                                this = this.item(
                                    base.checked(selected_index == Some(ix))
                                        .disabled(*disabled)
                                        .when_some(on_click.clone(), |this, on_click| {
                                            this.on_click(move |_, window, cx| {
                                                on_click(&ix, window, cx)
                                            })
                                        }),
                                );
                            }

                            this
                        })
                        .anchor(Anchor::TopRight),
                )
            })
            .when_some(self.suffix, |this, suffix| this.child(suffix))
    }
}
