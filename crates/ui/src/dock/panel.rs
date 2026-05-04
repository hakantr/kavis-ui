use crate::{button::Dugme, dock::SekmePaneli, menu::AcilirMenu};
use crate::ham_gpui::{
    AnyElement, AnyView, App, AppContext as _, Context, Entity, EntityId, EventEmitter,
    FocusHandle, Focusable, Global, Hsla, IntoElement, Render, SharedString, WeakEntity, Window,
};
use rust_i18n::t;
use std::{collections::HashMap, sync::Arc};

use super::{PanelInfo, PanelState, YerlesimAlani, invalid_panel::InvalidPanel};

pub enum PanelEvent {
    ZoomIn,
    ZoomOut,
    LayoutChanged,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PanelStyle {
    /// Birden çok sekme olduğunda SekmeCubugu gösterir; aksi halde basit başlık gösterir.
    #[default]
    Auto,
    /// Always gösterim sekme çubuk.
    SekmeCubugu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TitleStyle {
    pub background: Hsla,
    pub foreground: Hsla,
}

#[derive(Clone, Copy, Default)]
pub enum PanelControl {
    Both,
    #[default]
    Menu,
    Toolbar,
}

impl PanelControl {
    #[inline]
    pub fn toolbar_visible(&self) -> bool {
        matches!(self, PanelControl::Both | PanelControl::Toolbar)
    }

    #[inline]
    pub fn menu_visible(&self) -> bool {
        matches!(self, PanelControl::Both | PanelControl::Menu)
    }
}

/// Panel tanımlamak için kullanılan özellik.
#[allow(unused_variables)]
pub trait Panel: EventEmitter<PanelEvent> + Render + Focusable {
    /// Paneli serileştirmek, seriden çıkarmak ve tanımlamak için kullanılan ad.
    ///
    /// Panel seriden çıkarılırken paneli tanımlamak için kullanılır.
    /// Panel adını tanımladıktan sonra değiştirmemelisiniz.
    fn panel_name(&self) -> &'static str;

    /// Sekme paneli adı. Varsayılan `None`.
    ///
    /// Daraltılmış sekme panelinde gösterim için kullanılır.
    fn tab_name(&self, cx: &App) -> Option<SharedString> {
        None
    }

    /// Panel başlığı.
    fn title(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        t!("Yerlesim.Unnamed")
    }

    /// Panel başlığı teması. Varsayılan `None`.
    fn title_style(&self, cx: &App) -> Option<TitleStyle> {
        None
    }

    /// Panel başlığı son eki. Varsayılan `None`.
    ///
    /// Panel başlığına bir son ek öğesi eklemek için kullanılır.
    fn title_suffix(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<impl IntoElement> {
        None::<crate::ham_gpui::Div>
    }

    /// Panelin kapatılabilir olup olmadığını döndürür. Varsayılan `true`.
    ///
    /// Bu yöntem panel çizilirken çağrılır; hızlı çalışmasına dikkat edilmelidir.
    fn closable(&self, cx: &App) -> bool {
        true
    }

    /// Panel yakınlaştırılabiliyorsa `PanelControl` döndürür. Varsayılan `PanelControl::Menu`.
    ///
    /// Bu yöntem panel çizilirken çağrılır; hızlı çalışmasına dikkat edilmelidir.
    fn zoomable(&self, cx: &App) -> Option<PanelControl> {
        Some(PanelControl::Menu)
    }

    /// Paneli göstermek için true, gizlemek için false döndürür. Varsayılan `true`.
    ///
    /// Bu yöntem panel çizilirken çağrılır; hızlı çalışmasına dikkat edilmelidir.
    fn visible(&self, cx: &App) -> bool {
        true
    }

    /// Panelin etkin durumunu ayarlar.
    ///
    /// Panel etkin veya etkin olmayan duruma geçtiğinde bu yöntem çağrılır.
    ///
    /// Panel etkin olduğunda last_active_panel ve current_active_panel güncellenir.
    fn set_active(&mut self, active: bool, window: &mut Window, cx: &mut Context<Self>) {}

    /// Panelin yakınlaştırılmış durumunu ayarlar.
    ///
    /// Panel yakınlaştırıldığında veya yakınlaştırmadan çıkarıldığında bu yöntem çağrılır.
    ///
    /// Bu yönteme yalnızca geçerli panel dokunur.
    fn set_zoomed(&mut self, zoomed: bool, window: &mut Window, cx: &mut Context<Self>) {}

    /// Bu panel bir SekmePaneli'ne eklendiğinde çağrılır.
    fn on_added_to(
        &mut self,
        tab_panel: WeakEntity<SekmePaneli>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
    }

    /// Bu panel bir SekmePaneli'nden kaldırıldığında çağrılır.
    fn on_removed(&mut self, window: &mut Window, cx: &mut Context<Self>) {}

    /// Panele ek açılır menü sağlar. Varsayılan `None`.
    fn acilir_menu(
        &mut self,
        this: AcilirMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AcilirMenu {
        this
    }

    /// Panel sağ başlık çubuğunda gösterilecek ek araç çubuğu düğmeleri sağlar. Varsayılan `None`.
    fn toolbar_buttons(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Vec<Dugme>> {
        None
    }

    /// Paneli serileştirmek için panel durumunu döker.
    fn dump(&self, cx: &App) -> PanelState {
        PanelState::new(self)
    }

    /// Panel tabs yerleşimindeyken iç dolgusu olup olmadığını döndürür. Varsayılan `true`.
    fn inner_padding(&self, cx: &App) -> bool {
        true
    }
}

/// Panel görünümü tanımlamak için kullanılan özellik.
#[allow(unused_variables)]
pub trait PanelView: 'static + Send + Sync {
    fn panel_name(&self, cx: &App) -> &'static str;
    fn panel_id(&self, cx: &App) -> EntityId;
    fn tab_name(&self, cx: &App) -> Option<SharedString>;
    fn title(&self, window: &mut Window, cx: &mut App) -> AnyElement;
    fn title_suffix(&self, window: &mut Window, cx: &mut App) -> Option<AnyElement>;
    fn title_style(&self, cx: &App) -> Option<TitleStyle>;
    fn closable(&self, cx: &App) -> bool;
    fn zoomable(&self, cx: &App) -> Option<PanelControl>;
    fn visible(&self, cx: &App) -> bool;
    fn set_active(&self, active: bool, window: &mut Window, cx: &mut App);
    fn set_zoomed(&self, zoomed: bool, window: &mut Window, cx: &mut App);
    fn on_added_to(&self, tab_panel: WeakEntity<SekmePaneli>, window: &mut Window, cx: &mut App);
    fn on_removed(&self, window: &mut Window, cx: &mut App);
    fn acilir_menu(&self, menu: AcilirMenu, window: &mut Window, cx: &mut App) -> AcilirMenu;
    fn toolbar_buttons(&self, window: &mut Window, cx: &mut App) -> Option<Vec<Dugme>>;
    fn view(&self) -> AnyView;
    fn focus_handle(&self, cx: &App) -> FocusHandle;
    fn dump(&self, cx: &App) -> PanelState;
    fn inner_padding(&self, cx: &App) -> bool;
}

impl<T: Panel> PanelView for Entity<T> {
    fn panel_name(&self, cx: &App) -> &'static str {
        self.read(cx).panel_name()
    }

    fn panel_id(&self, _: &App) -> EntityId {
        self.entity_id()
    }

    fn tab_name(&self, cx: &App) -> Option<SharedString> {
        self.read(cx).tab_name(cx)
    }

    fn title(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        self.update(cx, |this, cx| this.title(window, cx).into_any_element())
    }

    fn title_suffix(&self, window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        self.update(cx, |this, cx| {
            this.title_suffix(window, cx)
                .map(|el| el.into_any_element())
        })
    }

    fn title_style(&self, cx: &App) -> Option<TitleStyle> {
        self.read(cx).title_style(cx)
    }

    fn closable(&self, cx: &App) -> bool {
        self.read(cx).closable(cx)
    }

    fn zoomable(&self, cx: &App) -> Option<PanelControl> {
        self.read(cx).zoomable(cx)
    }

    fn visible(&self, cx: &App) -> bool {
        self.read(cx).visible(cx)
    }

    fn set_active(&self, active: bool, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| {
            this.set_active(active, window, cx);
        })
    }

    fn set_zoomed(&self, zoomed: bool, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| {
            this.set_zoomed(zoomed, window, cx);
        })
    }

    fn on_added_to(&self, tab_panel: WeakEntity<SekmePaneli>, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.on_added_to(tab_panel, window, cx));
    }

