use gpui::{
    Anchor, AnyElement, App, Bounds, Context, Deferred, DismissEvent, Div, ElementId, EventEmitter,
    FocusHandle, Focusable, InteractiveElement as _, IntoElement, KeyBinding, MouseButton,
    ParentElement, Pixels, Point, Render, RenderOnce, Stateful, StyleRefinement, Styled,
    Subscription, Window, anchored, deferred, div, prelude::FluentBuilder as _, px,
};
use std::{cell::Cell, rc::Rc};

use crate::{
    ElementExt, Selectable, StyledExt as _, actions::Cancel, global_state::KureselDurum, v_flex,
};

const CONTEXT: &str = "AcilirKatman";
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("escape", Cancel, Some(CONTEXT))])
}

/// Bir düğme veya başka bir öğe ile tetiklenebilen açılır katman öğesi.
#[derive(IntoElement)]
pub struct AcilirKatman {
    id: ElementId,
    style: StyleRefinement,
    anchor: Anchor,
    default_open: bool,
    open: Option<bool>,
    tracked_focus_handle: Option<FocusHandle>,
    trigger: Option<Box<dyn FnOnce(bool, &Window, &App) -> AnyElement + 'static>>,
    content: Option<
        Rc<
            dyn Fn(
                    &mut AcilirKatmanDurumu,
                    &mut Window,
                    &mut Context<AcilirKatmanDurumu>,
                ) -> AnyElement
                + 'static,
        >,
    >,
    children: Vec<AnyElement>,
    /// stil için tetikleyici öğe.
    /// Bu için kullanılır hotfix tetikleyici öğe stil için destek w_full.
    trigger_style: Option<StyleRefinement>,
    mouse_button: MouseButton,
    appearance: bool,
    overlay_closable: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
}

