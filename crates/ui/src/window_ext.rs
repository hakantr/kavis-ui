use crate::{
    KokGorunum, Placement,
    dialog::{IletisimKutusu, UyariIletisimKutusu},
    input::InputState,
    notification::Bildirim,
    sheet::SayfaKatmani,
};
use crate::ham_gpui::{App, ElementId, Entity, Window};
use std::rc::Rc;

/// uzantı özellik için [`Window`] için ekler iletişim kutusu, sayfa katmanı. işlev.
pub trait PencereUzantisi: Sized {
    /// Sağ yerleşimde bir SayfaKatmani açar.
    fn open_sheet<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(SayfaKatmani, &mut Window, &mut App) -> SayfaKatmani + 'static;

    /// Verilen yerleşimde bir SayfaKatmani açar.
    fn open_sheet_at<F>(&mut self, placement: Placement, cx: &mut App, build: F)
    where
        F: Fn(SayfaKatmani, &mut Window, &mut App) -> SayfaKatmani + 'static;

    /// Etkin bir SayfaKatmani varsa true döndürür.
    fn has_active_sheet(&mut self, cx: &mut App) -> bool;

    /// Closes etkin SayfaKatmani.
    fn close_sheet(&mut self, cx: &mut App);

    /// Bir IletisimKutusu açar.
    fn open_dialog<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(IletisimKutusu, &mut Window, &mut App) -> IletisimKutusu + 'static;

    /// Bir UyariIletisimKutusu açar.
    ///
    /// Bu bir convenience yöntem için açma bir uyarı iletişim kutusu ile opinionated varsayılan değer.
    /// Alt bilgi düğmeleri ortalanır ve varyanta bağlı bir simge içerir.
    ///
    /// # Örnekler
    ///
    /// ```ignore
    /// use kavis_ui::{UyariIletisimKutusu, alert::UyariVaryanti};
    ///
    /// window.open_alert_dialog(cx, |alert, _, _| {
    ///     alert.warning()
    ///         .title("Unsaved Changes")
    ///         .description("You have unsaved changes. Are you sure you want to leave?")
    ///         .show_cancel(true)
    /// });
    /// ```
    fn open_alert_dialog<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(UyariIletisimKutusu, &mut Window, &mut App) -> UyariIletisimKutusu + 'static;

    /// Etkin bir IletisimKutusu varsa true döndürür.
    fn has_active_dialog(&mut self, cx: &mut App) -> bool;

    /// Closes son etkin IletisimKutusu.
    fn close_dialog(&mut self, cx: &mut App);

    /// Closes tüm etkin Dialogs.
    fn close_all_dialogs(&mut self, cx: &mut App);

    /// Pushes bir bildirim için bildirim liste.
    fn push_notification(&mut self, note: impl Into<Bildirim>, cx: &mut App);

    /// Kaldırır tüm bildirimler whose id matches `T`, dahil ones registered ile
    /// `Bildirim::id` veya `Bildirim::id1` değerlerinden biri (herhangi bir anahtar).
    fn remove_notification<T: Sized + 'static>(&mut self, cx: &mut App);

    /// Kaldırır tek bildirim eşleşen verilen tip `T` ve `key` (paired ile `Bildirim::id1`).
    fn remove_notification1<T: Sized + 'static>(&mut self, key: impl Into<ElementId>, cx: &mut App);

    /// Clears tüm bildirimler.
    fn clear_notifications(&mut self, cx: &mut App);

    /// sayı bildirimler. döndürür.
    fn notifications(&mut self, cx: &mut App) -> Rc<Vec<Entity<Bildirim>>>;

    /// geçerli odaklanmış girdi entity. döndürür.
    fn focused_input(&mut self, cx: &mut App) -> Option<Entity<InputState>>;
    /// Odaklanmış bir girdi varlığı varsa true döndürür.
    fn has_focused_input(&mut self, cx: &mut App) -> bool;
}