    fn on_removed(&self, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.on_removed(window, cx));
    }

    fn acilir_menu(&self, menu: AcilirMenu, window: &mut Window, cx: &mut App) -> AcilirMenu {
        self.update(cx, |this, cx| this.acilir_menu(menu, window, cx))
    }

    fn toolbar_buttons(&self, window: &mut Window, cx: &mut App) -> Option<Vec<Dugme>> {
        self.update(cx, |this, cx| this.toolbar_buttons(window, cx))
    }

    fn view(&self) -> AnyView {
        self.clone().into()
    }

    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.read(cx).focus_handle(cx)
    }

    fn dump(&self, cx: &App) -> PanelState {
        self.read(cx).dump(cx)
    }

    fn inner_padding(&self, cx: &App) -> bool {
        self.read(cx).inner_padding(cx)
    }
}

impl From<&dyn PanelView> for AnyView {
    fn from(handle: &dyn PanelView) -> Self {
        handle.view()
    }
}

impl<T: Panel> From<&dyn PanelView> for Entity<T> {
    fn from(value: &dyn PanelView) -> Self {
        value.view().downcast::<T>().unwrap()
    }
}

impl PartialEq for dyn PanelView {
    fn eq(&self, other: &Self) -> bool {
        self.view() == other.view()
    }
}

