//! Yerlesim is a fixed container that places at left, bottom, right of the Windows.

use std::{ops::Deref, sync::Arc};

use gpui::{
    App, AppContext, Axis, Context, Element, Empty, Entity, IntoElement, MouseMoveEvent,
    MouseUpEvent, ParentElement as _, Pixels, Point, Render, Style, StyleRefinement, Styled as _,
    WeakEntity, Window, div, prelude::FluentBuilder as _, px,
};
use serde::{Deserialize, Serialize};

use crate::{
    StilUzantisi,
    resizable::{PANEL_MIN_SIZE, resize_handle},
};

use super::{PanelView, SekmePaneli, YerlesimAlani, YerlesimOgesi};

#[derive(Clone)]
struct ResizePanel;

impl Render for ResizePanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum YerlesimKonumu {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "right")]
    Right,
}

impl YerlesimKonumu {
    fn axis(&self) -> Axis {
        match self {
            Self::Left | Self::Right => Axis::Horizontal,
            Self::Bottom => Axis::Vertical,
            Self::Center => unreachable!(),
        }
    }

    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left)
    }

    pub fn is_bottom(&self) -> bool {
        matches!(self, Self::Bottom)
    }

    pub fn is_right(&self) -> bool {
        matches!(self, Self::Right)
    }
}

/// Yerlesim; pencerenin sol, alt veya sağ tarafına yerleşen sabit bir kapsayıcıdır.
///
/// Panelden farklı olarak taşınamaz ve içine başka panel eklenemez.
pub struct Yerlesim {
    pub(super) placement: YerlesimKonumu,
    dock_area: WeakEntity<YerlesimAlani>,
    pub(crate) panel: YerlesimOgesi,
    /// Yerlesim genişliği veya yüksekliği. Yerleşim sol ya da sağdaysa genişlik,
    /// aksi halde yükseklik anlamına gelir.
    pub(super) size: Pixels,
    pub(super) open: bool,
    /// Yerlesim değerinin daraltılabilir olup olmadığını belirtir. Varsayılan: true.
    pub(super) collapsible: bool,

    // Runtime state
    /// Yerlesim değerinin yeniden boyutlandırılıp boyutlandırılmadığını belirtir.
    resizing: bool,
}

impl Yerlesim {
    pub(crate) fn new(
        dock_area: WeakEntity<YerlesimAlani>,
        placement: YerlesimKonumu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let panel = cx.new(|cx| {
            let mut tab = SekmePaneli::new(None, dock_area.clone(), window, cx);
            tab.closable = false;
            tab
        });

        let panel = YerlesimOgesi::Tabs {
            size: None,
            items: Vec::new(),
            active_ix: 0,
            view: panel.clone(),
        };

        Self::subscribe_panel_events(dock_area.clone(), &panel, window, cx);

        Self {
            placement,
            dock_area,
            panel,
            open: true,
            collapsible: true,
            size: px(200.0),
            resizing: false,
        }
    }

    pub fn left(
        dock_area: WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(dock_area, YerlesimKonumu::Left, window, cx)
    }

    pub fn bottom(
        dock_area: WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(dock_area, YerlesimKonumu::Bottom, window, cx)
    }

    pub fn right(
        dock_area: WeakEntity<YerlesimAlani>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(dock_area, YerlesimKonumu::Right, window, cx)
    }

    /// Yerlesim olmak için daraltılabilir veya değil. günceller.
    ///
    /// Yerlesim daraltılabilir değilse açık kalır.
    pub fn set_collapsible(&mut self, collapsible: bool, _: &mut Window, cx: &mut Context<Self>) {
        self.collapsible = collapsible;
        if !collapsible {
            self.open = true
        }
        cx.notify();
    }