impl AcilirKatman {
    /// Yeni bir AcilirKatman ile `view` mod oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            anchor: Anchor::TopLeft,
            trigger: None,
            trigger_style: None,
            content: None,
            tracked_focus_handle: None,
            children: vec![],
            mouse_button: MouseButton::Left,
            appearance: true,
            overlay_closable: true,
            default_open: false,
            open: None,
            on_open_change: None,
        }
    }

    /// Açılır katmanın sabitleme köşesini ayarlar. Varsayılan `Anchor::TopLeft` değeridir.
    ///
    /// Bu yöntem `Anchor` tipiyle geriye dönük uyumluluk için korunur.
    /// Internally, onu converts `Anchor` için `Anchor`.
    pub fn anchor(mut self, anchor: impl Into<Anchor>) -> Self {
        self.anchor = anchor.into();
        self
    }

    /// Açılır katman tetikleyicisi için fare düğmesini ayarlar. Varsayılan `MouseButton::Left` değeridir.
    pub fn mouse_button(mut self, mouse_button: MouseButton) -> Self {
        self.mouse_button = mouse_button;
        self
    }

    /// tetikleyici öğe açılır katman ayarlar.
    pub fn trigger<T>(mut self, trigger: T) -> Self
    where
        T: Selectable + IntoElement + 'static,
    {
        self.trigger = Some(Box::new(|is_open, _, _| {
            let selected = trigger.is_selected();
            trigger.selected(selected || is_open).into_any_element()
        }));
        self
    }

    /// Açılır katmanın varsayılan açık durumunu ayarlar. Varsayılan `false` değeridir.
    ///
    /// Bu yalnızca için kullanılır initialize açık durum açılır katman.
    ///
    /// `open` yöntemini kullanırsanız bu değerin yok sayılacağını unutmayın.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Force ayarlar açık durum açılır katman.
    ///
    /// Bu ayarlanırsa açılır katman bu değerle kontrol edilir.
    ///
    /// NOT: Durum değişimlerini işlemek için `on_open_change` ile birlikte kullanılmalıdır.
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    /// Bir geri çağrı olmak için çağrılır olduğunda açık durum değişimler ekler.
    ///
    /// İlk `&bool` parametresi **yeni açık durumudur**.
    ///
    /// Bu kullanışlıdır olduğunda kullanarak `open` yöntem için kontrol açılır katman durum.
    pub fn on_open_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(&bool, &mut Window, &mut App) + 'static,
    {
        self.on_open_change = Some(Rc::new(callback));
        self
    }

    /// stil için tetikleyici öğe ayarlar.
    pub fn trigger_style(mut self, style: StyleRefinement) -> Self {
        self.trigger_style = Some(style);
        self
    }

    /// Dışarı tıklanınca açılır katmanın kapatılıp kapatılmayacağını ayarlar. Varsayılan `true` değeridir.
    pub fn overlay_closable(mut self, closable: bool) -> Self {
        self.overlay_closable = closable;
        self
    }

    /// içerik oluşturucu için içerik AcilirKatman ayarlar.
    ///
    /// Bu geri çağrı açılır katman her çizildiğinde çağrılır.
    /// Bu yüzden içerik kapanışı içinde yeni öğeler veya varlıklar oluşturmaktan kaçının.
    pub fn content<F, E>(mut self, content: F) -> Self
    where
        E: IntoElement,
        F: Fn(&mut AcilirKatmanDurumu, &mut Window, &mut Context<AcilirKatmanDurumu>) -> E
            + 'static,
    {
        self.content = Some(Rc::new(move |state, window, cx| {
            content(state, window, cx).into_any_element()
        }));
        self
    }

    /// Açılır katmanın stilsiz olup olmadığını ayarlar. Varsayılan `false` değeridir.
    ///
    /// Eğer yok stil:
    ///
    /// - açılır katman olmayacak sahip bir bg, kenarlık, gölge, veya dolgu.
    /// - tıklama out açılır katman olmayacak dismiss onu.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }

    /// Açılır katman açıldığında odak alacak odak işleyicisini bağlar.
    /// Bu ayarlanmazsa açılır katman için yeni bir odak işleyici oluşturulur.
    ///
    /// Açılır katman açıldığında odak bu işleyiciye taşınır.
    pub fn track_focus(mut self, handle: &FocusHandle) -> Self {
        self.tracked_focus_handle = Some(handle.clone());
        self
    }

    /// Dikey yön çevirme: `Top*` ile `Bottom*` çifti arasında takas eder.
    /// Yatay (LeftCenter/RightCenter) anchor'lar olduğu gibi döner.
    pub(crate) fn flip_vertical(anchor: Anchor) -> Anchor {
        match anchor {
            Anchor::TopLeft => Anchor::BottomLeft,
            Anchor::TopCenter => Anchor::BottomCenter,
            Anchor::TopRight => Anchor::BottomRight,
            Anchor::BottomLeft => Anchor::TopLeft,
            Anchor::BottomCenter => Anchor::TopCenter,
            Anchor::BottomRight => Anchor::TopRight,
            other => other,
        }
    }

    pub(crate) fn resolved_corner(anchor: Anchor, trigger_bounds: Bounds<Pixels>) -> Point<Pixels> {
        match anchor {
            Anchor::TopLeft => trigger_bounds.origin,
            Anchor::TopCenter => trigger_bounds.top_center(),
            Anchor::TopRight => trigger_bounds.top_right(),
            Anchor::BottomLeft => Point {
                x: trigger_bounds.origin.x,
                y: trigger_bounds.origin.y - trigger_bounds.size.height,
            },
            Anchor::BottomCenter => Point {
                x: trigger_bounds.top_center().x,
                y: trigger_bounds.origin.y - trigger_bounds.size.height,
            },
            Anchor::BottomRight => Point {
                x: trigger_bounds.top_right().x,
                y: trigger_bounds.origin.y - trigger_bounds.size.height,
            },
            // Fallback for LeftCenter/RightCenter – adjust as needed.
            _ => trigger_bounds.origin,
        }
    }
}

