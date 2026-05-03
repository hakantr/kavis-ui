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

/// Acilir bir bilesenin tetikleyiciye gore acilis yonu.
///
/// `Anchor`'in soyut "kose" semantiginin uzerine, popover'lar icin daha dogal
/// bir "yon" yuzeyi sunar. Yatay hizalamayi gizler (default sol kenar); ozel
/// hiza icin [`AcilirKatman::anchor`] dogrudan kullanilabilir.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Yon {
    /// Popup tetikleyicinin altina acilir.
    Asagi,
    /// Popup tetikleyicinin ustune acilir.
    Yukari,
}

impl From<Yon> for Anchor {
    fn from(yon: Yon) -> Self {
        match yon {
            // TopLeft: popup origin = trigger top-left -> popup asagiya, sol hizali.
            Yon::Asagi => Anchor::TopLeft,
            // BottomLeft: popup origin = trigger top-left'in ustu -> popup yukariya, sol hizali.
            Yon::Yukari => Anchor::BottomLeft,
        }
    }
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
    /// True ise popup içeriği tetikleyici genişliğine kilitlenir.
    match_trigger_width: bool,
    /// True ise popup, ekran sinirina sigmiyorsa anchor'i zit kenara otomatik flip eder
    /// (GPUI `SwitchAnchor` mod'u). False ise mevcut "kenara yapis" davranisi (margin'li).
    auto_flip: bool,
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
            match_trigger_width: false,
            auto_flip: false,
            default_open: false,
            open: None,
            on_open_change: None,
        }
    }

    /// True ise açılır pencere genişliği tetikleyici öğenin genişliğine eşitlenir.
    /// Varsayılan: `false`.
    pub fn match_trigger_width(mut self, value: bool) -> Self {
        self.match_trigger_width = value;
        self
    }

    /// Acilis yonunu Turkce ad ile ayarlar.
    ///
    /// [`Anchor`] ile dogrudan ozel hiza belirlemek icin [`Self::anchor`] kullanilabilir.
    pub fn yon(mut self, yon: impl Into<Yon>) -> Self {
        self.anchor = yon.into().into();
        self
    }

    /// True ise popup ekran sinirina sigmiyorsa anchor zit kenara otomatik cevrilir
    /// (ornegin asagi acilim icin yer yoksa yukari acilir). Varsayilan `false`.
    ///
    /// Not: GPUI'nin tek `fit_mode` modeli nedeniyle bu mod, kenar margin'i
    /// (mevcut 8px) yerine SwitchAnchor mantigi kullanir; kenara yapisma yine
    /// gerceklesir ama margin sifir olur.
    pub fn otomatik_yon(mut self, value: bool) -> Self {
        self.auto_flip = value;
        self
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

    /// Anchor'i dikey eksende cevirir (`Top*` <-> `Bottom*`). Yatay olanlar dokunulmaz.
    pub(crate) fn flip_anchor_vertical(anchor: Anchor) -> Anchor {
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

    /// Tetikleyici/viewport olculerine gore istenen anchor'i, popup'in
    /// gorunur kalmasini saglayacak sekilde flip eder.
    ///
    /// Sadece dikey flip (asagi <-> yukari) destekler; yatay tasmaya GPUI'nin
    /// `snap_to_window_with_margin` mantigi karar verir. `auto_flip` false ise
    /// istenen anchor degisiklikten gecirilmeden donulur.
    pub(crate) fn choose_anchor(
        requested: Anchor,
        auto_flip: bool,
        trigger_bounds: Bounds<Pixels>,
        viewport: gpui::Size<Pixels>,
    ) -> Anchor {
        if !auto_flip {
            return requested;
        }

        // Tetikleyicinin altinda ve ustundeki kullanilabilir alani olc.
        let space_below = (viewport.height - trigger_bounds.bottom_left().y).max(px(0.));
        let space_above = trigger_bounds.origin.y.max(px(0.));
        // Popup boyutunu onceden bilmiyoruz; bu esik altinda "alt taraf yetersiz"
        // sayilir. Tipik bir dropdown menu en az ~120px ister.
        let min_needed = px(120.);

        let opens_down = matches!(
            requested,
            Anchor::TopLeft | Anchor::TopCenter | Anchor::TopRight
        );
        let opens_up = matches!(
            requested,
            Anchor::BottomLeft | Anchor::BottomCenter | Anchor::BottomRight
        );

        if opens_down && space_below < min_needed && space_above > space_below {
            Self::flip_anchor_vertical(requested)
        } else if opens_up && space_above < min_needed && space_below > space_above {
            Self::flip_anchor_vertical(requested)
        } else {
            requested
        }
    }

    /// Anchor'a gore popup'in tetikleyiciye yapismasi gereken referans noktasini doner.
    ///
    /// `Top*` anchor'lari popup'in **ust kenarini** tetikleyicinin **alt kenarina**
    /// hizalar (popup asagi acilir, trigger ile cakismaz).
    ///
    /// `Bottom*` anchor'lari popup'in **alt kenarini** tetikleyicinin **ust kenarina**
    /// hizalar (popup yukari acilir).
    pub(crate) fn resolved_corner(anchor: Anchor, trigger_bounds: Bounds<Pixels>) -> Point<Pixels> {
        match anchor {
            Anchor::TopLeft => trigger_bounds.bottom_left(),
            Anchor::TopCenter => trigger_bounds.bottom_center(),
            Anchor::TopRight => trigger_bounds.bottom_right(),
            Anchor::BottomLeft => trigger_bounds.origin,
            Anchor::BottomCenter => trigger_bounds.top_center(),
            Anchor::BottomRight => trigger_bounds.top_right(),
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
        auto_flip: bool,
        content: E,
        _: &mut Window,
        _: &mut App,
    ) -> Deferred
    where
        E: IntoElement + 'static,
    {
        // auto_flip true ise GPUI'nin `anchored()` default'u olan SwitchAnchor moduna
        // birakilir; bu mod, popup ekrana sigmiyorsa anchor'i zit kenara cevirir.
        // false ise mevcut "kenara yapis" davranisi (8px margin) korunur.
        let mut element = anchored().anchor(anchor).position(position.get());
        if !auto_flip {
            element = element.snap_to_window_with_margin(px(8.));
        }
        deferred(element.child(div().relative().child(content))).with_priority(1)
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

        let Some(trigger) = self.trigger else {
            return div().id("empty");
        };

        let parent_view_id = window.current_view();
        let viewport_size = window.viewport_size();
        let requested_anchor = self.anchor;
        let auto_flip = self.auto_flip;

        // Render zamaninda istenen anchor'i, tetikleyicinin viewport icindeki konumuna
        // gore flip et. Boylece popup pencere alt sinirinda kalmissa yukari, ust
        // sinirinda kalmissa asagi acilir; popup'in tetikleyiciyi kapatmadan gosterilir.
        let final_anchor =
            Self::choose_anchor(requested_anchor, auto_flip, trigger_bounds, viewport_size);

        // Shared cell so the deferred Anchored element can read the real trigger bounds at
        // prepaint time (after trigger's on_prepaint has already fired with the correct bounds).
        let position = Rc::new(Cell::new(Self::resolved_corner(
            final_anchor,
            trigger_bounds,
        )));

        // Tetikleyiciyi kendi sarmalayicisina al ve on_prepaint'i ona bagla.
        // Boylece bounds saf trigger ile sinirli kalir; popup absolute olsa da
        // deferred/anchored kombinasyonu outer div'in bounds'una katki yapinca
        // trigger.bottom hesabinin popup yukseligi kadar kayma sorunu olusmaz.
        let trigger_wrapper = div()
            .child((trigger)(open, window, cx))
            .on_prepaint({
                let state = state.clone();
                let position = position.clone();
                move |bounds, window, cx| {
                    let viewport = window.viewport_size();
                    let chosen =
                        Self::choose_anchor(requested_anchor, auto_flip, bounds, viewport);
                    position.set(Self::resolved_corner(chosen, bounds));
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

        let el = div()
            .id(self.id)
            .child(trigger_wrapper)
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
            });

        if !open || !trigger_bounds_captured {
            return el;
        }

        let popover_content =
            Self::render_popover_content(final_anchor, self.appearance, window, cx)
                .track_focus(&focus_handle)
                .key_context(CONTEXT)
                .on_action(window.listener_for(&state, AcilirKatmanDurumu::on_action_cancel))
                .when(self.match_trigger_width, |this| {
                    // Tetikleyici genişliğine kilitle: container ve içeriği aynı genişlikte kalsın.
                    this.w(trigger_bounds.size.width)
                })
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

        // Anchor'i biz sectik ve position'i ona gore hesapladik; GPUI'nin
        // SwitchAnchor mantigina gerek yok. SnapToWindowWithMargin'i koruyup
        // 8px kenar bosluguyla yatay tasmaya karsi koruma alinir.
        el.child(Self::render_popover(
            final_anchor,
            position,
            false,
            popover_content,
            window,
            cx,
        ))
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
    fn test_resolved_corner_aligns_to_trigger_edges() {
        use gpui::px;

        // Trigger: top-left=(100,100), size=(200,50) -> bounds x:[100,300], y:[100,150].
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

        // Top* anchor'lari popup'i trigger'in altina yaslar -> y = trigger.bottom = 150.
        let pos = AcilirKatman::resolved_corner(Anchor::TopLeft, bounds);
        assert_eq!(pos.x, px(100.));
        assert_eq!(pos.y, px(150.));

        let pos = AcilirKatman::resolved_corner(Anchor::TopCenter, bounds);
        assert_eq!(pos.x, px(200.));
        assert_eq!(pos.y, px(150.));

        let pos = AcilirKatman::resolved_corner(Anchor::TopRight, bounds);
        assert_eq!(pos.x, px(300.));
        assert_eq!(pos.y, px(150.));

        // Bottom* anchor'lari popup'i trigger'in ustune yaslar -> y = trigger.top = 100.
        let pos = AcilirKatman::resolved_corner(Anchor::BottomLeft, bounds);
        assert_eq!(pos.x, px(100.));
        assert_eq!(pos.y, px(100.));

        let pos = AcilirKatman::resolved_corner(Anchor::BottomCenter, bounds);
        assert_eq!(pos.x, px(200.));
        assert_eq!(pos.y, px(100.));

        let pos = AcilirKatman::resolved_corner(Anchor::BottomRight, bounds);
        assert_eq!(pos.x, px(300.));
        assert_eq!(pos.y, px(100.));
    }

    #[test]
    fn test_choose_anchor_flips_when_no_room_below() {
        use gpui::px;

        // Tetikleyici pencerenin alt kismina yakin: trigger.bottom = 580, viewport = 600.
        // Alt 20px, ust 580px alan var -> asagi acilim icin yer yok, flip beklenir.
        let trigger = Bounds {
            origin: Point {
                x: px(100.),
                y: px(560.),
            },
            size: gpui::Size {
                width: px(120.),
                height: px(20.),
            },
        };
        let viewport = gpui::Size {
            width: px(800.),
            height: px(600.),
        };

        // auto_flip=false -> istenen anchor degismeden donulur
        assert_eq!(
            AcilirKatman::choose_anchor(Anchor::TopLeft, false, trigger, viewport),
            Anchor::TopLeft
        );
        // auto_flip=true -> flip beklenir
        assert_eq!(
            AcilirKatman::choose_anchor(Anchor::TopLeft, true, trigger, viewport),
            Anchor::BottomLeft
        );
    }

    #[test]
    fn test_choose_anchor_does_not_flip_when_room_exists() {
        use gpui::px;

        // Tetikleyici ortada: alt ve ust tarafta yeterli alan var.
        let trigger = Bounds {
            origin: Point {
                x: px(100.),
                y: px(280.),
            },
            size: gpui::Size {
                width: px(120.),
                height: px(40.),
            },
        };
        let viewport = gpui::Size {
            width: px(800.),
            height: px(600.),
        };

        assert_eq!(
            AcilirKatman::choose_anchor(Anchor::TopLeft, true, trigger, viewport),
            Anchor::TopLeft
        );
        assert_eq!(
            AcilirKatman::choose_anchor(Anchor::BottomLeft, true, trigger, viewport),
            Anchor::BottomLeft
        );
    }

    #[test]
    fn test_choose_anchor_flips_up_to_down_at_top() {
        use gpui::px;

        // Tetikleyici pencerenin ust kismina yakin: ust 5px, alt 575px.
        // Yukari acilim icin yer yok -> Bottom* -> Top* flip.
        let trigger = Bounds {
            origin: Point {
                x: px(100.),
                y: px(5.),
            },
            size: gpui::Size {
                width: px(120.),
                height: px(20.),
            },
        };
        let viewport = gpui::Size {
            width: px(800.),
            height: px(600.),
        };

        assert_eq!(
            AcilirKatman::choose_anchor(Anchor::BottomLeft, true, trigger, viewport),
            Anchor::TopLeft
        );
    }
}
