use crate::{BilesenBoyutu, Boyutlandirilabilir, EtkinTema, StilUzantisi};
use crate::ham_gpui::prelude::FluentBuilder as _;
use crate::ham_gpui::{
    Animation, AnimationExt as _, AnyElement, App, ElementId, Hsla, InteractiveElement as _,
    IntoElement, ParentElement, Pixels, RenderOnce, StyleRefinement, Styled, Window, canvas,
    ease_in_out, px, relative,
};
use crate::ham_gpui::{Bounds, div};
use instant::Duration;
use std::f32::consts::TAU;

use super::IlerlemeDurumu;
use crate::plot::shape::{Arc, ArcData};

/// Bir circular ilerleme gösterge öğe.
#[derive(IntoElement)]
pub struct DaireselIlerleme {
    id: ElementId,
    style: StyleRefinement,
    color: Option<Hsla>,
    value: f32,
    size: BilesenBoyutu,
    children: Vec<AnyElement>,
    loading: bool,
}

impl DaireselIlerleme {
    /// Yeni bir circular ilerleme gösterge oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: Default::default(),
            color: None,
            style: StyleRefinement::default(),
            size: BilesenBoyutu::default(),
            children: Vec::new(),
            loading: false,
        }
    }

    /// Belirsiz yükleme animasyonunu etkinleştirir.
    ///
    /// `loading` true olduğunda `değer` yok sayılır ve sonsuz bir
    /// dönen yay animasyonu gösterilir.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// renk ilerleme circle ayarlar.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// İlerleme dairesinin yüzde değerini ayarlar.
    ///
    /// değer olmalıdır arasında 0.0 ve 100.0.
    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0., 100.);
        self
    }

    /// Yayı tuvale çizer. `start_value` ve `end_value` 0.0-100.0 yüzde aralığındadır.
    /// `end_value <= 0` olduğunda ilerleme yayı atlanır.
    fn render_circle(start_value: f32, end_value: f32, color: Hsla) -> impl IntoElement {
        struct PrepaintState {
            start_value: f32,
            end_value: f32,
            actual_inner_radius: f32,
            actual_outer_radius: f32,
            bounds: Bounds<Pixels>,
        }

        canvas(
            move |bounds: Bounds<Pixels>, _window: &mut Window, _cx: &mut App| {
                let stroke_width = (bounds.size.width * 0.15).min(px(5.));
                let actual_size = bounds.size.width.min(bounds.size.height);
                let actual_radius = (actual_size.as_f32() - stroke_width.as_f32()) / 2.;
                PrepaintState {
                    start_value,
                    end_value,
                    actual_inner_radius: actual_radius - stroke_width.as_f32() / 2.,
                    actual_outer_radius: actual_radius + stroke_width.as_f32() / 2.,
                    bounds,
                }
            },
            move |_bounds, prepaint, window: &mut Window, _cx: &mut App| {
                let arc = Arc::new()
                    .inner_radius(prepaint.actual_inner_radius)
                    .outer_radius(prepaint.actual_outer_radius);

                arc.paint(
                    &ArcData {
                        data: &(),
                        index: 0,
                        value: 100.,
                        start_angle: 0.,
                        end_angle: TAU,
                        pad_angle: 0.,
                    },
                    color.opacity(0.2),
                    None,
                    None,
                    &prepaint.bounds,
                    window,
                );

                if prepaint.end_value > 0. {
                    let start_angle = (prepaint.start_value / 100.) * TAU;
                    let end_angle = (prepaint.end_value / 100.) * TAU;
                    arc.paint(
                        &ArcData {
                            data: &(),
                            index: 1,
                            value: prepaint.end_value,
                            start_angle,
                            end_angle,
                            pad_angle: 0.,
                        },
                        color,
                        None,
                        None,
                        &prepaint.bounds,
                        window,
                    );
                }
            },
        )
        .absolute()
        .size_full()
    }
}

impl Styled for DaireselIlerleme {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Boyutlandirilabilir for DaireselIlerleme {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl ParentElement for DaireselIlerleme {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DaireselIlerleme {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = self.value;
        let loading = self.loading;
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| IlerlemeDurumu::new(value));
        let prev_target = state.read(cx).target();
        let has_changed = prev_target != value;

        let color = self.color.unwrap_or(cx.theme().progress_bar);

        div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .justify_center()
            .line_height(relative(1.))
            .map(|this| match self.size {
                BilesenBoyutu::CokKucuk => this.size_2(),
                BilesenBoyutu::Kucuk => this.size_3(),
                BilesenBoyutu::Orta => this.size_4(),
                BilesenBoyutu::Buyuk => this.size_5(),
                BilesenBoyutu::Ozel(s) => this.size(s * 0.75),
            })
            .refine_style(&self.style)
            .children(self.children)
            .map(|this| {
                if has_changed {
                    let from = prev_target;
                    state.read(cx).set_target(value);

                    let duration = Duration::from_secs_f64(0.15);
                    cx.spawn({
                        let state = state.clone();
                        async move |cx| {
                            cx.background_executor().timer(duration).await;
                            _ = state.update(cx, |this, _| {
                                this.value = this.target();
                            });
                        }
                    })
                    .detach();

                    this.with_animation(
                        format!("progress-circle-{}", from),
                        Animation::new(duration),
                        move |this, delta| {
                            let v = from + (value - from) * delta;
                            this.child(Self::render_circle(0., v, color))
                        },
                    )
                    .into_any_element()
                } else if loading {
                    this.with_animation(
                        "progress-circle-loading",
                        Animation::new(Duration::from_secs(1)).repeat(),
                        move |this, delta| {
                            let end = ease_in_out(delta) * 100.;
                            let start = ease_in_out(((delta - 0.5) / 0.5).clamp(0., 1.)) * 100.;
                            this.child(Self::render_circle(start, end, color))
                        },
                    )
                    .into_any_element()
                } else {
                    this.child(Self::render_circle(0., value, color))
                        .into_any_element()
                }
            })
    }
}
