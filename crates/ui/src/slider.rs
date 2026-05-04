use std::ops::Range;

use crate::{EksenUzantisi, EtkinTema, OgeUzantisi, StilUzantisi, h_flex};
use gpui::{
    Along, App, AppContext as _, Axis, Background, Bounds, Context, Corners, DefiniteLength,
    DragMoveEvent, Empty, Entity, EntityId, EventEmitter, Hsla, InteractiveElement, IntoElement,
    IsZero, MouseButton, MouseDownEvent, ParentElement as _, Pixels, Point, Render, RenderOnce,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative,
};

#[derive(Clone)]
struct DragThumb((EntityId, bool));

impl Render for DragThumb {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

#[derive(Clone)]
struct DragSlider(EntityId);

impl Render for DragSlider {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

/// [`KaydiriciDurumu`] tarafından yayılan olaylar.
pub enum KaydiriciOlayi {
    Change(KaydiriciDegeri),
}

/// Kaydırıcı değeri tek değer ya da değer aralığı olabilir.
///
/// - f32 değerden üretilebilir; tek değer olarak ele alınır.
/// - (f32, f32) demetinden üretilebilir; değer aralığı olarak ele alınır.
///
/// Varsayılan değer `KaydiriciDegeri::Single(0.0)` değeridir.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KaydiriciDegeri {
    Single(f32),
    Range(f32, f32),
}

impl std::fmt::Display for KaydiriciDegeri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KaydiriciDegeri::Single(value) => write!(f, "{}", value),
            KaydiriciDegeri::Range(start, end) => write!(f, "{}..{}", start, end),
        }
    }
}

impl From<f32> for KaydiriciDegeri {
    fn from(value: f32) -> Self {
        KaydiriciDegeri::Single(value)
    }
}

impl From<(f32, f32)> for KaydiriciDegeri {
    fn from(value: (f32, f32)) -> Self {
        KaydiriciDegeri::Range(value.0, value.1)
    }
}

impl From<Range<f32>> for KaydiriciDegeri {
    fn from(value: Range<f32>) -> Self {
        KaydiriciDegeri::Range(value.start, value.end)
    }
}

impl Default for KaydiriciDegeri {
    fn default() -> Self {
        KaydiriciDegeri::Single(0.)
    }
}

impl KaydiriciDegeri {
    /// Değeri verilen aralığa sınırlar.
    pub fn clamp(self, min: f32, max: f32) -> Self {
        match self {
            KaydiriciDegeri::Single(value) => KaydiriciDegeri::Single(value.clamp(min, max)),
            KaydiriciDegeri::Range(start, end) => {
                KaydiriciDegeri::Range(start.clamp(min, max), end.clamp(min, max))
            }
        }
    }

    /// Değerin tek değer olup olmadığını kontrol eder.
    #[inline]
    pub fn is_single(&self) -> bool {
        matches!(self, KaydiriciDegeri::Single(_))
    }

    /// Değerin aralık olup olmadığını kontrol eder.
    #[inline]
    pub fn is_range(&self) -> bool {
        matches!(self, KaydiriciDegeri::Range(_, _))
    }

    /// Başlangıç değerini döndürür.
    pub fn start(&self) -> f32 {
        match self {
            KaydiriciDegeri::Single(value) => *value,
            KaydiriciDegeri::Range(start, _) => *start,
        }
    }

    /// Bitiş değerini döndürür.
    pub fn end(&self) -> f32 {
        match self {
            KaydiriciDegeri::Single(value) => *value,
            KaydiriciDegeri::Range(_, end) => *end,
        }
    }

    fn set_start(&mut self, value: f32) {
        if let KaydiriciDegeri::Range(_, end) = self {
            *self = KaydiriciDegeri::Range(value.min(*end), *end);
        } else {
            *self = KaydiriciDegeri::Single(value);
        }
    }

    fn set_end(&mut self, value: f32) {
        if let KaydiriciDegeri::Range(start, _) = self {
            *self = KaydiriciDegeri::Range(*start, value.max(*start));
        } else {
            *self = KaydiriciDegeri::Single(value);
        }
    }
}