impl PencereUzantisi for Window {
    #[inline]
    fn open_sheet<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(SayfaKatmani, &mut Window, &mut App) -> SayfaKatmani + 'static,
    {
        self.open_sheet_at(Placement::Right, cx, build)
    }

    #[inline]
    fn open_sheet_at<F>(&mut self, placement: Placement, cx: &mut App, build: F)
    where
        F: Fn(SayfaKatmani, &mut Window, &mut App) -> SayfaKatmani + 'static,
    {
        KokGorunum::update(self, cx, move |root, window, cx| {
            root.open_sheet_at(placement, build, window, cx);
        })
    }

    #[inline]
    fn has_active_sheet(&mut self, cx: &mut App) -> bool {
        KokGorunum::read(self, cx).active_sheet.is_some()
    }

    #[inline]
    fn close_sheet(&mut self, cx: &mut App) {
        KokGorunum::update(self, cx, |root, window, cx| {
            root.close_sheet(window, cx);
        })
    }

    #[inline]
    fn open_dialog<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(IletisimKutusu, &mut Window, &mut App) -> IletisimKutusu + 'static,
    {
        KokGorunum::update(self, cx, move |root, window, cx| {
            root.open_dialog(build, window, cx);
        })
    }

    #[inline]
    fn open_alert_dialog<F>(&mut self, cx: &mut App, build: F)
    where
        F: Fn(UyariIletisimKutusu, &mut Window, &mut App) -> UyariIletisimKutusu + 'static,
    {
        self.open_dialog(cx, move |_, window, cx| {
            build(UyariIletisimKutusu::new(cx), window, cx).into_dialog(window, cx)
        })
    }

    #[inline]
    fn has_active_dialog(&mut self, cx: &mut App) -> bool {
        KokGorunum::read(self, cx).active_dialogs.len() > 0
    }

    #[inline]
    fn close_dialog(&mut self, cx: &mut App) {
        KokGorunum::update(self, cx, |root, window, cx| {
            root.close_dialog(window, cx);
        })
    }

    #[inline]
    fn close_all_dialogs(&mut self, cx: &mut App) {
        KokGorunum::update(self, cx, |root, window, cx| {
            root.close_all_dialogs(window, cx);
        })
    }

    #[inline]
    fn push_notification(&mut self, note: impl Into<Bildirim>, cx: &mut App) {
        let note = note.into();
        KokGorunum::update(self, cx, |root, window, cx| {
            root.push_notification(note, window, cx);
        })
    }

    #[inline]
    fn remove_notification<T: Sized + 'static>(&mut self, cx: &mut App) {
        KokGorunum::update(self, cx, |root, window, cx| {
            root.remove_notification::<T>(window, cx);
        })
    }

    #[inline]
    fn remove_notification1<T: Sized + 'static>(
        &mut self,
        key: impl Into<ElementId>,
        cx: &mut App,
    ) {
        let key = key.into();
        KokGorunum::update(self, cx, |root, window, cx| {
            root.remove_notification1::<T>(key, window, cx);
        })
    }

    #[inline]
    fn clear_notifications(&mut self, cx: &mut App) {
        KokGorunum::update(self, cx, |root, window, cx| {
            root.clear_notifications(window, cx);
        })
    }

    #[inline]
    fn notifications(&mut self, cx: &mut App) -> Rc<Vec<Entity<Bildirim>>> {
        Rc::new(
            KokGorunum::read(self, cx)
                .notification
                .read(cx)
                .notifications(),
        )
    }

    #[inline]
    fn has_focused_input(&mut self, cx: &mut App) -> bool {
        KokGorunum::read(self, cx).focused_input.is_some()
    }

    #[inline]
    fn focused_input(&mut self, cx: &mut App) -> Option<Entity<InputState>> {
        KokGorunum::read(self, cx).focused_input.clone()
    }
}
