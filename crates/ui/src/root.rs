use crate::{
    ElementExt, EtkinTema, Placement, StilUzantisi,
    dialog::{ANIMATION_DURATION, IletisimKutusu},
    focus_trap::FocusTrapManager,
    input::InputState,
    notification::{Bildirim, BildirimListesi},
    sheet::SayfaKatmani,
    tooltip::AracIpucuKatmani,
    window_border,
};
use gpui::{
    Anchor, AnyView, App, AppContext, Context, DefiniteLength, ElementId, Entity, FocusHandle,
    InteractiveElement, IntoElement, KeyBinding, ParentElement as _, Pixels, Render,
    StyleRefinement, Styled, WeakFocusHandle, Window, actions, div, prelude::FluentBuilder as _,
};
use std::{any::TypeId, rc::Rc};

actions!(root, [Sekme, TabPrev]);

const CONTEXT: &str = "KokGorunum";
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("tab", Sekme, Some(CONTEXT)),
        KeyBinding::new("shift-tab", TabPrev, Some(CONTEXT)),
    ]);
}

/// KokGorunum, uygulama penceresinin üst seviye görünümüdür ve penceredeki ilk görünüm olmalıdır.
///
/// SayfaKatmani, IletisimKutusu ve Bildirim yönetimi için kullanılır.
pub struct KokGorunum {
    style: StyleRefinement,
    view: AnyView,
    pub(crate) active_sheet: Option<ActiveSheet>,
    pub(crate) active_dialogs: Vec<ActiveDialog>,
    pub(super) focused_input: Option<Entity<InputState>>,
    pub notification: Entity<BildirimListesi>,
    pub(crate) tooltip_overlay: Entity<AracIpucuKatmani>,
    sheet_size: Option<DefiniteLength>,
    window_shadow_size: Pixels,
    /// İletişim kutusu animasyonla kapandıktan sonra geri yüklenecek odak işleyicisi.
    /// Hızlı iletişim kutusu açma/kapatma durumlarında doğru odak zincirini korumak için kullanılır.
    pending_focus_restore: Option<WeakFocusHandle>,
}

#[derive(Clone)]
pub(crate) struct ActiveSheet {
    focus_handle: FocusHandle,
    /// önceki odaklanmış işleyici önce açma SayfaKatmani.
    previous_focused_handle: Option<WeakFocusHandle>,
    placement: Placement,
    builder: Rc<dyn Fn(SayfaKatmani, &mut Window, &mut App) -> SayfaKatmani + 'static>,
}

#[derive(Clone)]
pub(crate) struct ActiveDialog {
    focus_handle: FocusHandle,
    /// önceki odaklanmış işleyici önce açma IletisimKutusu.
    previous_focused_handle: Option<WeakFocusHandle>,
    builder: Rc<dyn Fn(IletisimKutusu, &mut Window, &mut App) -> IletisimKutusu + 'static>,
}

impl ActiveDialog {
    pub(crate) fn new(
        focus_handle: FocusHandle,
        previous_focused_handle: Option<WeakFocusHandle>,
        builder: impl Fn(IletisimKutusu, &mut Window, &mut App) -> IletisimKutusu + 'static,
    ) -> Self {
        Self {
            focus_handle,
            previous_focused_handle,
            builder: Rc::new(builder),
        }
    }
}

