use std::{cell::Cell, rc::Rc, time::Duration};

use gpui::{
    Action, AnyElement, AnyView, App, AppContext, Bounds, Context, Display, Element, ElementId,
    GlobalElementId, Half, InspectorElementId, IntoElement, LayoutId, ParentElement, Pixels, Point,
    Position, Render, SharedString, Size, StatefulInteractiveElement, Style, StyleRefinement,
    Styled, Task, Window, deferred, div, point, prelude::FluentBuilder, px,
};

use crate::{
    EtkinTema, StyledExt,
    animation::{Transition, ease_in_out_cubic, ease_out_cubic},
    h_flex,
    kbd::KlavyeTusu,
    root::KokGorunum,
    text::Text,
};

pub(crate) fn init(_cx: &mut App) {
    // No app-level init needed — AracIpucuKatmani is per-window via KokGorunum.
}

// ── AracIpucu view (unchanged API) ────────────────────────────────────────────

enum TooltipContext {
    Text(Text),
    Element(Box<dyn Fn(&mut Window, &mut App) -> AnyElement>),
}

/// Metin veya özel içerik gösterebilen bir AracIpucu öğesi,
/// ile isteğe bağlı key binding bilgi.
pub struct AracIpucu {
    style: StyleRefinement,
    content: TooltipContext,
    key_binding: Option<KlavyeTusu>,
    action: Option<(Box<dyn Action>, Option<SharedString>)>,
}

impl AracIpucu {
    /// Bir AracIpucu ile bir metin içerik. oluşturur.
    pub fn new(text: impl Into<Text>) -> Self {
        Self {
            style: StyleRefinement::default(),
            content: TooltipContext::Text(text.into()),
            key_binding: None,
            action: None,
        }
    }

    /// Bir AracIpucu ile özel öğe. oluşturur.
    pub fn element<E, F>(builder: F) -> Self
    where
        E: IntoElement,
        F: Fn(&mut Window, &mut App) -> E + 'static,
    {
        Self {
            style: StyleRefinement::default(),
            key_binding: None,
            action: None,
            content: TooltipContext::Element(Box::new(move |window, cx| {
                builder(window, cx).into_any_element()
            })),
        }
    }

    /// eylem göstermek için key binding bilgi için araç ipucu ise onu exists ayarlar.
    pub fn action(mut self, action: &dyn Action, context: Option<&str>) -> Self {
        self.action = Some((action.boxed_clone(), context.map(SharedString::new)));
        self
    }

    /// KeyBinding bilgi için araç ipucu ayarlar.
    pub fn key_binding(mut self, key_binding: Option<KlavyeTusu>) -> Self {
        self.key_binding = key_binding;
        self
    }

    /// Oluşturur araç ipucu ve döndürür onu olarak bir `AnyView`.
    pub fn build(self, _: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|_| self).into()
    }
}

impl FluentBuilder for AracIpucu {}
impl Styled for AracIpucu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}
impl Render for AracIpucu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let key_binding = if let Some(key_binding) = &self.key_binding {
            Some(key_binding.clone())
        } else {
            if let Some((action, context)) = &self.action {
                KlavyeTusu::binding_for_action(
                    action.as_ref(),
                    context.as_ref().map(|s| s.as_ref()),
                    window,
                )
            } else {
                None
            }
        };

        div().child(
            // Wrap in a child, to ensure the left margin is applied to the tooltip
            h_flex()
                .font_family(cx.theme().font_family.clone())
                .m_3()
                .bg(cx.theme().popover)
                .text_color(cx.theme().popover_foreground)
                .bg(cx.theme().popover)
                .border_1()
                .border_color(cx.theme().border)
                .shadow_md()
                .rounded(px(6.))
                .justify_between()
                .py_0p5()
                .px_2()
                .text_sm()
                .gap_3()
                .refine_style(&self.style)
                .map(|this| {
                    this.child(div().map(|this| match self.content {
                        TooltipContext::Text(ref text) => this.child(text.clone()),
                        TooltipContext::Element(ref builder) => this.child(builder(window, cx)),
                    }))
                })
                .when_some(key_binding, |this, kbd| {
                    this.child(
                        div()
                            .text_xs()
                            .flex_shrink_0()
                            .text_color(cx.theme().muted_foreground)
                            .child(kbd.appearance(false)),
                    )
                }),
        )
    }
}

