use std::{ops::Deref, rc::Rc};

use wry::{
    Rect,
    dpi::{self, LogicalSize},
};

use kavis_ui::{
    App, Bounds, Element, ElementId, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, ParentElement as _, Pixels, Render, Styled as _, Window, div,
    ham_gpui::{
        ContentMask, DismissEvent, GlobalElementId, Hitbox, HitboxBehavior, InspectorElementId,
        LayoutId, MouseDownEvent, Size, Style, canvas,
    },
};

/// wry WebGorunumu tabanlı bir web görünümü.
///
/// [deneysel]
pub struct WebGorunumu {
    focus_handle: FocusHandle,
    webview: Rc<wry::WebView>,
    visible: bool,
    bounds: Bounds<Pixels>,
}

impl Drop for WebGorunumu {
    fn drop(&mut self) {
        self.hide();
    }
}

impl WebGorunumu {
    /// wry WebGorunumu üzerinden yeni bir WebGorunumu oluşturur.
    pub fn new(webview: wry::WebView, _: &mut Window, cx: &mut App) -> Self {
        let _ = webview.set_bounds(Rect::default());

        Self {
            focus_handle: cx.focus_handle(),
            visible: true,
            bounds: Bounds::default(),
            webview: Rc::new(webview),
        }
    }

    /// Web görünümünü gösterir.
    pub fn show(&mut self) {
        let _ = self.webview.set_visible(true);
        self.visible = true;
    }

    /// Web görünümünü gizler.
    pub fn hide(&mut self) {
        _ = self.webview.focus_parent();
        _ = self.webview.set_visible(false);
        self.visible = false;
    }

    /// Web görünümünün görünür olup olmadığını döndürür.
    pub fn visible(&self) -> bool {
        self.visible
    }

    /// Web görünümünün geçerli sınırlarını döndürür.
    pub fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    /// Web görünümü geçmişinde geri gider.
    pub fn back(&mut self) -> anyhow::Result<()> {
        Ok(self.webview.evaluate_script("history.back();")?)
    }

    /// Web görünümünde bir URL yükler.
    pub fn load_url(&mut self, url: &str) {
        let _ = self.webview.load_url(url);
    }

    /// Ham wry web görünümünü döndürür.
    pub fn raw(&self) -> &wry::WebView {
        &self.webview
    }
}

impl Deref for WebGorunumu {
    type Target = wry::WebView;

    fn deref(&self) -> &Self::Target {
        &self.webview
    }
}

impl Focusable for WebGorunumu {
    fn focus_handle(&self, _cx: &kavis_ui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for WebGorunumu {}

impl Render for WebGorunumu {
    fn render(
        &mut self,
        window: &mut kavis_ui::Window,
        cx: &mut kavis_ui::Context<Self>,
    ) -> impl IntoElement {
        let view = cx.entity().clone();

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .child({
                let view = cx.entity().clone();
                canvas(
                    move |bounds, _, cx| view.update(cx, |r, _| r.bounds = bounds),
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full()
            })
            .child(WebGorunumuOgesi::new(
                self.webview.clone(),
                view,
                window,
                cx,
            ))
    }
}

/// Bir web görünümü öğesi wry web görünümünü gösterebilir.
pub struct WebGorunumuOgesi {
    parent: Entity<WebGorunumu>,
    view: Rc<wry::WebView>,
}

impl WebGorunumuOgesi {
    /// wry WebGorunumu üzerinden yeni bir web görünümü öğesi oluşturur.
    pub fn new(
        view: Rc<wry::WebView>,
        parent: Entity<WebGorunumu>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self {
        Self { view, parent }
    }
}

impl IntoElement for WebGorunumuOgesi {
    type Element = WebGorunumuOgesi;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for WebGorunumuOgesi {
    type RequestLayoutState = ();
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let style = Style {
            size: Size::full(),
            flex_shrink: 1.,
            ..Default::default()
        };

        // If the parent view is no longer visible, we don't need to layout the webview
        let id = window.request_layout(style, [], cx);
        (id, ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        if !self.parent.read(cx).visible() {
            return None;
        }

        let _ = self.view.set_bounds(Rect {
            size: dpi::Size::Logical(LogicalSize {
                width: bounds.size.width.into(),
                height: bounds.size.height.into(),
            }),
            position: dpi::Position::Logical(dpi::LogicalPosition::new(
                bounds.origin.x.into(),
                bounds.origin.y.into(),
            )),
        });

        // Create a hitbox to handle mouse event
        Some(window.insert_hitbox(bounds, HitboxBehavior::Normal))
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        hitbox: &mut Self::PrepaintState,
        window: &mut Window,
        _: &mut App,
    ) {
        let bounds = hitbox.clone().map(|h| h.bounds).unwrap_or(bounds);
        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            let webview = self.view.clone();
            window.on_mouse_event(move |event: &MouseDownEvent, _, _, _| {
                if !bounds.contains(&event.position) {
                    // Click white space to blur the input focus
                    let _ = webview.focus_parent();
                }
            });
        });
    }
}
