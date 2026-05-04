use std::rc::Rc;

use gpui::{
    Anchor, Context, DismissEvent, ElementId, Entity, Focusable, InteractiveElement, IntoElement,
    RenderOnce, SharedString, StyleRefinement, Styled, Window,
};

use crate::{Secilebilir, button::Dugme, menu::AcilirMenu, popover::AcilirKatman};

/// Bir açılır menü özellik için düğmeler ve diğer etkileşimli öğeler
pub trait AcilirMenuTetikleyici:
    Styled + Secilebilir + InteractiveElement + IntoElement + 'static
{
    /// Bir açılır menü ile verilen öğeler, anchored için TopLeft köşe oluşturur.
    fn acilir_menu(
        self,
        f: impl Fn(AcilirMenu, &mut Window, &mut Context<AcilirMenu>) -> AcilirMenu + 'static,
    ) -> AcilirMenuKatmani<Self> {
        self.acilir_menu_capa_ile(Anchor::TopLeft, f)
    }

    /// Bir açılır menü ile verilen öğeler, anchored için verilen köşe oluşturur.
    fn acilir_menu_capa_ile(
        mut self,
        anchor: impl Into<Anchor>,
        f: impl Fn(AcilirMenu, &mut Window, &mut Context<AcilirMenu>) -> AcilirMenu + 'static,
    ) -> AcilirMenuKatmani<Self> {
        let style = self.style().clone();
        let id = self.interactivity().element_id.clone();

        AcilirMenuKatmani::new(id.unwrap_or(0.into()), anchor, self, f).trigger_style(style)
    }
}

impl AcilirMenuTetikleyici for Dugme {}

#[derive(IntoElement)]
pub struct AcilirMenuKatmani<T: Secilebilir + IntoElement + 'static> {
    id: ElementId,
    style: StyleRefinement,
    anchor: Anchor,
    trigger: T,
    builder: Rc<dyn Fn(AcilirMenu, &mut Window, &mut Context<AcilirMenu>) -> AcilirMenu>,
}

impl<T> AcilirMenuKatmani<T>
where
    T: Secilebilir + IntoElement + 'static,
{
    fn new(
        id: ElementId,
        anchor: impl Into<Anchor>,
        trigger: T,
        builder: impl Fn(AcilirMenu, &mut Window, &mut Context<AcilirMenu>) -> AcilirMenu + 'static,
    ) -> Self {
        Self {
            id: SharedString::from(format!("dropdown-menu:{:?}", id)).into(),
            style: StyleRefinement::default(),
            anchor: anchor.into(),
            trigger,
            builder: Rc::new(builder),
        }
    }

    /// sabitleyici köşe için açılır menü açılır katman ayarlar.
    pub fn anchor(mut self, anchor: impl Into<Anchor>) -> Self {
        self.anchor = anchor.into();
        self
    }

    /// stil refinement için açılır menü tetikleyici ayarlar.
    fn trigger_style(mut self, style: StyleRefinement) -> Self {
        self.style = style;
        self
    }
}

#[derive(Default)]
struct AcilirMenuDurumu {
    menu: Option<Entity<AcilirMenu>>,
}

impl<T> RenderOnce for AcilirMenuKatmani<T>
where
    T: Secilebilir + IntoElement + 'static,
{
    fn render(self, window: &mut Window, cx: &mut gpui::App) -> impl IntoElement {
        let builder = self.builder.clone();
        let menu_state =
            window.use_keyed_state(self.id.clone(), cx, |_, _| AcilirMenuDurumu::default());

        AcilirKatman::new(SharedString::from(format!("popover:{}", self.id)))
            .appearance(false)
            .overlay_closable(false)
            .trigger(self.trigger)
            .trigger_style(self.style)
            .anchor(self.anchor)
            .content(move |_, window, cx| {
                // Here is special logic to only create the AcilirMenu once and reuse it.
                // Because this `content` will called in every time render, so we need to store the menu
                // in state to avoid recreating at every render.
                //
                // And we also need to rebuild the menu when it is dismissed, to rebuild menu items
                // dynamically for support `acilir_menu` method, so we listen for DismissEvent below.
                let menu = match menu_state.read(cx).menu.clone() {
                    Some(menu) => menu,
                    None => {
                        let builder = builder.clone();
                        let menu = AcilirMenu::build(window, cx, move |menu, window, cx| {
                            builder(menu, window, cx)
                        });
                        menu_state.update(cx, |state, _| {
                            state.menu = Some(menu.clone());
                        });
                        menu.focus_handle(cx).focus(window, cx);

                        // Listen for dismiss events from the AcilirMenu to close the popover.
                        let popover_state = cx.entity();
                        window
                            .subscribe(&menu, cx, {
                                let menu_state = menu_state.clone();
                                move |_, _: &DismissEvent, window, cx| {
                                    popover_state.update(cx, |state, cx| {
                                        state.dismiss(window, cx);
                                    });
                                    menu_state.update(cx, |state, _| {
                                        state.menu = None;
                                    });
                                }
                            })
                            .detach();

                        menu.clone()
                    }
                };

                menu.clone()
            })
    }
}