// ── Managed tooltip system ──────────────────────────────────────────────────

/// Bekleme süresi: Bu süre içinde bir araç ipucu gizlendiyse sonraki gösterimde gecikme atlanır.
const GRACE_PERIOD: Duration = Duration::from_millis(300);
/// Şu anda etkin araç ipucu yokken araç ipucunu göstermeden önce beklenecek gecikme.
const SHOW_DELAY: Duration = Duration::from_millis(500);
/// süre slide-down enter animasyon.
const ENTER_DURATION: Duration = Duration::from_millis(150);
/// süre konum-slide animasyon olduğunda switching tooltips.
const SLIDE_DURATION: Duration = Duration::from_millis(200);
const TOOLTIP_WINDOW_MARGIN: Pixels = px(4.);

#[derive(Clone, Copy, Debug, PartialEq)]
enum TooltipPlacement {
    Above,
    Below,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TooltipOverlayPosition {
    bounds: Bounds<Pixels>,
    placement: TooltipPlacement,
}

fn tooltip_overlay_position(
    trigger_bounds: Bounds<Pixels>,
    tooltip_size: Size<Pixels>,
    viewport_size: Size<Pixels>,
    margin: Pixels,
) -> TooltipOverlayPosition {
    let centered_x = trigger_bounds.center().x - tooltip_size.width.half();
    let above_bounds = Bounds::new(
        point(centered_x, trigger_bounds.top() - tooltip_size.height),
        tooltip_size,
    );
    let below_bounds = Bounds::new(point(centered_x, trigger_bounds.bottom()), tooltip_size);

    let bottom_limit = (viewport_size.height - margin).max(margin);
    let available_above = (trigger_bounds.top() - margin).max(px(0.));
    let available_below = (bottom_limit - trigger_bounds.bottom()).max(px(0.));

    let (bounds, placement) = if above_bounds.top() >= margin {
        (above_bounds, TooltipPlacement::Above)
    } else if below_bounds.bottom() <= bottom_limit {
        (below_bounds, TooltipPlacement::Below)
    } else if available_below >= available_above {
        (below_bounds, TooltipPlacement::Below)
    } else {
        (above_bounds, TooltipPlacement::Above)
    };

    TooltipOverlayPosition {
        bounds: clamp_tooltip_bounds(bounds, viewport_size, margin),
        placement,
    }
}

fn clamp_tooltip_bounds(
    mut bounds: Bounds<Pixels>,
    viewport_size: Size<Pixels>,
    margin: Pixels,
) -> Bounds<Pixels> {
    let right_limit = (viewport_size.width - margin).max(margin);
    let bottom_limit = (viewport_size.height - margin).max(margin);

    if bounds.right() > right_limit {
        bounds.origin.x -= bounds.right() - right_limit;
    }
    if bounds.left() < margin {
        bounds.origin.x = margin;
    }

    if bounds.bottom() > bottom_limit {
        bounds.origin.y -= bounds.bottom() - bottom_limit;
    }
    if bounds.top() < margin {
        bounds.origin.y = margin;
    }

    bounds
}

struct TooltipOverlayPositioner {
    trigger_bounds: Bounds<Pixels>,
    children: Vec<AnyElement>,
}

struct TooltipOverlayPositionerState {
    child_layout_ids: Vec<LayoutId>,
}

fn tooltip_overlay_positioner(trigger_bounds: Bounds<Pixels>) -> TooltipOverlayPositioner {
    TooltipOverlayPositioner {
        trigger_bounds,
        children: Vec::new(),
    }
}

impl ParentElement for TooltipOverlayPositioner {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Element for TooltipOverlayPositioner {
    type RequestLayoutState = TooltipOverlayPositionerState;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let child_layout_ids = self
            .children
            .iter_mut()
            .map(|child| child.request_layout(window, cx))
            .collect::<Vec<_>>();