/// Kaydırıcı ölçek modu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KaydiriciOlcegi {
    /// Doğrusal ölçekte değerler kaydırıcı aralığı boyunca eşit değişir.
    /// Bu varsayılan moddur.
    #[default]
    Linear,
    /// Logaritmik ölçekte değerler arasındaki mesafe üstel olarak artar.
    ///
    /// Bu, geniş değer aralığına sahip ve küçük değerlerde daha hassas
    /// değişimlerin daha önemli olduğu parametreler için kullanışlıdır. Yaygın örnekler:
    ///
    /// - Ses kontrolleri (insan işitme algısı logaritmiktir)
    /// - Frekans kontrolleri (müzik notaları logaritmik ölçeği izler)
    /// - Yakınlaştırma seviyeleri
    /// - Düşük değerlerde daha hassas kontrol istediğiniz herhangi bir parametre
    ///
    /// # Örneğin
    ///
    /// ```
    /// use kavis_ui::slider::{KaydiriciDurumu, KaydiriciOlcegi};
    ///
    /// let slider = KaydiriciDurumu::new()
    ///     .min(1.0)    // Must be > 0 for logarithmic scale
    ///     .max(1000.0)
    ///     .scale(KaydiriciOlcegi::Logarithmic);
    /// ```
    ///
    /// - Kaydırıcıyı yolun 1/3'üne taşımak yaklaşık 10 üretir
    /// - Kaydırıcıyı yolun 2/3'üne taşımak yaklaşık 100 üretir
    /// - Tam aralık 3 büyüklük mertebesini eşit şekilde kapsar
    Logarithmic,
}

impl KaydiriciOlcegi {
    #[inline]
    pub fn is_linear(&self) -> bool {
        matches!(self, KaydiriciOlcegi::Linear)
    }

    #[inline]
    pub fn is_logarithmic(&self) -> bool {
        matches!(self, KaydiriciOlcegi::Logarithmic)
    }
}

/// durum [`Kaydirici`].
pub struct KaydiriciDurumu {
    min: f32,
    max: f32,
    step: f32,
    value: KaydiriciDegeri,
    /// Tek değer modunda yalnızca `end` kullanılır; başlangıç her zaman 0.0 değeridir.
    percentage: Range<f32>,
    /// Çizimden sonra kaydırıcı sınırları.
    bounds: Bounds<Pixels>,
    scale: KaydiriciOlcegi,
}

