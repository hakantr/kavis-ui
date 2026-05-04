use std::{
    ops::{Deref, Range},
    rc::Rc,
};

use gpui::{
    Along, AnyElement, App, AppContext, Axis, Bounds, Context, Element, ElementId, Empty, Entity,
    EventEmitter, InteractiveElement as _, IntoElement, IsZero as _, MouseMoveEvent, MouseUpEvent,
    ParentElement, Pixels, Render, RenderOnce, Style, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder,
};

use crate::{
    AxisExt, OgeUzantisi, h_flex, resizable::PANEL_MIN_SIZE, styled::StilUzantisi as _, v_flex,
};

use super::{YenidenBoyutlandirilabilirDurum, resizable_panel, resize_handle};

pub enum YenidenBoyutlandirilabilirPanelOlayi {
    Resized,
}

#[derive(Clone)]
pub(crate) struct DragPanel;
impl Render for DragPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<'_, Self>) -> impl IntoElement {
        Empty
    }
}

/// Yeniden boyutlandırılabilir panel grubu.
#[derive(IntoElement)]
pub struct YenidenBoyutlandirilabilirPanelGrubu {
    id: ElementId,
    state: Option<Entity<YenidenBoyutlandirilabilirDurum>>,
    axis: Axis,
    size: Option<Pixels>,
    children: Vec<YenidenBoyutlandirilabilirPanel>,
    on_resize: Rc<dyn Fn(&Entity<YenidenBoyutlandirilabilirDurum>, &mut Window, &mut App)>,
}

impl YenidenBoyutlandirilabilirPanelGrubu {
    /// Yeni bir yeniden boyutlandırılabilir panel grubu oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            axis: Axis::Horizontal,
            children: vec![],
            state: None,
            size: None,
            on_resize: Rc::new(|_, _, _| {}),
        }
    }

    /// Harici bir yeniden boyutlandırılabilir durum varlığına bağlanır.
    ///
    /// Sağlanmazsa grup kendi durumunu içeride yönetir.
    pub fn with_state(mut self, state: &Entity<YenidenBoyutlandirilabilirDurum>) -> Self {
        self.state = Some(state.clone());
        self
    }

    /// Yeniden boyutlandırılabilir panel grubunun eksenini ayarlar. Varsayılan yataydır.
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// Gruba bir panel ekler.
    ///
    /// - `axis`, grubun ekseniyle aynı değere ayarlanır.
    /// - `initial_size` sağlanmazsa tüm panellerin ortalama boyutuna ayarlanır.
    /// - `grup`, grup varlığına ayarlanır.
    pub fn child(mut self, panel: impl Into<YenidenBoyutlandirilabilirPanel>) -> Self {
        self.children.push(panel.into());
        self
    }

    /// Gruba birden çok panel ekler.
    pub fn children<I>(mut self, panels: impl IntoIterator<Item = I>) -> Self
    where
        I: Into<YenidenBoyutlandirilabilirPanel>,
    {
        self.children = panels.into_iter().map(|panel| panel.into()).collect();
        self
    }

    /// Yeniden boyutlandırılabilir panel grubunun boyutunu ayarlar.
    ///
    /// - Eksen yataysa boyut grubun yüksekliğidir.
    /// - Eksen dikeyse boyut grubun genişliğidir.
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = Some(size);
        self
    }

    /// Paneller yeniden boyutlandırıldığında çağrılacak geri çağrıyı ayarlar.
    ///
    /// ## geri çağrı argümanlar
    ///
    /// - Entity<YenidenBoyutlandirilabilirDurum>: durum YenidenBoyutlandirilabilirPanelGrubu.
    pub fn on_resize(
        mut self,
        on_resize: impl Fn(&Entity<YenidenBoyutlandirilabilirDurum>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_resize = Rc::new(on_resize);
        self
    }
}

impl<T> From<T> for YenidenBoyutlandirilabilirPanel
where
    T: Into<AnyElement>,
{
    fn from(value: T) -> Self {
        resizable_panel().child(value.into())
    }
}

impl From<YenidenBoyutlandirilabilirPanelGrubu> for YenidenBoyutlandirilabilirPanel {
    fn from(value: YenidenBoyutlandirilabilirPanelGrubu) -> Self {
        resizable_panel().child(value)
    }
}

impl EventEmitter<YenidenBoyutlandirilabilirPanelOlayi> for YenidenBoyutlandirilabilirPanelGrubu {}