pub struct PanelRegistry {
    pub(super) items: HashMap<
        String,
        Arc<
            dyn Fn(
                WeakEntity<YerlesimAlani>,
                &PanelState,
                &PanelInfo,
                &mut Window,
                &mut App,
            ) -> Box<dyn PanelView>,
        >,
    >,
}
impl PanelRegistry {
    /// Panel kaydını başlatır.
    pub(crate) fn init(cx: &mut App) {
        if let None = cx.try_global::<PanelRegistry>() {
            cx.set_global(PanelRegistry::new());
        }
    }

    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<PanelRegistry>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<PanelRegistry>()
    }

    /// Adıyla bir panel oluşturur.
    ///
    /// Kayıtlı değilse InvalidPanel döndürür.
    pub fn build_panel(
        panel_name: &str,
        dock_area: WeakEntity<YerlesimAlani>,
        panel_state: &PanelState,
        panel_info: &PanelInfo,
        window: &mut Window,
        cx: &mut App,
    ) -> Box<dyn PanelView> {
        if let Some(view) = Self::global(cx)
            .items
            .get(panel_name)
            .cloned()
            .map(|f| f(dock_area, panel_state, panel_info, window, cx))
        {
            return view;
        } else {
            // Show an invalid panel if the panel is not registered.
            Box::new(cx.new(|cx| InvalidPanel::new(&panel_name, panel_state.clone(), window, cx)))
        }
    }
}
impl Global for PanelRegistry {}

/// `panel_name` ile panel başlatıcısını global kayda ekler.
pub fn register_panel<F>(cx: &mut App, panel_name: &str, deserialize: F)
where
    F: Fn(
            WeakEntity<YerlesimAlani>,
            &PanelState,
            &PanelInfo,
            &mut Window,
            &mut App,
        ) -> Box<dyn PanelView>
        + 'static,
{
    PanelRegistry::init(cx);
    PanelRegistry::global_mut(cx)
        .items
        .insert(panel_name.to_string(), Arc::new(deserialize));
}