        let layout_id = window.request_layout(
            Style {
                position: Position::Absolute,
                display: Display::Flex,
                ..Style::default()
            },
            child_layout_ids.iter().copied(),
            cx,
        );

        (
            layout_id,
            TooltipOverlayPositionerState { child_layout_ids },
        )
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if request_layout.child_layout_ids.is_empty() {
            return;
        }

        let mut child_min: Point<Pixels> = point(Pixels::MAX, Pixels::MAX);
        let mut child_max = Point::default();
        for child_layout_id in &request_layout.child_layout_ids {
            let child_bounds = window.layout_bounds(*child_layout_id);
            child_min = child_min.min(&child_bounds.origin);
            child_max = child_max.max(&child_bounds.bottom_right());
        }

        let tooltip_size: Size<Pixels> = (child_max - child_min).into();
        let client_inset = window.client_inset().unwrap_or(px(0.));
        let tooltip_position = tooltip_overlay_position(
            self.trigger_bounds,
            tooltip_size,
            window.viewport_size(),
            TOOLTIP_WINDOW_MARGIN + client_inset,
        );

        let offset = tooltip_position.bounds.origin - bounds.origin;
        let offset = point(offset.x.round(), offset.y.round());

        window.with_element_offset(offset, |window| {
            for child in &mut self.children {
                child.prepaint(window, cx);
            }
        });
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        for child in &mut self.children {
            child.paint(window, cx);
        }
    }
}

impl IntoElement for TooltipOverlayPositioner {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// içerik için bir yönetilir araç ipucu.
#[derive(Clone)]
pub(crate) struct AracIpucuIcerigi {
    pub build: Rc<dyn Fn(&mut Window, &mut App) -> AnyView>,
    pub trigger_bounds: Bounds<Pixels>,
}

/// Manages araç ipucu lifecycle: gecikme, grace period, animations, ve çizim.
///
/// Bir tek örnek lives içinde [`KokGorunum`] başına pencere. bileşenler kaydeder üzerine gelme
/// [`ManagedTooltipExt::managed_tooltip`] üzerinden bu kaplamaya çağrı yapar.
pub struct AracIpucuKatmani {
    content: Option<AracIpucuIcerigi>,
    prev_trigger_bounds: Option<Bounds<Pixels>>,
    epoch: usize,
    had_recent_tooltip: bool,
    animation_epoch: usize,
    is_switching: bool,

    _show_task: Option<Task<()>>,
    _hide_task: Option<Task<()>>,
}

impl AracIpucuKatmani {
    pub fn new() -> Self {
        Self {
            content: None,
            prev_trigger_bounds: None,
            epoch: 0,
            had_recent_tooltip: false,
            animation_epoch: 0,
            is_switching: false,
            _show_task: None,
            _hide_task: None,
        }
    }

    fn next_epoch(&mut self) -> usize {
        self.epoch += 1;
        self.epoch
    }

    /// Araç ipucu gösterme isteği gönderir. Başka bir araç ipucu etkinse veya yakın zamanda
    /// gizlenir, shows hemen ile bir slide animasyon. Otherwise başlatır bir gecikme.
    pub(crate) fn request_show(
        &mut self,
        content: AracIpucuIcerigi,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Cancel any pending hide
        self._hide_task = None;

        let was_visible = self.content.is_some();
        let in_grace = self.had_recent_tooltip;

        if was_visible || in_grace {
            // Anahtar: show immediately with slide animation
            self.prev_trigger_bounds = self.content.as_ref().map(|c| c.trigger_bounds);
            self.content = Some(content);
            self._show_task = None;
            self.is_switching = was_visible;
            self.animation_epoch += 1;
            cx.notify();
        } else {
            // New: delay then show with slideDown
            let epoch = self.next_epoch();
            let content = content.clone();
            self._show_task = Some(cx.spawn_in(window, async move |this, cx| {
                cx.background_executor().timer(SHOW_DELAY).await;
                let _ = this.update_in(cx, |this, _, cx| {
                    if this.epoch != epoch {
                        return;
                    }

                    this.content = Some(content);
                    this.prev_trigger_bounds = None;
                    this.is_switching = false;
                    this.animation_epoch += 1;
                    cx.notify();
                });
            }));
        }
    }