impl RenderOnce for YenidenBoyutlandirilabilirPanelGrubu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self
            .state
            .unwrap_or(window.use_keyed_state(self.id.clone(), cx, |_, _| {
                YenidenBoyutlandirilabilirDurum::default()
            }));
        let container = if self.axis.is_horizontal() {
            h_flex()
        } else {
            v_flex()
        };

        // Sync panels to the state
        let panels_count = self.children.len();
        state.update(cx, |state, cx| {
            state.sync_panels_count(self.axis, panels_count, cx);
        });

        container
            .id(self.id)
            .size_full()
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(|(ix, mut panel)| {
                        panel.panel_ix = ix;
                        panel.axis = self.axis;
                        panel.state = Some(state.clone());
                        panel
                    }),
            )
            .on_prepaint({
                let state = state.clone();
                move |bounds, _, cx| {
                    state.update(cx, |state, cx| {
                        let size_changed =
                            state.bounds.size.along(self.axis) != bounds.size.along(self.axis);

                        state.bounds = bounds;

                        if size_changed {
                            state.adjust_to_container_size(cx);
                        }
                    })
                }
            })
            .child(ResizePanelGroupElement {
                state: state.clone(),
                axis: self.axis,
                on_resize: self.on_resize.clone(),
            })
    }
}

/// Bir yeniden boyutlandırılabilir panel içinde bir [`YenidenBoyutlandirilabilirPanelGrubu`].
///
/// [`Styled`] uygular; bu yüzden çağrı noktaları panelin çizilmiş stillerini
/// geçersiz kılabilir. Kullanıcı geçersiz kılmaları panelin flex varsayılanları
/// ile boyut yönetimi arasında uygulanır. Çağıran taraf dahili `flex_grow: 1`
/// değerini (ör. `.flex_none()` ile) geçersiz kılabilir ve kendi dolgu, renk
/// veya kenarlıklarını ekleyebilir; ancak çalışma zamanındaki boyut kısıtları
/// (`min_w`/`max_w`/`flex_basis`, `YenidenBoyutlandirilabilirDurum` tarafından
/// sürülür) her zaman baskın gelir.
///
/// Yaygın bir geçersiz kılma `.flex_none()` çağrısıdır: panel içeride
/// `flex_grow: 1` ayarlar. Bu yüzden genişliğini kardeş öğe daralırken
/// koruması gereken boyutlu bir panelin `.flex_none()` ile büyümeden çıkması gerekir.
///
/// ```ignore
/// h_resizable("layout")
///     .child(resizable_panel().size(px(220.)).flex_none().child(sidebar))
///     .child(resizable_panel().child(content))                // flex
///     .child(resizable_panel().size(px(280.)).flex_none().child(metadata))
/// ```
///
/// **Ayrılmış stiller**: bunları dışarıdan çağırmayın; panelin kendi yerleşim
/// yönetimiyle çakışırlar:
/// - `.flex_basis(...)` çağıran tarafından değil, `YenidenBoyutlandirilabilirDurum`
///   tarafından sürülür.
/// - `.absolute()` paneli yeniden boyutlandırılabilir flex akışından çıkarır.
/// - `.overflow_hidden()` ilk panelden sonraki her panelde `left: -4px` mutlak
///   konumuna yerleştirilen yeniden boyutlandırma işleyicisini kırpabilir.
#[derive(IntoElement)]
pub struct YenidenBoyutlandirilabilirPanel {
    axis: Axis,
    panel_ix: usize,
    state: Option<Entity<YenidenBoyutlandirilabilirDurum>>,
    /// Panel oluşturulduğunda sahip olacağı başlangıç boyutu.
    initial_size: Option<Pixels>,
    /// Bu paneli sınırlandıran boyut aralığı.
    size_range: Range<Pixels>,
    children: Vec<AnyElement>,
    visible: bool,
    style: StyleRefinement,
}

impl YenidenBoyutlandirilabilirPanel {
    /// Yeni bir yeniden boyutlandırılabilir panel oluşturur.
    pub(super) fn new() -> Self {
        Self {
            panel_ix: 0,
            initial_size: None,
            state: None,
            size_range: (PANEL_MIN_SIZE..Pixels::MAX),
            axis: Axis::Horizontal,
            children: vec![],
            visible: true,
            style: StyleRefinement::default(),
        }
    }

    /// Panel görünürlüğünü ayarlar. Varsayılan true.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// Panelin başlangıç boyutunu ayarlar.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.initial_size = Some(size.into());
        self
    }

    /// Panel yeniden boyutlandırması için boyut aralığı sınırını ayarlar.
    ///
    /// Varsayılan [`PANEL_MIN_SIZE`] ile [`Pixels::MAX`] aralığıdır.
    pub fn size_range(mut self, range: impl Into<Range<Pixels>>) -> Self {
        self.size_range = range.into();
        self
    }
}

