use gpui::{App, ElementId, Entity, FocusHandle, Global, OwnedMenu};
use std::collections::HashSet;

use crate::text::MetinGorunumuDurumu;

pub(crate) fn init(cx: &mut App) {
    cx.set_global(KureselDurum::new());
}

impl Global for KureselDurum {}

pub struct KureselDurum {
    pub(crate) text_view_state_stack: Vec<Entity<MetinGorunumuDurumu>>,
    /// Ertelenmiş çizim kullanan açık açılır katman ID kümesini ayarlar.
    /// Bu küme boş değilse en az bir ertelenmiş çizim bağlamındayız.
    /// GPUI panic hatasına yol açabilecek çift ertelenmiş öğeleri önlemek için kullanılır.
    open_deferred_popovers: HashSet<ElementId>,
    /// uygulama menüler storage
    app_menus: Vec<OwnedMenu>,
}

impl KureselDurum {
    pub(crate) fn new() -> Self {
        Self {
            text_view_state_stack: Vec::new(),
            open_deferred_popovers: HashSet::new(),
            app_menus: Vec::new(),
        }
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }

    pub(crate) fn text_view_state(&self) -> Option<&Entity<MetinGorunumuDurumu>> {
        self.text_view_state_stack.last()
    }

    /// Şu anda ertelenmiş çizim bağlamında olup olmadığımızı kontrol eder (ör. açık bir AcilirKatman içinde).
    pub(crate) fn is_in_deferred_context(&self) -> bool {
        !self.open_deferred_popovers.is_empty()
    }

    /// Ertelenmiş çizim kullanan açılır katmanı açık olarak kaydeder.
    pub(crate) fn register_deferred_popover(&mut self, focus_handle: &FocusHandle) {
        self.open_deferred_popovers
            .insert(format!("{focus_handle:?}").into());
    }

    /// Unregister bir açılır katman olduğunda onu closes.
    pub(crate) fn unregister_deferred_popover(&mut self, focus_handle: &FocusHandle) {
        let element_id: ElementId = format!("{focus_handle:?}").into();
        self.open_deferred_popovers.remove(&element_id);
    }

    /// Uygulama menülerini döndürür.
    pub fn app_menus(&self) -> &[OwnedMenu] {
        &self.app_menus
    }

    /// Uygulama menülerini ayarlar.
    pub fn set_app_menus(&mut self, menus: Vec<OwnedMenu>) {
        self.app_menus = menus;
    }
}
