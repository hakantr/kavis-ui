use gpui::{
    AnyElement, App, Bounds, Element, ElementId, FocusHandle, Global, GlobalElementId,
    InteractiveElement, Interactivity, IntoElement, LayoutId, ParentElement, Pixels,
    StatefulInteractiveElement, StyleRefinement, Styled, WeakFocusHandle, Window,
};
use std::collections::HashMap;

/// Initialize odak trap yönetici olarak bir global
pub(crate) fn init(cx: &mut App) {
    cx.set_global(OdakTuzagiYoneticisi::new());
}

/// Bir uzantı özellik için ekler `focus_trap` işlev için etkileşimli öğeler.
pub trait OdakTuzagiOgesi: InteractiveElement + Sized {
    /// Bu öğe için odak tuzağını etkinleştirir.
    ///
    /// Etkin olduğunda odak bu kapsayıcı içinde otomatik olarak döner.
    /// bunun yerine escaping için üst öğe öğeler. Bu kullanışlıdır için modal iletişim kutuları,
    /// sheets, ve diğer kaplama bileşenler.
    ///
    /// odak trap çalışır ile:
    /// 1. Registering bu öğe olarak bir odak trap kapsayıcı
    /// 2. Sekme/Shift-Sekme basıldığında KokGorunum olayı yakalar
    /// 3. Eğer odak olurdu leave kapsayıcı, onu cycles back için beginning/bitiş
    ///
    /// # Örnek
    ///
    /// ```ignore
    /// v_flex()
    ///     .child(Dugme::new("btn1").label("Dugme 1"))
    ///     .child(Dugme::new("btn2").label("Dugme 2"))
    ///     .child(Dugme::new("btn3").label("Dugme 3"))
    ///     .odak_tuzagi("trap1", &self.container_focus_handle)
    /// // Sekme tuşuna basmak döngü yapar: btn1 -> btn2 -> btn3 -> btn1
    /// // Odak bu kapsayıcının dışındaki öğelere kaçmaz
    /// ```
    ///
    /// Bakınız ayrıca: <https://github.com/odak-trap/odak-trap-react>
    fn odak_tuzagi(
        self,
        id: impl Into<ElementId>,
        focus_handle: &FocusHandle,
    ) -> OdakTuzagiKapsayici<Self>
    where
        Self: ParentElement + Styled + Element + 'static,
    {
        OdakTuzagiKapsayici::new(id, focus_handle.clone(), self)
    }
}
impl<T: InteractiveElement + Sized> OdakTuzagiOgesi for T {}

/// Global durum için manage tüm odak trap containers
pub(crate) struct OdakTuzagiYoneticisi {
    /// Kapsayıcı öğe ID değerinden odak kapanı bilgisine eşleme.
    traps: HashMap<GlobalElementId, WeakFocusHandle>,
}

impl Global for OdakTuzagiYoneticisi {}

impl OdakTuzagiYoneticisi {
    /// Yeni bir odak trap yönetici oluşturur.
    fn new() -> Self {
        Self {
            traps: HashMap::new(),
        }
    }

    pub(crate) fn global(cx: &App) -> &Self {
        cx.global::<OdakTuzagiYoneticisi>()
    }

    fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<OdakTuzagiYoneticisi>()
    }

    /// kaydeder bir odak trap kapsayıcı
    fn register_trap(id: &GlobalElementId, container_handle: WeakFocusHandle, cx: &mut App) {
        let this = Self::global_mut(cx);
        this.traps.insert(id.clone(), container_handle);
        this.cleanup();
    }

    /// Şu anda odaklanmış öğeyi içeren odak kapanını bulur.
    pub(crate) fn find_active_trap(window: &Window, cx: &App) -> Option<FocusHandle> {
        for (_id, container_handle) in Self::global(cx).traps.iter() {
            let Some(container) = container_handle.upgrade() else {
                continue;
            };

            if container.contains_focused(window, cx) {
                return Some(container.clone());
            }
        }
        None
    }

    /// Cleanup herhangi bir traps ile dropped handles
    fn cleanup(&mut self) {
        self.traps.retain(|_, handle| handle.upgrade().is_some());
    }
}

impl Default for OdakTuzagiYoneticisi {
    fn default() -> Self {
        Self::new()
    }
}

/// Odak kapanı davranışını uygulayan sarmalayıcı öğe.
///
/// Bu öğe wraps başka bir öğe ve registers onu olarak bir odak trap kapsayıcı.
/// Sekme/Shift-Sekme basıldığında odak kapsayıcı içinde otomatik olarak döner.
pub struct OdakTuzagiKapsayici<E: InteractiveElement + ParentElement + Styled + Element> {
    id: ElementId,
    focus_handle: FocusHandle,
    base: E,
}

impl<E: InteractiveElement + ParentElement + Styled + Element> OdakTuzagiKapsayici<E> {
    pub(crate) fn new(id: impl Into<ElementId>, focus_handle: FocusHandle, child: E) -> Self {
        Self {
            id: id.into(),
            base: child.track_focus(&focus_handle),
            focus_handle,
        }
    }
}

impl<E: InteractiveElement + ParentElement + Styled + Element> IntoElement
    for OdakTuzagiKapsayici<E>
{
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
impl<E: InteractiveElement + ParentElement + Styled + Element> ParentElement
    for OdakTuzagiKapsayici<E>
{
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.base.extend(elements);
    }
}
impl<E: InteractiveElement + ParentElement + Styled + Element> InteractiveElement
    for OdakTuzagiKapsayici<E>
{
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}
impl<E: InteractiveElement + ParentElement + Styled + Element> StatefulInteractiveElement
    for OdakTuzagiKapsayici<E>
{
}
impl<E: InteractiveElement + ParentElement + Styled + Element> Styled for OdakTuzagiKapsayici<E> {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl<E: InteractiveElement + ParentElement + Styled + Element + 'static> Element
    for OdakTuzagiKapsayici<E>
{
    type RequestLayoutState = E::RequestLayoutState;
    type PrepaintState = E::PrepaintState;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&gpui::GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        // Register this focus trap with the manager
        OdakTuzagiYoneticisi::register_trap(global_id.unwrap(), self.focus_handle.downgrade(), cx);

        self.base.request_layout(global_id, None, window, cx)
    }

    fn prepaint(
        &mut self,
        global_id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.base
            .prepaint(global_id, inspector_id, bounds, request_layout, window, cx)
    }

    fn paint(
        &mut self,
        global_id: Option<&gpui::GlobalElementId>,
        inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.base.paint(
            global_id,
            inspector_id,
            bounds,
            request_layout,
            prepaint,
            window,
            cx,
        )
    }
}