    /// Geçerli araç ipucunu gizleme isteği gönderir. Kısa bir bekleme süresi başlatır; böylece
    /// taşıma için başka bir araç ipucu-bearing öğe feels instant.
    pub(crate) fn request_hide(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Cancel any pending show
        self._show_task = None;

        if self.content.is_none() {
            return;
        }

        let epoch = self.next_epoch();
        self.had_recent_tooltip = true;

        self._hide_task = Some(cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(GRACE_PERIOD).await;
            let _ = this.update_in(cx, |this, _, cx| {
                if this.epoch != epoch {
                    return;
                }
                this.content = None;
                this.prev_trigger_bounds = None;
                this.had_recent_tooltip = false;
                cx.notify();
            });
        }));
    }
}

impl Render for AracIpucuKatmani {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(content) = self.content.as_ref() else {
            return div().into_any_element();
        };

        let content_view = (content.build)(window, cx);
        let trigger_bounds = content.trigger_bounds;
        let animation_epoch = self.animation_epoch;
        let is_switching = self.is_switching;
        let prev_trigger_bounds = self.prev_trigger_bounds;

        deferred(
            tooltip_overlay_positioner(trigger_bounds).child(div().child(content_view).map(|el| {
                if is_switching {
                    let Some(prev_bounds) = prev_trigger_bounds else {
                        return el.into_any_element();
                    };

                    let is_same_y =
                        (trigger_bounds.origin.y - prev_bounds.origin.y).abs() < px(10.);
                    if !is_same_y {
                        // If the new trigger is at a different Y level, don't slide horizontally
                        // to avoid weird diagonal movement. (We could consider sliding vertically
                        // in this case, but it might be less visually clear.)
                        return el.into_any_element();
                    }

                    let dx = trigger_bounds.center().x - prev_bounds.center().x;

                    Transition::new(SLIDE_DURATION)
                        .ease(ease_in_out_cubic)
                        .slide_x(-dx, px(0.))
                        .apply(
                            el,
                            ElementId::NamedInteger("tooltip-slide".into(), animation_epoch as u64),
                        )
                        .into_any_element()
                } else {
                    // New tooltip: slideDown + fadeIn
                    Transition::new(ENTER_DURATION)
                        .ease(ease_out_cubic)
                        .slide_y(px(4.), px(0.))
                        .fade(0.0, 1.0)
                        .apply(
                            el,
                            ElementId::NamedInteger("tooltip-enter".into(), animation_epoch as u64),
                        )
                        .into_any_element()
                }
            })),
        )
        .with_priority(2)
        .into_any_element()
    }
}

// ── Extension trait for managed tooltips ─────────────────────────────────────

// ── Shared tooltip state for components ─────────────────────────────────────

/// Dugme, Anahtar, OnayKutusu, Radyo vb. bileşenlerin paylaştığı araç ipucu durumu.
/// olabilir embed almak için `.araç ipucu()` destek ile minimal boilerplate.
#[derive(Default)]
pub(crate) struct ComponentTooltip {
    pub text: Option<(
        SharedString,
        Option<(Rc<Box<dyn Action>>, Option<SharedString>)>,
    )>,
    pub builder: Option<Rc<dyn Fn(&mut Window, &mut App) -> AnyView>>,
}

impl ComponentTooltip {
    /// bu araç ipucu için bir `Stateful<Div>` (veya herhangi bir `ManagedTooltipExt` öğe). uygular.
    pub fn apply<E: ManagedTooltipExt>(self, el: E) -> E {
        if let Some(builder) = self.builder {
            el.managed_tooltip(move |window, cx| builder(window, cx))
        } else if let Some((text, action)) = self.text {
            el.managed_tooltip(move |window, cx| {
                AracIpucu::new(text.clone())
                    .when_some(action.clone(), |this, (action, context)| {
                        this.action(
                            action.boxed_clone().as_ref(),
                            context.as_ref().map(|c| c.as_ref()),
                        )
                    })
                    .build(window, cx)
            })
        } else {
            el
        }
    }
}

