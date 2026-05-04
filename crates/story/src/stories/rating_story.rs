use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, Styled, Window,
};
use kavis_ui::{
    BilesenBoyutu, Boyutlandirilabilir as _, EtkinTema, Secilebilir as _, SimgeAdi,
    button::{Dugme, DugmeGrubu},
    h_flex,
    rating::Puanlama,
    v_flex,
};

use crate::section;

pub struct RatingStory {
    focus_handle: kavis_ui::ham_gpui::FocusHandle,
    size: BilesenBoyutu,
    value: usize,
}

impl super::Story for RatingStory {
    fn title() -> &'static str {
        "Puanlama"
    }

    fn description() -> &'static str {
        "A simple interactive star rating component."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl RatingStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            size: BilesenBoyutu::default(),
            value: 3,
        }
    }
}

impl Focusable for RatingStory {
    fn focus_handle(&self, _: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

pub fn init(_cx: &mut App) {
    // No global init required for RatingStory
}

impl Render for RatingStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_3()
            .child(
                h_flex().w_full().gap_3().child(
                    DugmeGrubu::new("toggle-size")
                        .outline()
                        .compact()
                        .child(
                            Dugme::new("xsmall")
                                .label("XSmall")
                                .selected(self.size == BilesenBoyutu::CokKucuk),
                        )
                        .child(
                            Dugme::new("small")
                                .label("Small")
                                .selected(self.size == BilesenBoyutu::Kucuk),
                        )
                        .child(
                            Dugme::new("medium")
                                .label("Medium")
                                .selected(self.size == BilesenBoyutu::Orta),
                        )
                        .child(
                            Dugme::new("large")
                                .label("Large")
                                .selected(self.size == BilesenBoyutu::Buyuk),
                        )
                        .on_click(cx.listener(|this, selecteds: &Vec<usize>, _, cx| {
                            let size = match selecteds[0] {
                                0 => BilesenBoyutu::CokKucuk,
                                1 => BilesenBoyutu::Kucuk,
                                2 => BilesenBoyutu::Orta,
                                3 => BilesenBoyutu::Buyuk,
                                _ => unreachable!(),
                            };
                            this.size = size;
                            cx.notify();
                        })),
                ),
            )
            .child(
                section("Basic Puanlama").max_w_md().child(
                    v_flex()
                        .w_full()
                        .gap_3()
                        .justify_center()
                        .items_center()
                        .child(
                            Puanlama::new("rating-1")
                                .with_size(self.size)
                                .value(self.value)
                                .max(5)
                                .on_click(cx.listener(|this, value: &usize, _, cx| {
                                    this.value = *value;
                                    cx.notify();
                                })),
                        )
                        .child(
                            h_flex()
                                .gap_x_2()
                                .child(
                                    Dugme::new("r-dec")
                                        .small()
                                        .outline()
                                        .icon(SimgeAdi::Minus)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            let v = this.value.saturating_sub(1);
                                            this.value = v;
                                            cx.notify();
                                        })),
                                )
                                .child(
                                    Dugme::new("r-inc")
                                        .small()
                                        .outline()
                                        .icon(SimgeAdi::Plus)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            let v = (this.value + 1).min(5);
                                            this.value = v;
                                            cx.notify();
                                        })),
                                ),
                        ),
                ),
            )
            .child(
                section("Disabled").max_w_md().child(
                    Puanlama::new("rating-2")
                        .with_size(self.size)
                        .value(2)
                        .color(cx.theme().green)
                        .max(5)
                        .disabled(true),
                ),
            )
            .child(
                section("Custom Color").max_w_md().child(
                    Puanlama::new("rating-3")
                        .large()
                        .value(self.value)
                        .color(cx.theme().green)
                        .max(5),
                ),
            )
    }
}