impl KokGorunum {
    /// Yeni bir KokGorunum görünüm oluşturur.
    pub fn new(view: impl Into<AnyView>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            style: StyleRefinement::default(),
            view: view.into(),
            active_sheet: None,
            active_dialogs: Vec::new(),
            focused_input: None,
            notification: cx.new(|cx| BildirimListesi::new(window, cx)),
            tooltip_overlay: cx.new(|_| AracIpucuKatmani::new()),
            sheet_size: None,
            window_shadow_size: window_border::SHADOW_SIZE,
            pending_focus_restore: None,
        }
    }

    /// pencere kenarlık gölge boyut için Linux client-taraf decorations ayarlar.
    ///
    /// varsayılan: [`window_border::SHADOW_SIZE`]
    pub fn window_shadow_size(mut self, size: impl Into<Pixels>) -> Self {
        self.window_shadow_size = size.into();
        self
    }

    pub fn update<F, R>(window: &mut Window, cx: &mut App, f: F) -> R
    where
        F: FnOnce(&mut Self, &mut Window, &mut Context<Self>) -> R,
    {
        let root = window
            .root::<KokGorunum>()
            .flatten()
            .expect("HATA: pencerenin ilk katmanı kavis_ui::KokGorunum olmalı.");

        root.update(cx, |root, cx| f(root, window, cx))
    }

    pub fn read<'a>(window: &'a Window, cx: &'a App) -> &'a Self {
        &window
            .root::<KokGorunum>()
            .expect("Pencere kök görünümü `ui::KokGorunum` türünde olmalı.")
            .unwrap()
            .read(cx)
    }

    // Render Bildirim layer.
    pub fn render_notification_layer(
        window: &mut Window,
        cx: &mut App,
    ) -> Option<impl IntoElement + use<>> {
        let root = window.root::<KokGorunum>()??;

        let active_sheet_placement = root.read(cx).active_sheet.clone().map(|d| d.placement);

        let sheet_size = root.read(cx).sheet_size;
        let (mt, mr, mb, ml) = match active_sheet_placement {
            Some(Placement::Top) => (sheet_size, None, None, None),
            Some(Placement::Right) => (None, sheet_size, None, None),
            Some(Placement::Bottom) => (None, None, sheet_size, None),
            Some(Placement::Left) => (None, None, None, sheet_size),
            _ => (None, None, None, None),
        };

        let placement = cx.theme().notification.placement;

        Some(
            div()
                .absolute()
                .when(matches!(placement, Anchor::TopRight), |this| {
                    this.top_0().right_0()
                })
                .when(matches!(placement, Anchor::TopLeft), |this| {
                    this.top_0().left_0()
                })
                .when(matches!(placement, Anchor::TopCenter), |this| {
                    this.top_0().mx_auto()
                })
                .when(matches!(placement, Anchor::BottomRight), |this| {
                    this.bottom_0().right_0()
                })
                .when(matches!(placement, Anchor::BottomLeft), |this| {
                    this.bottom_0().left_0()
                })
                .when(matches!(placement, Anchor::BottomCenter), |this| {
                    this.bottom_0().mx_auto()
                })
                .when_some(mt, |this, offset| this.mt(offset))
                .when_some(mr, |this, offset| this.mr(offset))
                .when_some(mb, |this, offset| this.mb(offset))
                .when_some(ml, |this, offset| this.ml(offset))
                .child(root.read(cx).notification.clone()),
        )
    }

    /// SayfaKatmani katmanını çizer.
    pub fn render_sheet_layer(
        window: &mut Window,
        cx: &mut App,
    ) -> Option<impl IntoElement + use<>> {
        let root = window.root::<KokGorunum>()??;

        if let Some(active_sheet) = root.read(cx).active_sheet.clone() {
            let mut sheet = SayfaKatmani::new(window, cx);
            sheet = (active_sheet.builder)(sheet, window, cx);
            sheet.focus_handle = active_sheet.focus_handle.clone();
            sheet.placement = active_sheet.placement;

            let size = sheet.size;

            return Some(
                div()
                    .relative()
                    .child(sheet)
                    .on_prepaint(move |_, _, cx| root.update(cx, |r, _| r.sheet_size = Some(size))),
            );
        }

        None
    }

    /// IletisimKutusu katmanını çizer.
    pub fn render_dialog_layer(
        window: &mut Window,
        cx: &mut App,
    ) -> Option<impl IntoElement + use<>> {
        let root = window.root::<KokGorunum>()??;

        let active_dialogs = root.read(cx).active_dialogs.clone();

        if active_dialogs.is_empty() {
            return None;
        }

        let mut show_overlay_ix = None;

        let mut dialogs = active_dialogs
            .iter()
            .enumerate()
            .map(|(i, active_dialog)| {
                let mut dialog = IletisimKutusu::new(cx);

                dialog = (active_dialog.builder)(dialog, window, cx);

                // Give the dialog the focus handle, because `dialog` is a temporary value, is not possible to
                // keep the focus handle in the dialog.
                //
                // So we keep the focus handle in the `active_dialog`, this is owned by the `KokGorunum`.
                dialog.focus_handle = active_dialog.focus_handle.clone();

                dialog.layer_ix = i;
                // Find the dialog which one needs to show overlay.
                if dialog.has_overlay() {
                    show_overlay_ix = Some(i);
                }

                dialog
            })
            .collect::<Vec<_>>();

        if let Some(ix) = show_overlay_ix {
            if let Some(dialog) = dialogs.get_mut(ix) {
                dialog.props.overlay_visible = true;
            }
        }

        Some(div().children(dialogs))
    }

    pub fn open_dialog<F>(
        &mut self,
        build: F,
        window: &mut Window,
        cx: &mut Context<'_, KokGorunum>,
    ) where
        F: Fn(IletisimKutusu, &mut Window, &mut App) -> IletisimKutusu + 'static,
    {
        let mut previous_focused_handle = window.focused(cx).map(|h| h.downgrade());

        // Use pending focus restore if available to maintain correct focus chain
        // when a new dialog is opened immediately after closing another dialog.
        if let Some(pending_handle) = self.pending_focus_restore.take() {
            previous_focused_handle = Some(pending_handle);
        }

        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        self.active_dialogs.push(ActiveDialog::new(
            focus_handle,
            previous_focused_handle,
            build,
        ));
        cx.notify();
    }

    fn close_dialog_internal(&mut self) -> Option<FocusHandle> {
        self.focused_input = None;
        self.active_dialogs
            .pop()
            .and_then(|d| d.previous_focused_handle)
            .and_then(|h| h.upgrade())
    }

    pub fn close_dialog(&mut self, window: &mut Window, cx: &mut Context<'_, KokGorunum>) {
        if let Some(handle) = self.close_dialog_internal() {
            window.focus(&handle, cx);
        }
        cx.notify();
    }

    pub(crate) fn defer_close_dialog(
        &mut self,
        window: &mut Window,
        cx: &mut Context<'_, KokGorunum>,
    ) {
        if let Some(handle) = self.close_dialog_internal() {
            let dialogs_count = self.active_dialogs.len();

            // Save for new dialogs opened during animation to maintain focus chain
            self.pending_focus_restore = Some(handle.downgrade());

            cx.spawn_in(window, async move |this, cx| {
                cx.background_executor().timer(*ANIMATION_DURATION).await;
                let _ = this.update_in(cx, |this, window, cx| {
                    let current_dialogs_count = this.active_dialogs.len();
                    // Only restore focus if no new dialogs were opened during animation
                    if current_dialogs_count == dialogs_count {
                        window.focus(&handle, cx);
                    }
                    this.pending_focus_restore = None;
                });
            })
            .detach();
        }
        cx.notify();
    }

    pub fn close_all_dialogs(&mut self, window: &mut Window, cx: &mut Context<'_, KokGorunum>) {
        self.focused_input = None;
        let previous_focused_handle = self
            .active_dialogs
            .first()
            .and_then(|d| d.previous_focused_handle.clone());
        self.active_dialogs.clear();
        if let Some(handle) = previous_focused_handle.and_then(|h| h.upgrade()) {
            window.focus(&handle, cx);
        }
        cx.notify();
    }

    pub fn open_sheet_at<F>(
        &mut self,
        placement: Placement,
        build: F,
        window: &mut Window,
        cx: &mut Context<'_, KokGorunum>,
    ) where
        F: Fn(SayfaKatmani, &mut Window, &mut App) -> SayfaKatmani + 'static,
    {
        let previous_focused_handle = self
            .active_sheet
            .take()
            .and_then(|s| s.previous_focused_handle)
            .or_else(|| window.focused(cx).map(|h| h.downgrade()));

        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);
        self.active_sheet = Some(ActiveSheet {
            focus_handle,
            previous_focused_handle,
            placement,
            builder: Rc::new(build),
        });
        cx.notify();
    }

    pub fn close_sheet(&mut self, window: &mut Window, cx: &mut Context<'_, KokGorunum>) {
        self.focused_input = None;
        if let Some(previous_handle) = self
            .active_sheet
            .as_ref()
            .and_then(|s| s.previous_focused_handle.as_ref())
            .and_then(|h| h.upgrade())
        {
            window.focus(&previous_handle, cx);
        }
        self.active_sheet = None;
        cx.notify();
    }

    pub fn push_notification(
        &mut self,
        note: impl Into<Bildirim>,
        window: &mut Window,
        cx: &mut Context<'_, KokGorunum>,
    ) {
        self.notification
            .update(cx, |view, cx| view.push(note, window, cx));
        cx.notify();
    }

    /// Kaldırır tüm bildirimler whose id matches `T`, dahil ones registered ile
    /// [`Bildirim::id`] veya [`Bildirim::id1`] değerlerinden biri (herhangi bir anahtar).
    pub fn remove_notification<T: Sized + 'static>(
        &mut self,
        window: &mut Window,
        cx: &mut Context<'_, KokGorunum>,
    ) {
        self.notification.update(cx, |view, cx| {
            view.close_by_type(TypeId::of::<T>(), window, cx);
        });
        cx.notify();
    }

    /// Kaldırır bildirim eşleşen verilen tip ve öğe id (paired ile [`Bildirim::id1`]).
    pub fn remove_notification1<T: Sized + 'static>(
        &mut self,
        key: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut Context<'_, KokGorunum>,
    ) {
        let key = key.into();
        self.notification.update(cx, |view, cx| {
            view.close((TypeId::of::<T>(), key), window, cx);
        });
        cx.notify();
    }

    pub fn clear_notifications(&mut self, window: &mut Window, cx: &mut Context<'_, KokGorunum>) {
        self.notification
            .update(cx, |view, cx| view.clear(window, cx));
        cx.notify();
    }

    /// araç ipucu kaplama entity için bu pencere döndürür.
    pub(crate) fn tooltip_overlay(window: &Window, cx: &App) -> Option<Entity<AracIpucuKatmani>> {
        let root = window.root::<KokGorunum>()??;
        Some(root.read(cx).tooltip_overlay.clone())
    }

    /// root görünüm KokGorunum döndürür.
    pub fn view(&self) -> &AnyView {
        &self.view
    }

    fn on_action_tab(&mut self, _: &Sekme, window: &mut Window, cx: &mut Context<Self>) {
        // Check if we're inside a focus trap
        if let Some(container_focus_handle) = FocusTrapManager::find_active_trap(window, cx) {
            // We're in a focus trap - try to focus next, then check if we're still inside
            let before_focus = window.focused(cx);

            // Try normal focus navigation
            window.focus_next(cx);

            // Check if we're still in the trap
            if !container_focus_handle.contains_focused(window, cx) {
                // We jumped out of the trap - need to cycle back to the beginning
                // Find the first focusable element in the trap by continuing to focus_next
                let mut attempts = 0;
                const MAX_ATTEMPTS: usize = 100; // Prevent infinite loop

                while !container_focus_handle.contains_focused(window, cx)
                    && attempts < MAX_ATTEMPTS
                {
                    window.focus_next(cx);
                    attempts += 1;

                    // If we cycled back to where we started, restore original focus
                    if window.focused(cx) == before_focus {
                        break;
                    }
                }
            }
            return;
        }

        // Normal tab navigation
        window.focus_next(cx);
    }

    fn on_action_tab_prev(&mut self, _: &TabPrev, window: &mut Window, cx: &mut Context<Self>) {
        // Check if we're inside a focus trap
        if let Some(container_focus_handle) = FocusTrapManager::find_active_trap(window, cx) {
            // We're in a focus trap - try to focus previous, then check if we're still inside
            let before_focus = window.focused(cx);

            // Try normal focus navigation
            window.focus_prev(cx);

            // Check if we're still in the trap
            if !container_focus_handle.contains_focused(window, cx) {
                // We jumped out of the trap - need to cycle back to the end
                // Find the last focusable element in the trap by continuing to focus_prev
                let mut attempts = 0;
                const MAX_ATTEMPTS: usize = 100; // Prevent infinite loop

                while !container_focus_handle.contains_focused(window, cx)
                    && attempts < MAX_ATTEMPTS
                {
                    window.focus_prev(cx);
                    attempts += 1;

                    // If we cycled back to where we started, restore original focus
                    if window.focused(cx) == before_focus {
                        break;
                    }
                }
            }
            return;
        }

        // Normal tab navigation
        window.focus_prev(cx);
    }
}

impl Styled for KokGorunum {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Render for KokGorunum {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        window.set_rem_size(cx.theme().font_size);

        window_border().shadow_size(self.window_shadow_size).child(
            div()
                .id("root")
                .key_context(CONTEXT)
                .on_action(cx.listener(Self::on_action_tab))
                .on_action(cx.listener(Self::on_action_tab_prev))
                .relative()
                .size_full()
                .font_family(cx.theme().font_family.clone())
                .bg(cx.theme().background)
                .text_color(cx.theme().foreground)
                .refine_style(&self.style)
                .child(self.view.clone())
                .child(self.tooltip_overlay.clone()),
        )
    }
}
