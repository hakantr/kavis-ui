use std::{cell::RefCell, rc::Rc};

use crate::ham_gpui::{
    Anchor, AnyElement, App, Context, DismissEvent, Element, ElementId, Entity, Focusable,
    GlobalElementId, Hitbox, HitboxBehavior, InspectorElementId, InteractiveElement, IntoElement,
    MouseButton, MouseDownEvent, ParentElement, Pixels, Point, StyleRefinement, Styled,
    Subscription, Window, anchored, deferred, div, prelude::FluentBuilder, px,
};

use crate::menu::AcilirMenu;

/// Bir uzantı özellik için adding bir bağlam menü için bir öğe.
pub trait BaglamMenusuUzantisi: InteractiveElement + ParentElement + Styled {
    /// Bir bağlam menü için öğe ekler.
    ///
    /// Bu, öğeyi `relative` konumlandırılmış hale getirir ve altına bir `BaglamMenusu` öğesi ekler.
    /// `BaglamMenusu` öğesi `absolute` konumlandırıldığı için üst öğenin yerleşimini etkilemez.
    fn baglam_menusu(
        mut self,
        f: impl Fn(AcilirMenu, &mut Window, &mut Context<AcilirMenu>) -> AcilirMenu + 'static,
    ) -> BaglamMenusu<Self>
    where
        Self: Sized,
    {
        // Generate a unique ID based on the element's memory address to ensure
        // each context menu has its own state and doesn't share with others
        let id = self
            .interactivity()
            .element_id
            .clone()
            .map(|id| format!("context-menu-{:?}", id))
            .unwrap_or_else(|| format!("context-menu-{:p}", &self as *const _));
        BaglamMenusu::new(id, self).menu(f)
    }
}

impl<E: InteractiveElement + ParentElement + Styled> BaglamMenusuUzantisi for E {}

/// Bir bağlam menü olan olabilir shown üzerinde sağ-tıklama.
pub struct BaglamMenusu<E: ParentElement + Styled + Sized> {
    id: ElementId,
    element: Option<E>,
    menu: Option<Rc<dyn Fn(AcilirMenu, &mut Window, &mut Context<AcilirMenu>) -> AcilirMenu>>,
    // This is not in use, just for style refinement forwarding.
    _ignore_style: StyleRefinement,
    anchor: Anchor,
}

impl<E: ParentElement + Styled> BaglamMenusu<E> {
    /// Yeni bir bağlam menü ile verilen ID oluşturur.
    pub fn new(id: impl Into<ElementId>, element: E) -> Self {
        Self {
            id: id.into(),
            element: Some(element),
            menu: None,
            anchor: Anchor::TopLeft,
            _ignore_style: StyleRefinement::default(),
        }
    }

    /// Oluşturur bağlam menü kullanarak verilen oluşturucu fonksiyon.
    #[must_use]
    fn menu<F>(mut self, builder: F) -> Self
    where
        F: Fn(AcilirMenu, &mut Window, &mut Context<AcilirMenu>) -> AcilirMenu + 'static,
    {
        self.menu = Some(Rc::new(builder));
        self
    }

    fn with_element_state<R>(
        &mut self,
        id: &GlobalElementId,
        window: &mut Window,
        cx: &mut App,
        f: impl FnOnce(&mut Self, &mut BaglamMenusuDurumu, &mut Window, &mut App) -> R,
    ) -> R {
        window.with_optional_element_state::<BaglamMenusuDurumu, _>(
            Some(id),
            |element_state, window| {
                let mut element_state = element_state.unwrap().unwrap_or_default();
                let result = f(self, &mut element_state, window, cx);
                (result, Some(element_state))
            },
        )
    }
}

impl<E: ParentElement + Styled> ParentElement for BaglamMenusu<E> {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        if let Some(element) = &mut self.element {
            element.extend(elements);
        }
    }
}

impl<E: ParentElement + Styled> Styled for BaglamMenusu<E> {
    fn style(&mut self) -> &mut StyleRefinement {
        if let Some(element) = &mut self.element {
            element.style()
        } else {
            &mut self._ignore_style
        }
    }
}

impl<E: ParentElement + Styled + IntoElement + 'static> IntoElement for BaglamMenusu<E> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

struct BaglamMenusuPaylasimliDurum {
    menu_view: Option<Entity<AcilirMenu>>,
    open: bool,
    position: Point<Pixels>,
    _subscription: Option<Subscription>,
}

pub struct BaglamMenusuDurumu {
    element: Option<AnyElement>,
    shared_state: Rc<RefCell<BaglamMenusuPaylasimliDurum>>,
}

impl Default for BaglamMenusuDurumu {
    fn default() -> Self {
        Self {
            element: None,
            shared_state: Rc::new(RefCell::new(BaglamMenusuPaylasimliDurum {
                menu_view: None,
                open: false,
                position: Default::default(),
                _subscription: None,
            })),
        }
    }
}