    pub(super) fn from_state(
        dock_area: WeakEntity<YerlesimAlani>,
        placement: YerlesimKonumu,
        size: Pixels,
        panel: YerlesimOgesi,
        open: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::subscribe_panel_events(dock_area.clone(), &panel, window, cx);

        if !open {
            match panel.clone() {
                YerlesimOgesi::Tabs { view, .. } => {
                    view.update(cx, |panel, cx| {
                        panel.set_collapsed(true, window, cx);
                    });
                }
                YerlesimOgesi::Split { items, .. } => {
                    for item in items {
                        item.set_collapsed(true, window, cx);
                    }
                }
                _ => {}
            }
        }

        Self {
            placement,
            dock_area,
            panel,
            open,
            size,
            collapsible: true,
            resizing: false,
        }
    }

    fn subscribe_panel_events(
        dock_area: WeakEntity<YerlesimAlani>,
        panel: &YerlesimOgesi,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match panel {
            YerlesimOgesi::Tabs { view, .. } => {
                window.defer(cx, {
                    let view = view.clone();
                    move |window, cx| {
                        _ = dock_area.update(cx, |this, cx| {
                            this.subscribe_panel(&view, window, cx);
                        });
                    }
                });
            }
            YerlesimOgesi::Split { items, view, .. } => {
                for item in items {
                    Self::subscribe_panel_events(dock_area.clone(), item, window, cx);
                }
                window.defer(cx, {
                    let view = view.clone();
                    move |window, cx| {
                        _ = dock_area.update(cx, |this, cx| {
                            this.subscribe_panel(&view, window, cx);
                        });
                    }
                });
            }
            YerlesimOgesi::Tiles { view, .. } => {
                window.defer(cx, {
                    let view = view.clone();
                    move |window, cx| {
                        _ = dock_area.update(cx, |this, cx| {
                            this.subscribe_panel(&view, window, cx);
                        });
                    }
                });
            }
            YerlesimOgesi::Panel { .. } => {
                // Not supported
            }
        }
    }

    pub fn set_panel(&mut self, panel: YerlesimOgesi, _: &mut Window, cx: &mut Context<Self>) {
        self.panel = panel;
        cx.notify();
    }

    pub fn panel(&self) -> &YerlesimOgesi {
        &self.panel
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_open(!self.open, window, cx);
    }

    /// Yerlesim boyutunu döndürür; boyut genişlik veya yükseklik anlamına gelir.
    /// Yerleşim sol veya sağ ise boyut genişliktir,
    /// Aksi halde boyut yüksekliktir.
    pub fn size(&self) -> Pixels {
        self.size
    }

    /// boyut Yerlesim ayarlar.
    pub fn set_size(&mut self, size: Pixels, _: &mut Window, cx: &mut Context<Self>) {
        self.size = size.max(PANEL_MIN_SIZE);
        cx.notify();
    }