impl ParentElement for AcilirKatman {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for AcilirKatman {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

pub struct AcilirKatmanDurumu {
    focus_handle: FocusHandle,
    pub(crate) tracked_focus_handle: Option<FocusHandle>,
    previous_focus_handle: Option<FocusHandle>,
    trigger_bounds: Bounds<Pixels>,
    trigger_bounds_captured: bool,
    /// Açılır içeriğin son render'da paint edildiği gerçek pencere sınırları.
    /// Yön çevirme (flip) kararı için bir önceki frame'in değeri kullanılır.
    pub(crate) popup_bounds: Bounds<Pixels>,
    pub(crate) popup_bounds_captured: bool,
    /// İlk açılış frame'inde popup'ı görünmez çizip ölçü almayı, ikinci frame'de
    /// (doğru anchor ile) görünür çizmeyi sağlar. Böylece flip kararı verilirken
    /// kullanıcı popup'ın yer değiştirdiğini fark etmez.
    pub(crate) popup_visible: bool,
    /// Bir önceki frame'de yön çevirme kararı verildi mi? Çift atlamayı (oscillation)
    /// engellemek için kararlı tutulur — her render'da `self.anchor`'dan baştan
    /// türetilmez, bir önceki kararın üzerine inşa edilir.
    pub(crate) is_flipped: bool,
    open: bool,
    on_open_change: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,

    _dismiss_subscription: Option<Subscription>,
}

impl AcilirKatmanDurumu {
    pub fn new(default_open: bool, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            tracked_focus_handle: None,
            previous_focus_handle: None,
            trigger_bounds: Bounds::default(),
            trigger_bounds_captured: false,
            popup_bounds: Bounds::default(),
            popup_bounds_captured: false,
            popup_visible: false,
            is_flipped: false,
            open: default_open,
            on_open_change: None,
            _dismiss_subscription: None,
        }
    }

    /// Açılır katmanın açık olup olmadığını kontrol eder.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Açılır katman açıksa kapatır.
    pub fn dismiss(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.open {
            self.toggle_open(window, cx);
        }
    }

    /// Açılır katman kapalıysa açar.
    pub fn show(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            self.toggle_open(window, cx);
        }
    }

    fn set_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.open = open;
        if self.open {
            KureselDurum::global_mut(cx).register_deferred_popover(&self.focus_handle);
        } else {
            KureselDurum::global_mut(cx).unregister_deferred_popover(&self.focus_handle);
            // Bir sonraki açılışta önceki bounds'tan etkilenip yanlış flip
            // verme veya görünür açılma riskini ortadan kaldır.
            self.popup_bounds_captured = false;
            self.popup_visible = false;
            self.is_flipped = false;
        }
    }

    fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let opening = !self.open;
        if opening {
            // Save the focused element before opening, so we can restore it on close.
            self.previous_focus_handle = window.focused(cx);
        }
        self.set_open(opening, cx);
        if self.open {
            let state = cx.entity();
            let focus_handle = if let Some(tracked_focus_handle) = self.tracked_focus_handle.clone()
            {
                tracked_focus_handle
            } else {
                self.focus_handle.clone()
            };
            focus_handle.focus(window, cx);

            self._dismiss_subscription =
                Some(
                    window.subscribe(&cx.entity(), cx, move |_, _: &DismissEvent, window, cx| {
                        state.update(cx, |state, cx| {
                            state.dismiss(window, cx);
                        });
                        window.refresh();
                    }),
                );
        } else {
            self._dismiss_subscription = None;
            // Restore focus to the element that was focused before the popover opened.
            if let Some(prev) = self.previous_focus_handle.take() {
                if self.focus_handle.contains_focused(window, cx) {
                    prev.focus(window, cx);
                }
            }
        }

        if let Some(callback) = self.on_open_change.as_ref() {
            callback(&self.open, window, cx);
        }
        cx.notify();
    }

    fn on_action_cancel(&mut self, _: &Cancel, window: &mut Window, cx: &mut Context<Self>) {
        self.dismiss(window, cx);
    }
}

