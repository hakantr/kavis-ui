use crate::section;
use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, Styled,
    Window, px,
};
use kavis_ui::{EtkinTema, divider::Ayirici, h_flex, label::Etiket, v_flex};

const DESCRIPTION: &str = "Kavis UI, GPUI ile güçlü çapraz platform masaüstü uygulamaları geliştirmek için Rust GUI bileşenleri sunar.";

pub struct AyiriciStory {
    focus_handle: kavis_ui::ham_gpui::FocusHandle,
}

impl super::Story for AyiriciStory {
    fn title() -> &'static str {
        "Ayirici"
    }

    fn description() -> &'static str {
        "Dikey veya yatay kullanılabilen ayırıcı."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl AyiriciStory {
    pub fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
        })
    }
}

impl Focusable for AyiriciStory {
    fn focus_handle(&self, _: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AyiriciStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .child(
                section("Yatay Ayırıcılar").child(
                    v_flex()
                        .gap_4()
                        .w_full()
                        .mt_4()
                        .child(Ayirici::horizontal())
                        .child(Ayirici::horizontal().label("Etiketli"))
                        .child(Ayirici::horizontal_dashed())
                        .child(Ayirici::horizontal_dashed().label("Kesikli ve Etiketli")),
                ),
            )
            .child(
                section("Dikey Ayırıcılar").child(
                    h_flex()
                        .gap_4()
                        .h(px(100.))
                        .child(Ayirici::vertical())
                        .child(Ayirici::vertical().label("Düz"))
                        .child(Ayirici::vertical_dashed())
                        .child(Ayirici::vertical_dashed().label("Kesikli")),
                ),
            )
            .child(
                section("Birleşik Ayırıcılar").child(
                    v_flex()
                        .gap_y_4()
                        .child(
                            v_flex().gap_y_2().child("Merhaba Kavis UI").child(
                                Etiket::new(DESCRIPTION)
                                    .text_color(cx.theme().muted_foreground)
                                    .text_sm(),
                            ),
                        )
                        .child(Ayirici::horizontal())
                        .child(
                            h_flex()
                                .gap_x_4()
                                .child("Docs")
                                .child(Ayirici::vertical().dashed())
                                .child("Github")
                                .child(Ayirici::vertical().dashed())
                                .child("Source"),
                        ),
                ),
            )
    }
}