// ── Internal managed tooltip trait ──────────────────────────────────────────

pub(crate) trait ManagedTooltipExt:
    StatefulInteractiveElement + crate::ElementExt + Sized
{
    fn managed_tooltip(
        self,
        build_tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        let build_tooltip = Rc::new(build_tooltip);
        let trigger_bounds_cell: Rc<Cell<Bounds<Pixels>>> = Rc::new(Cell::new(Bounds::default()));
        let bounds_writer = trigger_bounds_cell.clone();

        self.on_prepaint(move |bounds, _, _| {
            bounds_writer.set(bounds);
        })
        .on_hover({
            let trigger_bounds_cell = trigger_bounds_cell.clone();
            let build_tooltip = build_tooltip.clone();
            move |hovered, window, cx| {
                if let Some(overlay) = KokGorunum::tooltip_overlay(window, cx) {
                    if *hovered {
                        let bounds = trigger_bounds_cell.get();
                        overlay.update(cx, |o: &mut AracIpucuKatmani, cx| {
                            o.request_show(
                                AracIpucuIcerigi {
                                    build: build_tooltip.clone(),
                                    trigger_bounds: bounds,
                                },
                                window,
                                cx,
                            );
                        });
                    } else {
                        overlay.update(cx, |o: &mut AracIpucuKatmani, cx| {
                            o.request_hide(window, cx);
                        });
                    }
                }
            }
        })
    }
}

impl<E: StatefulInteractiveElement + crate::ElementExt> ManagedTooltipExt for E {}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::size;

    fn test_bounds(x: f32, y: f32, width: f32, height: f32) -> Bounds<Pixels> {
        Bounds::new(point(px(x), px(y)), size(px(width), px(height)))
    }

    fn test_size(width: f32, height: f32) -> Size<Pixels> {
        size(px(width), px(height))
    }

    #[test]
    fn tooltip_overlay_position_prefers_above_when_space_allows() {
        let trigger_bounds = test_bounds(100., 80., 80., 24.);
        let position = tooltip_overlay_position(
            trigger_bounds,
            test_size(120., 30.),
            test_size(300., 200.),
            TOOLTIP_WINDOW_MARGIN,
        );

        assert_eq!(position.placement, TooltipPlacement::Above);
        assert_eq!(position.bounds.origin.x, px(80.));
        assert_eq!(position.bounds.origin.y, px(50.));
        assert_eq!(position.bounds.bottom(), trigger_bounds.top());
    }

    #[test]
    fn tooltip_overlay_position_flips_below_near_top_edge() {
        let trigger_bounds = test_bounds(24., 4., 120., 32.);
        let position = tooltip_overlay_position(
            trigger_bounds,
            test_size(240., 32.),
            test_size(520., 260.),
            TOOLTIP_WINDOW_MARGIN,
        );

        assert_eq!(position.placement, TooltipPlacement::Below);
        assert_eq!(position.bounds.top(), trigger_bounds.bottom());
        assert!(position.bounds.top() >= trigger_bounds.bottom());
    }

    #[test]
    fn tooltip_overlay_position_clamps_horizontal_edges() {
        let trigger_bounds = test_bounds(4., 80., 24., 24.);
        let position = tooltip_overlay_position(
            trigger_bounds,
            test_size(120., 30.),
            test_size(300., 200.),
            TOOLTIP_WINDOW_MARGIN,
        );

        assert_eq!(position.placement, TooltipPlacement::Above);
        assert_eq!(position.bounds.left(), TOOLTIP_WINDOW_MARGIN);
    }

    #[test]
    fn tooltip_overlay_position_uses_larger_side_when_neither_side_fits() {
        let trigger_bounds = test_bounds(120., 20., 40., 20.);
        let position = tooltip_overlay_position(
            trigger_bounds,
            test_size(160., 120.),
            test_size(300., 100.),
            TOOLTIP_WINDOW_MARGIN,
        );

        assert_eq!(position.placement, TooltipPlacement::Below);
        assert_eq!(position.bounds.top(), TOOLTIP_WINDOW_MARGIN);
        assert_eq!(position.bounds.left(), px(60.));
    }
}