impl Focusable for AcilirKatmanDurumu {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AcilirKatmanDurumu {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

impl EventEmitter<DismissEvent> for AcilirKatmanDurumu {}

impl AcilirKatman {
    pub(crate) fn render_popover<E>(
        anchor: Anchor,
        position: Rc<Cell<Point<Pixels>>>,
        content: E,
        _: &mut Window,
        _: &mut App,
    ) -> Deferred
    where
        E: IntoElement + 'static,
    {
        deferred(
            anchored()
                .snap_to_window_with_margin(px(8.))
                .anchor(anchor)
                .position(position.get())
                .child(div().relative().child(content)),
        )
        .with_priority(1)
    }

    pub(crate) fn render_popover_content(
        anchor: Anchor,
        appearance: bool,
        _: &mut Window,
        cx: &mut App,
    ) -> Stateful<Div> {
        v_flex()
            .id("content")
            .occlude()
            .tab_group()
            .when(appearance, |this| this.popover_style(cx).p_3())
            .map(|this| match anchor {
                Anchor::TopLeft | Anchor::TopCenter | Anchor::TopRight => this.top_1(),
                Anchor::BottomLeft | Anchor::BottomCenter | Anchor::BottomRight => this.bottom_1(),
                Anchor::LeftCenter | Anchor::RightCenter => this.top_1(), // Fallback for centered
            })
    }
}

impl RenderOnce for AcilirKatman {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let force_open = self.open;
        let default_open = self.default_open;
        let tracked_focus_handle = self.tracked_focus_handle.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, |_, cx| {
            AcilirKatmanDurumu::new(default_open, cx)
        });

        state.update(cx, |state, cx| {
            if let Some(tracked_focus_handle) = tracked_focus_handle {
                state.tracked_focus_handle = Some(tracked_focus_handle);
            }
            state.on_open_change = self.on_open_change.clone();
            if let Some(force_open) = force_open {
                state.set_open(force_open, cx);
            }
        });

        let open = state.read(cx).open;
        let focus_handle = state.read(cx).focus_handle.clone();
        let trigger_bounds = state.read(cx).trigger_bounds;
        let trigger_bounds_captured = state.read(cx).trigger_bounds_captured;
        let popup_bounds = state.read(cx).popup_bounds;
        let popup_bounds_captured = state.read(cx).popup_bounds_captured;
        let popup_visible = state.read(cx).popup_visible;
        let was_flipped = state.read(cx).is_flipped;

        let Some(trigger) = self.trigger else {
            return div().id("empty");
        };

        let parent_view_id = window.current_view();

        // Yön çevirme (flip) kararı.
        //
        // Strateji:
        //  1. **Mevcut yön = `current_anchor`**: bir önceki frame'in `is_flipped`
        //     değerine göre `self.anchor` ya da `flip_vertical(self.anchor)`.
        //     popup_bounds bu yönde ölçülmüştür.
        //  2. Mevcut yönde taşma var mı hesapla.
        //  3. Karşı yön için **dikey simetri varsayımı** ile taşmayı tahmin et
        //     (popup ile trigger arasındaki boşluk korunur).
        //  4. **Histerez marjı** (= trigger yüksekliği): mevcut yön taşıyor ve
        //     karşı yön bu marjdan daha fazla farkla daha iyi olursa flip et.
        //     Aksi takdirde mevcut yönde kal — bu, "popup_h + trigger_h" kadar
        //     dar bir şeritte iki yön de eşit kötü olduğunda iki frame arası
        //     çift atlama (oscillation) olmasını engeller.
        //
        // İlk frame'de `popup_bounds_captured = false` olduğu için karar
        // verilemez; orijinal `self.anchor` kullanılır, bir sonraki render'da
        // gerçek paint bounds'una göre değerlendirilir.
        let current_anchor = if was_flipped {
            Self::flip_vertical(self.anchor)
        } else {
            self.anchor
        };