impl<E: ParentElement + Styled + IntoElement + 'static> Element for BaglamMenusu<E> {
    type RequestLayoutState = BaglamMenusuDurumu;
    type PrepaintState = Hitbox;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&crate::ham_gpui::GlobalElementId>,
        _: Option<&crate::ham_gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (crate::ham_gpui::LayoutId, Self::RequestLayoutState) {
        let anchor = self.anchor;

        self.with_element_state(
            id.unwrap(),
            window,
            cx,
            |this, state: &mut BaglamMenusuDurumu, window, cx| {
                let (position, open) = {
                    let shared_state = state.shared_state.borrow();
                    (shared_state.position, shared_state.open)
                };
                let menu_view = state.shared_state.borrow().menu_view.clone();
                let mut menu_element = None;
                if open {
                    let has_menu_item = menu_view
                        .as_ref()
                        .map(|menu| !menu.read(cx).is_empty())
                        .unwrap_or(false);

                    if has_menu_item {
                        menu_element = Some(
                            deferred(
                                anchored().child(
                                    div()
                                        .w(window.bounds().size.width)
                                        .h(window.bounds().size.height)
                                        .on_scroll_wheel(|_, _, cx| {
                                            cx.stop_propagation();
                                        })
                                        .child(
                                            anchored()
                                                .position(position)
                                                .snap_to_window_with_margin(px(8.))
                                                .anchor(anchor)
                                                .when_some(menu_view, |this, menu| {
                                                    // Focus the menu, so that can be handle the action.
                                                    if !menu
                                                        .focus_handle(cx)
                                                        .contains_focused(window, cx)
                                                    {
                                                        menu.focus_handle(cx).focus(window, cx);
                                                    }

                                                    this.child(menu.clone())
                                                }),
                                        ),
                                ),
                            )
                            .with_priority(1)
                            .into_any(),
                        );
                    }
                }

                let mut element = this
                    .element
                    .take()
                    .expect("Öğe mevcut olmalı.")
                    .children(menu_element)
                    .into_any_element();

                let layout_id = element.request_layout(window, cx);

                (
                    layout_id,
                    BaglamMenusuDurumu {
                        element: Some(element),
                        ..Default::default()
                    },
                )
            },
        )
    }

    fn prepaint(
        &mut self,
        _: Option<&crate::ham_gpui::GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: crate::ham_gpui::Bounds<crate::ham_gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        if let Some(element) = &mut request_layout.element {
            element.prepaint(window, cx);
        }
        window.insert_hitbox(bounds, HitboxBehavior::Normal)
    }

    fn paint(
        &mut self,
        id: Option<&crate::ham_gpui::GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: crate::ham_gpui::Bounds<crate::ham_gpui::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(element) = &mut request_layout.element {
            element.paint(window, cx);
        }

        // Take the builder before setting up element state to avoid borrow issues
        let builder = self.menu.clone();

        self.with_element_state(
            id.unwrap(),
            window,
            cx,
            |_view, state: &mut BaglamMenusuDurumu, window, _| {
                let shared_state = state.shared_state.clone();

                let hitbox = hitbox.clone();
                // When right mouse click, to build content menu, and show it at the mouse position.
                window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
                    if phase.bubble()
                        && event.button == MouseButton::Right
                        && hitbox.is_hovered(window)
                    {
                        {
                            let mut shared_state = shared_state.borrow_mut();
                            // Clear any existing menu view to allow immediate replacement
                            // Set the new position and open the menu
                            shared_state.menu_view = None;
                            shared_state._subscription = None;
                            shared_state.position = event.position;
                            shared_state.open = true;
                        }

                        // Use defer to build the menu in the next frame, avoiding race conditions
                        window.defer(cx, {
                            let shared_state = shared_state.clone();
                            let builder = builder.clone();
                            move |window, cx| {
                                let menu =
                                    AcilirMenu::build(window, cx, move |menu, window, cx| {
                                        let Some(build) = &builder else {
                                            return menu;
                                        };
                                        build(menu, window, cx)
                                    });

                                // Set up the subscription for dismiss handling
                                let _subscription = window.subscribe(&menu, cx, {
                                    let shared_state = shared_state.clone();
                                    move |_, _: &DismissEvent, window, _cx| {
                                        shared_state.borrow_mut().open = false;
                                        window.refresh();
                                    }
                                });

                                // Update the shared state with the built menu and subscription
                                {
                                    let mut state = shared_state.borrow_mut();
                                    state.menu_view = Some(menu.clone());
                                    state._subscription = Some(_subscription);
                                    window.refresh();
                                }
                            }
                        });
                    }
                });
            },
        );
    }
}