    /// açık durum Yerlesim ayarlar.
    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.open = open;
        let item = self.panel.clone();
        cx.defer_in(window, move |_, window, cx| {
            item.set_collapsed(!open, window, cx);
        });
        cx.notify();
    }

    /// öğe için Yerlesim. ekler.
    pub fn add_panel(
        &mut self,
        panel: Arc<dyn PanelView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.panel
            .add_panel(panel, &self.dock_area, None, window, cx);
        cx.notify();
    }

    /// Öğeyi Yerlesim içinden kaldırır.
    pub fn remove_panel(
        &mut self,
        panel: Arc<dyn PanelView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.panel.remove_panel(panel, window, cx);
        cx.notify();
    }

    fn render_resize_handle(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let axis = self.placement.axis();
        let view = cx.entity().clone();

        resize_handle("resize-handle", axis)
            .placement(self.placement)
            .on_drag(ResizePanel {}, move |info, _, _, cx| {
                cx.stop_propagation();
                view.update(cx, |view, _| {
                    view.resizing = true;
                });
                cx.new(|_| info.deref().clone())
            })
    }
    fn resize(&mut self, mouse_position: Point<Pixels>, _: &mut Window, cx: &mut Context<Self>) {
        if !self.resizing {
            return;
        }

        let dock_area = self
            .dock_area
            .upgrade()
            .expect("YerlesimAlani eksik")
            .read(cx);
        let area_bounds = dock_area.bounds;
        let mut left_dock_size = px(0.0);
        let mut right_dock_size = px(0.0);

        // Get the size of the left dock if it's open and not the current dock
        if let Some(left_dock) = &dock_area.left_dock {
            if left_dock.entity_id() != cx.entity().entity_id() {
                let left_dock_read = left_dock.read(cx);
                if left_dock_read.is_open() {
                    left_dock_size = left_dock_read.size;
                }
            }
        }

        // Get the size of the right dock if it's open and not the current dock
        if let Some(right_dock) = &dock_area.right_dock {
            if right_dock.entity_id() != cx.entity().entity_id() {
                let right_dock_read = right_dock.read(cx);
                if right_dock_read.is_open() {
                    right_dock_size = right_dock_read.size;
                }
            }
        }

        let size = match self.placement {
            YerlesimKonumu::Left => mouse_position.x - area_bounds.left(),
            YerlesimKonumu::Right => area_bounds.right() - mouse_position.x,
            YerlesimKonumu::Bottom => area_bounds.bottom() - mouse_position.y,
            YerlesimKonumu::Center => unreachable!(),
        };
        match self.placement {
            YerlesimKonumu::Left => {
                let max_size = area_bounds.size.width - PANEL_MIN_SIZE - right_dock_size;
                self.size = size.clamp(PANEL_MIN_SIZE, max_size);
            }
            YerlesimKonumu::Right => {
                let max_size = area_bounds.size.width - PANEL_MIN_SIZE - left_dock_size;
                self.size = size.clamp(PANEL_MIN_SIZE, max_size);
            }
            YerlesimKonumu::Bottom => {
                let max_size = area_bounds.size.height - PANEL_MIN_SIZE;
                self.size = size.clamp(PANEL_MIN_SIZE, max_size);
            }
            YerlesimKonumu::Center => unreachable!(),
        }

        cx.notify();
    }

    fn done_resizing(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.resizing = false;
    }
}

impl Render for Yerlesim {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        if !self.open && !self.placement.is_bottom() {
            return div();
        }

        let cache_style = StyleRefinement::default().absolute().size_full();

        div()
            .relative()
            .overflow_hidden()
            .map(|this| match self.placement {
                YerlesimKonumu::Left | YerlesimKonumu::Right => this.h_flex().h_full().w(self.size),
                YerlesimKonumu::Bottom => this.w_full().h(self.size),
                YerlesimKonumu::Center => unreachable!(),
            })
            // Bottom Yerlesim should keep the title bar, then user can click the Gecis button
            .when(!self.open && self.placement.is_bottom(), |this| {
                this.h(px(29.))
            })
            .map(|this| match &self.panel {
                YerlesimOgesi::Split { view, .. } => this.child(view.clone()),
                YerlesimOgesi::Tabs { view, .. } => this.child(view.clone()),
                YerlesimOgesi::Panel { view, .. } => {
                    this.child(view.clone().view().cached(cache_style))
                }
                // Not support to render Tiles and Tile into Yerlesim
                YerlesimOgesi::Tiles { .. } => this,
            })
            .child(self.render_resize_handle(window, cx))
            .child(DockElement {
                view: cx.entity().clone(),
            })
    }
}

struct DockElement {
    view: Entity<Yerlesim>,
}

impl IntoElement for DockElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for DockElement {
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
        window: &mut gpui::Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, Self::RequestLayoutState) {
        (window.request_layout(Style::default(), None, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: gpui::Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _window: &mut gpui::Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        ()
    }

    fn paint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: gpui::Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut gpui::Window,
        cx: &mut App,
    ) {
        window.on_mouse_event({
            let view = self.view.clone();
            let resizing = view.read(cx).resizing;
            move |e: &MouseMoveEvent, phase, window, cx| {
                if !resizing {
                    return;
                }
                if !phase.bubble() {
                    return;
                }

                view.update(cx, |view, cx| view.resize(e.position, window, cx))
            }
        });

        // When any mouse up, stop dragging
        window.on_mouse_event({
            let view = self.view.clone();
            move |_: &MouseUpEvent, phase, window, cx| {
                if phase.bubble() {
                    view.update(cx, |view, cx| view.done_resizing(window, cx));
                }
            }
        })
    }
}