        let new_flipped = if popup_bounds_captured {
            let viewport = window.viewport_size();
            let opens_down = matches!(
                current_anchor,
                Anchor::TopLeft | Anchor::TopCenter | Anchor::TopRight
            );

            let popup_top = popup_bounds.origin.y;
            let popup_bot = popup_top + popup_bounds.size.height;
            let trigger_top = trigger_bounds.origin.y;
            let trigger_bot = trigger_top + trigger_bounds.size.height;
            let popup_h = popup_bounds.size.height;

            let (flip_top, flip_bot) = if opens_down {
                let gap = popup_top - trigger_bot;
                let new_bot = trigger_top - gap;
                (new_bot - popup_h, new_bot)
            } else {
                let gap = trigger_top - popup_bot;
                let new_top = trigger_bot + gap;
                (new_top, new_top + popup_h)
            };

            let overflow = |top: Pixels, bot: Pixels| -> Pixels {
                let below = (bot - viewport.height).max(Pixels::ZERO);
                let above = (Pixels::ZERO - top).max(Pixels::ZERO);
                below + above
            };

            let overflow_current = overflow(popup_top, popup_bot);
            let overflow_flip = overflow(flip_top, flip_bot);
            let hysteresis = trigger_bounds.size.height;

            if overflow_current > Pixels::ZERO
                && overflow_flip + hysteresis < overflow_current
            {
                !was_flipped
            } else {
                was_flipped
            }
        } else {
            was_flipped
        };

        if new_flipped != was_flipped {
            state.update(cx, |s, _| s.is_flipped = new_flipped);
        }

        let effective_anchor = if new_flipped {
            Self::flip_vertical(self.anchor)
        } else {
            self.anchor
        };

        // Shared cell so the deferred Anchored element can read the real trigger bounds at
        // prepaint time (after trigger's on_prepaint has already fired with the correct bounds).
        let position = Rc::new(Cell::new(Self::resolved_corner(
            effective_anchor,
            trigger_bounds,
        )));

        let el = div()
            .id(self.id)
            .child((trigger)(open, window, cx))
            .on_mouse_down(self.mouse_button, {
                let state = state.clone();
                move |_, window, cx| {
                    cx.stop_propagation();
                    state.update(cx, |state, cx| {
                        // We force set open to false to toggle it correctly.
                        // Because if the mouse down out will toggle open first.
                        state.set_open(open, cx);
                        state.toggle_open(window, cx);
                    });
                    cx.notify(parent_view_id);
                }
            })
            .on_prepaint({
                let state = state.clone();
                let position = position.clone();
                let anchor = effective_anchor;
                move |bounds, window, cx| {
                    position.set(Self::resolved_corner(anchor, bounds));
                    let first_capture = state.update(cx, |state, _| {
                        let first = !state.trigger_bounds_captured;
                        state.trigger_bounds = bounds;
                        state.trigger_bounds_captured = true;
                        first
                    });
                    // On the very first bounds capture, request a new frame so the popover
                    // renders at the correct position (outside the current paint cycle).
                    if first_capture {
                        window.request_animation_frame();
                    }
                }
            });

        if !open || !trigger_bounds_captured {
            return el;
        }

        let popover_content =
            Self::render_popover_content(effective_anchor, self.appearance, window, cx)
                .track_focus(&focus_handle)
                .key_context(CONTEXT)
                .on_action(window.listener_for(&state, AcilirKatmanDurumu::on_action_cancel))
                .when_some(self.content, |this, content| {
                    this.child(state.update(cx, |state, cx| (content)(state, window, cx)))
                })
                .children(self.children)
                .when(self.overlay_closable, |this| {
                    this.on_mouse_down_out({
                        let state = state.clone();
                        move |_, window, cx| {
                            state.update(cx, |state, cx| {
                                state.dismiss(window, cx);
                            });
                            cx.notify(parent_view_id);
                        }
                    })
                })
                .refine_style(&self.style);

