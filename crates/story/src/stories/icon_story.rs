use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Window,
};
use kavis_ui::{
    Boyutlandirilabilir, EtkinTema as _, Simge, SimgeAdi,
    button::{Dugme, DugmeVaryanti, DugmeVaryantlari},
    dock::PanelDenetimi,
    h_flex, neutral_500, v_flex,
};

use crate::section;

pub struct IconStory {
    focus_handle: kavis_ui::ham_gpui::FocusHandle,
}

impl IconStory {
    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl super::Story for IconStory {
    fn title() -> &'static str {
        "Simge"
    }

    fn description() -> &'static str {
        "SVG Icons based on Lucide.dev"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn zoomable() -> Option<PanelDenetimi> {
        None
    }
}

impl Focusable for IconStory {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for IconStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(
                section("Simge")
                    .text_lg()
                    .child(SimgeAdi::Info)
                    .child(SimgeAdi::Map)
                    .child(SimgeAdi::Bot)
                    .child(SimgeAdi::Github)
                    .child(SimgeAdi::Calendar)
                    .child(SimgeAdi::Globe)
                    .child(SimgeAdi::Heart),
            )
            .child(
                section("Color Simge")
                    .child(
                        Simge::new(SimgeAdi::Maximize)
                            .size_6()
                            .text_color(cx.theme().green),
                    )
                    .child(
                        Simge::new(SimgeAdi::Minimize)
                            .size_6()
                            .text_color(cx.theme().red),
                    ),
            )
            .child(
                section("Simge Dugme").child(
                    h_flex()
                        .gap_4()
                        .child(
                            Dugme::new("like1")
                                .icon(
                                    Simge::new(SimgeAdi::Heart)
                                        .text_color(neutral_500())
                                        .size_6(),
                                )
                                .with_variant(DugmeVaryanti::Ghost),
                        )
                        .child(
                            Dugme::new("like2")
                                .icon(
                                    Simge::new(SimgeAdi::HeartOff)
                                        .text_color(cx.theme().red)
                                        .size_6(),
                                )
                                .with_variant(DugmeVaryanti::Ghost),
                        )
                        .child(
                            Dugme::new("like3")
                                .icon(
                                    Simge::new(SimgeAdi::Heart)
                                        .text_color(cx.theme().green)
                                        .size_6(),
                                )
                                .with_variant(DugmeVaryanti::Ghost),
                        ),
                ),
            )
            .child(
                section("Dugme with size").child(
                    Dugme::new("button-with-size")
                        .outline()
                        .size_5()
                        .small()
                        .px_0()
                        .label("10"),
                ),
            )
    }
}