impl KaydiriciDurumu {
    /// Yeni bir [`KaydiriciDurumu`] oluşturur.
    pub fn new() -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: KaydiriciDegeri::default(),
            percentage: (0.0..0.0),
            bounds: Bounds::default(),
            scale: KaydiriciOlcegi::default(),
        }
    }

    /// minimum kaydırıcı değeri ayarlar. Varsayılan: 0.0
    pub fn min(mut self, min: f32) -> Self {
        if self.scale.is_logarithmic() {
            assert!(
                min > 0.0,
                "`min` must be greater than 0 for KaydiriciOlcegi::Logarithmic"
            );
            assert!(
                min < self.max,
                "`min` must be less than `max` for Logarithmic scale"
            );
        }
        self.min = min;
        self.update_thumb_pos();
        self
    }

    /// en yüksek kaydırıcı değeri ayarlar. Varsayılan: 100.0
    pub fn max(mut self, max: f32) -> Self {
        if self.scale.is_logarithmic() {
            assert!(
                max > self.min,
                "`max` must be greater than `min` for Logarithmic scale"
            );
        }
        self.max = max;
        self.update_thumb_pos();
        self
    }

    /// step kaydırıcı değeri ayarlar. Varsayılan: 1.0
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// ölçek kaydırıcı, varsayılan: [`KaydiriciOlcegi::Linear`] ayarlar.
    pub fn scale(mut self, scale: KaydiriciOlcegi) -> Self {
        if scale.is_logarithmic() {
            assert!(
                self.min > 0.0,
                "`min` must be greater than 0 for Logarithmic scale"
            );
            assert!(
                self.max > self.min,
                "`max` must be greater than `min` for Logarithmic scale"
            );
        }
        self.scale = scale;
        self.update_thumb_pos();
        self
    }

    /// varsayılan kaydırıcı değeri ayarlar. Varsayılan: 0.0
    pub fn default_value(mut self, value: impl Into<KaydiriciDegeri>) -> Self {
        self.value = value.into();
        self.update_thumb_pos();
        self
    }

    /// kaydırıcı değeri ayarlar.
    pub fn set_value(
        &mut self,
        value: impl Into<KaydiriciDegeri>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.value = value.into();
        self.update_thumb_pos();
        cx.notify();
    }

    /// kaydırıcı değeri döndürür.
    pub fn value(&self) -> KaydiriciDegeri {
        self.value
    }

    /// bir değer arasında 0.0 ve 1.0 için bir değer arasında minimum ve en yüksek değer, dönüştürür.
    /// depending üzerinde chosen ölçek.
    fn percentage_to_value(&self, percentage: f32) -> f32 {
        match self.scale {
            KaydiriciOlcegi::Linear => self.min + (self.max - self.min) * percentage,
            KaydiriciOlcegi::Logarithmic => {
                // when percentage is 0, this simplifies to (max/min)^0 * min = 1 * min = min
                // when percentage is 1, this simplifies to (max/min)^1 * min = (max*min)/min = max
                // we clamp just to make sure we don't have issue with floating point precision
                let base = self.max / self.min;
                (base.powf(percentage) * self.min).clamp(self.min, self.max)
            }
        }
    }

    /// bir değer arasında minimum ve en yüksek değer için bir değer arasında 0.0 ve 1.0, dönüştürür.
    /// depending üzerinde chosen ölçek.
    fn value_to_percentage(&self, value: f32) -> f32 {
        match self.scale {
            KaydiriciOlcegi::Linear => {
                let range = self.max - self.min;
                if range <= 0.0 {
                    0.0
                } else {
                    (value - self.min) / range
                }
            }
            KaydiriciOlcegi::Logarithmic => {
                let base = self.max / self.min;
                (value / self.min).log(base).clamp(0.0, 1.0)
            }
        }
    }

    fn update_thumb_pos(&mut self) {
        match self.value {
            KaydiriciDegeri::Single(value) => {
                let percentage = self.value_to_percentage(value.clamp(self.min, self.max));
                self.percentage = 0.0..percentage;
            }
            KaydiriciDegeri::Range(start, end) => {
                let clamped_start = start.clamp(self.min, self.max);
                let clamped_end = end.clamp(self.min, self.max);
                self.percentage =
                    self.value_to_percentage(clamped_start)..self.value_to_percentage(clamped_end);
            }
        }
    }

    /// değer ile fare konum günceller.
    fn update_value_by_position(
        &mut self,
        axis: Axis,
        position: Point<Pixels>,
        is_start: bool,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let bounds = self.bounds;
        let step = self.step;

        let inner_pos = if axis.yatay_mi() {
            position.x - bounds.left()
        } else {
            bounds.bottom() - position.y
        };
        let total_size = bounds.size.along(axis);
        let percentage = inner_pos.clamp(px(0.), total_size) / total_size;

        let percentage = if is_start {
            percentage.clamp(0.0, self.percentage.end)
        } else {
            percentage.clamp(self.percentage.start, 1.0)
        };

        let value = self.percentage_to_value(percentage);
        let value = (value / step).round() * step;

        if is_start {
            self.percentage.start = percentage;
            self.value.set_start(value);
        } else {
            self.percentage.end = percentage;
            self.value.set_end(value);
        }
        cx.emit(KaydiriciOlayi::Change(self.value));
        cx.notify();
    }
}

impl EventEmitter<KaydiriciOlayi> for KaydiriciDurumu {}