        // Popup için inline `anchored` kullanıyoruz; render_popover ile aynı,
        // tek farkı sarmalayıcı div'in `on_prepaint`'inde popup'ın gerçek
        // paint bounds'unu state'e yazıp bir sonraki frame için flip kararı
        // verilebilmesini sağlamamız. İlk frame'de popup `invisible()` olarak
        // çizilir (layout korunur, bounds yakalanır), bir sonraki frame'de
        // doğru anchor'la görünür çizilir → kullanıcı sıçramayı görmez.
        let popup = deferred(
            anchored()
                .snap_to_window_with_margin(px(8.))
                .anchor(effective_anchor)
                .position(position.get())
                .child(
                    div()
                        .relative()
                        .when(!popup_visible, |this| this.invisible())
                        .child(popover_content)
                        .on_prepaint({
                            let state = state.clone();
                            move |bounds, window, cx| {
                                let mut should_request_frame = false;
                                state.update(cx, |s, cx| {
                                    let bounds_changed = s.popup_bounds != bounds;
                                    let first_capture = !s.popup_bounds_captured;
                                    let became_visible = !s.popup_visible;
                                    if bounds_changed || first_capture {
                                        s.popup_bounds = bounds;
                                        s.popup_bounds_captured = true;
                                    }
                                    if became_visible {
                                        s.popup_visible = true;
                                    }
                                    if bounds_changed
                                        || first_capture
                                        || became_visible
                                    {
                                        cx.notify();
                                    }
                                    should_request_frame = first_capture || became_visible;
                                });
                                if should_request_frame {
                                    window.request_animation_frame();
                                }
                            }
                        }),
                ),
        )
        .with_priority(1);

        el.child(popup)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::MouseButton;

    #[test]
    fn test_popover_builder_chaining() {
        let popover = AcilirKatman::new("test")
            .anchor(Anchor::BottomCenter)
            .mouse_button(MouseButton::Right)
            .default_open(true)
            .appearance(false)
            .overlay_closable(false);

        assert_eq!(popover.anchor, Anchor::BottomCenter);
        assert_eq!(popover.mouse_button, MouseButton::Right);
        assert!(popover.default_open);
        assert!(!popover.appearance);
        assert!(!popover.overlay_closable);
    }

    #[test]
    fn test_resolved_corner_top_positions() {
        use gpui::px;

        let bounds = Bounds {
            origin: Point {
                x: px(100.),
                y: px(100.),
            },
            size: gpui::Size {
                width: px(200.),
                height: px(50.),
            },
        };

        let pos = AcilirKatman::resolved_corner(Anchor::TopLeft, bounds);
        assert_eq!(pos.x, px(100.));
        assert_eq!(pos.y, px(100.));

        let pos = AcilirKatman::resolved_corner(Anchor::TopCenter, bounds);
        assert_eq!(pos.x, px(200.));
        assert_eq!(pos.y, px(100.));

        let pos = AcilirKatman::resolved_corner(Anchor::TopRight, bounds);
        assert_eq!(pos.x, px(300.));
        assert_eq!(pos.y, px(100.));

        let pos = AcilirKatman::resolved_corner(Anchor::BottomLeft, bounds);
        assert_eq!(pos.x, px(100.));
        assert_eq!(pos.y, px(50.));

        let pos = AcilirKatman::resolved_corner(Anchor::BottomCenter, bounds);
        assert_eq!(pos.x, px(200.));
        assert_eq!(pos.y, px(50.));

        let pos = AcilirKatman::resolved_corner(Anchor::BottomRight, bounds);
        assert_eq!(pos.x, px(300.));
        assert_eq!(pos.y, px(50.));
    }
}