impl Styled for YenidenBoyutlandirilabilirPanel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for YenidenBoyutlandirilabilirPanel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for YenidenBoyutlandirilabilirPanel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.visible {
            return div().id(("resizable-panel", self.panel_ix));
        }

        let state = self
            .state
            .expect("HATA: YenidenBoyutlandirilabilirPanel içindeki `state` mevcut olmalı.");
        let panel_state = state.read(cx).panels.get(self.panel_ix).expect(
            "HATA: YenidenBoyutlandirilabilirPanel `index` değeri `state` içinde bulunmalı.",
        );
        let size_range = self.size_range.clone();

        div()
            .id(("resizable-panel", self.panel_ix))
            .flex()
            .flex_grow()
            .size_full()
            .relative()
            // Apply caller style overrides here — between the flex defaults
            // above and the size management below. This lets callers cancel
            // the unconditional `.flex_grow()` (via `.flex_none()`, the load-
            // bearing case for sized panels next to a collapsing sibling) and
            // add their own padding / colors / borders, while keeping the
            // panel's runtime size constraints (min/max + `flex_basis` driven
            // by `YenidenBoyutlandirilabilirDurum`) authoritative.
            .refine_style(&self.style)
            .when(self.axis.is_vertical(), |this| {
                this.min_h(size_range.start).max_h(size_range.end)
            })
            .when(self.axis.is_horizontal(), |this| {
                this.min_w(size_range.start).max_w(size_range.end)
            })
            // 1. initial_size is None, to use auto size.
            // 2. initial_size is Some and size is none, to use the initial size of the panel for first time render.
            // 3. initial_size is Some and size is Some, use `size`.
            .when(self.initial_size.is_none(), |this| this.flex_shrink())
            .when_some(self.initial_size, |this, initial_size| {
                // The `self.size` is None, that mean the initial size for the panel,
                // so we need set `flex_shrink_0` To let it keep the initial size.
                this.when(
                    panel_state.size.is_none() && !initial_size.is_zero(),
                    |this| this.flex_none(),
                )
                .flex_basis(initial_size)
            })
            .map(|this| match panel_state.size {
                Some(size) => this.flex_basis(size.min(size_range.end).max(size_range.start)),
                None => this,
            })
            .on_prepaint({
                let state = state.clone();
                move |bounds, _, cx| {
                    state.update(cx, |state, cx| {
                        state.update_panel_size(self.panel_ix, bounds, self.size_range, cx)
                    })
                }
            })
            .children(self.children)
            .when(self.panel_ix > 0, |this| {
                let ix = self.panel_ix - 1;
                this.child(resize_handle(("resizable-handle", ix), self.axis).on_drag(
                    DragPanel,
                    move |drag_panel, _, _, cx| {
                        cx.stop_propagation();
                        // Set current resizing panel ix
                        state.update(cx, |state, _| {
                            state.resizing_panel_ix = Some(ix);
                        });
                        cx.new(|_| drag_panel.deref().clone())
                    },
                ))
            })
    }
}

struct ResizePanelGroupElement {
    state: Entity<YenidenBoyutlandirilabilirDurum>,
    on_resize: Rc<dyn Fn(&Entity<YenidenBoyutlandirilabilirDurum>, &mut Window, &mut App)>,
    axis: Axis,
}

impl IntoElement for ResizePanelGroupElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ResizePanelGroupElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<gpui::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        (window.request_layout(Style::default(), None, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        ()
    }

    fn paint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        window.on_mouse_event({
            let state = self.state.clone();
            let axis = self.axis;
            let current_ix = state.read(cx).resizing_panel_ix;
            move |e: &MouseMoveEvent, phase, window, cx| {
                if !phase.bubble() {
                    return;
                }
                let Some(ix) = current_ix else { return };

                state.update(cx, |state, cx| {
                    let panel = state.panels.get(ix).expect("HATA: geçersiz panel indeksi");

                    match axis {
                        Axis::Horizontal => state.resize_panel_at_handle(
                            ix,
                            e.position.x - panel.bounds.left(),
                            window,
                            cx,
                        ),
                        Axis::Vertical => state.resize_panel_at_handle(
                            ix,
                            e.position.y - panel.bounds.top(),
                            window,
                            cx,
                        ),
                    }
                    cx.notify();
                })
            }
        });

        // When any mouse up, stop dragging
        window.on_mouse_event({
            let state = self.state.clone();
            let current_ix = state.read(cx).resizing_panel_ix;
            let on_resize = self.on_resize.clone();
            move |_: &MouseUpEvent, phase, window, cx| {
                if current_ix.is_none() {
                    return;
                }
                if phase.bubble() {
                    state.update(cx, |state, cx| state.done_resizing(cx));
                    on_resize(&state, window, cx);
                }
            }
        })
    }
}