/// Bir Kaydirici öğe.
#[derive(IntoElement)]
pub struct Kaydirici {
    state: Entity<KaydiriciDurumu>,
    axis: Axis,
    style: StyleRefinement,
    disabled: bool,
}

impl Kaydirici {
    /// Yeni bir [`Kaydirici`] öğe bind için [`KaydiriciDurumu`] oluşturur.
    pub fn new(state: &Entity<KaydiriciDurumu>) -> Self {
        Self {
            axis: Axis::Horizontal,
            state: state.clone(),
            style: StyleRefinement::default(),
            disabled: false,
        }
    }

    /// As bir yatay kaydırıcı.
    pub fn horizontal(mut self) -> Self {
        self.axis = Axis::Horizontal;
        self
    }

    /// As bir dikey kaydırıcı.
    pub fn vertical(mut self) -> Self {
        self.axis = Axis::Vertical;
        self
    }

    /// devre dışı durum kaydırıcı ayarlar. Varsayılan: false
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[allow(clippy::too_many_arguments)]
    fn render_thumb(
        &self,
        start: DefiniteLength,
        is_start: bool,
        bar_color: Background,
        thumb_color: Hsla,
        radius: Corners<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl gpui::IntoElement {
        let entity_id = self.state.entity_id();
        let axis = self.axis;
        let id = ("slider-thumb", is_start as u32);

        if self.disabled {
            return div().id(id);
        }

        div()
            .id(id)
            .absolute()
            .when(axis.yatay_mi(), |this| {
                this.top(px(-5.)).left(start).ml(-px(8.))
            })
            .when(axis.dikey_mi(), |this| {
                this.bottom(start).left(px(-5.)).mb(-px(8.))
            })
            .flex()
            .items_center()
            .justify_center()
            .flex_shrink_0()
            .corner_radii(radius)
            .bg(bar_color.opacity(0.5))
            .when(cx.theme().shadow, |this| this.shadow_md())
            .size_4()
            .p(px(1.))
            .child(
                div()
                    .flex_shrink_0()
                    .size_full()
                    .corner_radii(radius)
                    .bg(thumb_color),
            )
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .on_drag(DragThumb((entity_id, is_start)), |drag, _, _, cx| {
                cx.stop_propagation();
                cx.new(|_| drag.clone())
            })
            .on_drag_move(window.listener_for(
                &self.state,
                move |view, e: &DragMoveEvent<DragThumb>, window, cx| {
                    match e.drag(cx) {
                        DragThumb((id, is_start)) => {
                            if *id != entity_id {
                                return;
                            }

                            // set value by mouse position
                            view.update_value_by_position(
                                axis,
                                e.event.position,
                                *is_start,
                                window,
                                cx,
                            )
                        }
                    }
                },
            ))
    }
}

impl Styled for Kaydirici {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Kaydirici {
    fn render(self, window: &mut Window, cx: &mut gpui::App) -> impl IntoElement {
        let axis = self.axis;
        let entity_id = self.state.entity_id();
        let state = self.state.read(cx);
        let is_range = state.value().is_range();
        let percentage = state.percentage.clone();
        let bar_start = relative(percentage.start);
        let bar_end = relative(1. - percentage.end);
        let rem_size = window.rem_size();

        let bar_color = self
            .style
            .background
            .clone()
            .and_then(|bg| bg.color())
            .unwrap_or(cx.theme().slider_bar.into());
        let thumb_color = self
            .style
            .text
            .color
            .unwrap_or_else(|| cx.theme().slider_thumb);
        let corner_radii = self.style.corner_radii.clone();
        let default_radius = px(999.);
        let mut radius = Corners {
            top_left: corner_radii
                .top_left
                .map(|v| v.to_pixels(rem_size))
                .unwrap_or(default_radius),
            top_right: corner_radii
                .top_right
                .map(|v| v.to_pixels(rem_size))
                .unwrap_or(default_radius),
            bottom_left: corner_radii
                .bottom_left
                .map(|v| v.to_pixels(rem_size))
                .unwrap_or(default_radius),
            bottom_right: corner_radii
                .bottom_right
                .map(|v| v.to_pixels(rem_size))
                .unwrap_or(default_radius),
        };
        if cx.theme().radius.is_zero() {
            radius.top_left = px(0.);
            radius.top_right = px(0.);
            radius.bottom_left = px(0.);
            radius.bottom_right = px(0.);
        }

        div()
            .id(("slider", self.state.entity_id()))
            .flex()
            .flex_1()
            .items_center()
            .justify_center()
            .when(axis.dikey_mi(), |this| this.h(px(120.)))
            .when(axis.yatay_mi(), |this| this.w_full())
            .refine_style(&self.style)
            .bg(cx.theme().transparent)
            .text_color(cx.theme().foreground)
            .child(
                h_flex()
                    .id("slider-bar-container")
                    .when(!self.disabled, |this| {
                        this.on_mouse_down(
                            MouseButton::Left,
                            window.listener_for(
                                &self.state,
                                move |state, e: &MouseDownEvent, window, cx| {
                                    let mut is_start = false;
                                    if is_range {
                                        let bar_size = state.bounds.size.along(axis);
                                        let inner_pos = if axis.yatay_mi() {
                                            e.position.x - state.bounds.left()
                                        } else {
                                            state.bounds.bottom() - e.position.y
                                        };
                                        let center = ((percentage.end - percentage.start) / 2.0
                                            + percentage.start)
                                            * bar_size;
                                        is_start = inner_pos < center;
                                    }

                                    state.update_value_by_position(
                                        axis, e.position, is_start, window, cx,
                                    )
                                },
                            ),
                        )
                    })
                    .when(!self.disabled && !is_range, |this| {
                        this.on_drag(DragSlider(entity_id), |drag, _, _, cx| {
                            cx.stop_propagation();
                            cx.new(|_| drag.clone())
                        })
                        .on_drag_move(window.listener_for(
                            &self.state,
                            move |view, e: &DragMoveEvent<DragSlider>, window, cx| match e.drag(cx)
                            {
                                DragSlider(id) => {
                                    if *id != entity_id {
                                        return;
                                    }

                                    view.update_value_by_position(
                                        axis,
                                        e.event.position,
                                        false,
                                        window,
                                        cx,
                                    )
                                }
                            },
                        ))
                    })
                    .when(axis.yatay_mi(), |this| this.items_center().h_6().w_full())
                    .when(axis.dikey_mi(), |this| this.justify_center().w_6().h_full())
                    .flex_shrink_0()
                    .child(
                        div()
                            .id("slider-bar")
                            .relative()
                            .when(axis.yatay_mi(), |this| this.w_full().h_1p5())
                            .when(axis.dikey_mi(), |this| this.h_full().w_1p5())
                            .bg(bar_color.opacity(0.2))
                            .active(|this| this.bg(bar_color.opacity(0.4)))
                            .corner_radii(radius)
                            .child(
                                div()
                                    .absolute()
                                    .when(axis.yatay_mi(), |this| {
                                        this.h_full().left(bar_start).right(bar_end)
                                    })
                                    .when(axis.dikey_mi(), |this| {
                                        this.w_full().bottom(bar_start).top(bar_end)
                                    })
                                    .bg(bar_color)
                                    .when(!cx.theme().radius.is_zero(), |this| this.rounded_full()),
                            )
                            .when(is_range, |this| {
                                this.child(self.render_thumb(
                                    relative(percentage.start),
                                    true,
                                    bar_color,
                                    thumb_color,
                                    radius,
                                    window,
                                    cx,
                                ))
                            })
                            .child(self.render_thumb(
                                relative(percentage.end),
                                false,
                                bar_color,
                                thumb_color,
                                radius,
                                window,
                                cx,
                            ))
                            .on_prepaint({
                                let state = self.state.clone();
                                move |bounds, _, cx| state.update(cx, |r, _| r.bounds = bounds)
                            }),
                    ),
            )
    }
}
