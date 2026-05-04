use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Window, px,
};

use kavis_ui::{Boyutlandirilabilir, RenkAdi, h_flex, indigo_50, indigo_500, tag::Cip, v_flex};

use crate::section;

pub struct TagStory {
    focus_handle: FocusHandle,
}

impl super::Story for TagStory {
    fn title() -> &'static str {
        "Etiket"
    }

    fn description() -> &'static str {
        "A short item that can be used to categorize or label content."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl TagStory {
    pub(crate) fn new(_: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}
impl Focusable for TagStory {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for TagStory {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_3()
            .child(
                section("Cip (default)").child(
                    h_flex()
                        .gap_2()
                        .child(Cip::primary().child("Etiket"))
                        .child(Cip::secondary().child("İkincil"))
                        .child(Cip::danger().child("Tehlike"))
                        .child(Cip::success().child("Başarılı"))
                        .child(Cip::warning().child("Uyarı"))
                        .child(Cip::info().child("Bilgi"))
                        .child(Cip::custom(indigo_500(), indigo_50(), indigo_500()).child("Özel")),
                ),
            )
            .child(
                section("Cip (outline)").child(
                    h_flex()
                        .gap_2()
                        .child(Cip::primary().outline().child("Etiket"))
                        .child(Cip::secondary().outline().child("İkincil"))
                        .child(Cip::danger().outline().child("Tehlike"))
                        .child(Cip::success().outline().child("Başarılı"))
                        .child(Cip::warning().outline().child("Uyarı"))
                        .child(Cip::info().outline().child("Bilgi"))
                        .child(
                            Cip::custom(indigo_500(), indigo_500(), indigo_500())
                                .outline()
                                .child("Özel"),
                        ),
                ),
            )
            .child(
                section("Cip (small)").child(
                    h_flex()
                        .gap_2()
                        .child(Cip::primary().small().child("Etiket"))
                        .child(Cip::secondary().small().child("İkincil"))
                        .child(Cip::danger().small().child("Tehlike"))
                        .child(Cip::success().small().child("Başarılı"))
                        .child(Cip::warning().small().child("Uyarı"))
                        .child(Cip::info().small().child("Bilgi")),
                ),
            )
            .child(
                section("Cip (rounded full)").child(
                    h_flex()
                        .gap_2()
                        .child(Cip::primary().rounded_full().child("Etiket"))
                        .child(Cip::secondary().rounded_full().child("İkincil"))
                        .child(Cip::danger().rounded_full().child("Tehlike"))
                        .child(Cip::success().rounded_full().child("Başarılı"))
                        .child(Cip::warning().rounded_full().child("Uyarı"))
                        .child(Cip::info().rounded_full().child("Bilgi")),
                ),
            )
            .child(
                section("Cip (small with rounded full)").child(
                    h_flex()
                        .gap_2()
                        .child(Cip::primary().small().rounded_full().child("Etiket"))
                        .child(Cip::secondary().small().rounded_full().child("İkincil"))
                        .child(Cip::danger().small().rounded_full().child("Tehlike"))
                        .child(Cip::success().small().rounded_full().child("Başarılı"))
                        .child(Cip::warning().small().rounded_full().child("Uyarı"))
                        .child(Cip::info().small().rounded_full().child("Bilgi")),
                ),
            )
            .child(
                section("Cip (rounded 0px)").child(
                    h_flex()
                        .gap_2()
                        .child(Cip::primary().small().rounded(px(0.)).child("Etiket"))
                        .child(Cip::secondary().small().rounded(px(0.)).child("İkincil"))
                        .child(Cip::danger().small().rounded(px(0.)).child("Tehlike"))
                        .child(Cip::success().small().rounded(px(0.)).child("Başarılı"))
                        .child(Cip::warning().small().rounded(px(0.)).child("Uyarı"))
                        .child(Cip::info().small().rounded(px(0.)).child("Bilgi")),
                ),
            )
            .child(
                section("Color Tags").child(
                    v_flex().gap_4().child(
                        h_flex().gap_2().flex_wrap().children(
                            RenkAdi::all()
                                .into_iter()
                                .filter(|color| *color != RenkAdi::Gray)
                                .map(|color| Cip::color(color).child(color.to_string())),
                        ),
                    ),
                ),
            )
    }
}
