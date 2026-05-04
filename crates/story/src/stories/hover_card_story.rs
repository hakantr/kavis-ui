use gpui::{
    Anchor, App, AppContext as _, Context, Entity, IntoElement, ParentElement as _, Render,
    Styled as _, Window, div, px, relative,
};
use kavis_ui::{
    EtkinTema, StilUzantisi, avatar::Avatar, button::Dugme, h_flex, hover_card::UzerineGelmeKarti,
    v_flex,
};
use std::time::Duration;

use crate::{Story, section};

pub struct HoverCardStory {}

impl HoverCardStory {
    fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for HoverCardStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .child(self.render_basic_example(cx))
            .child(self.render_user_profile_example(cx))
            .child(self.render_custom_timing_example(cx))
            .child(self.render_positioning_examples(cx))
    }
}

impl HoverCardStory {
    /// Temel üzerine gelme kartı örneği
    fn render_basic_example(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section("Basic").child(
            UzerineGelmeKarti::new("basic")
                .trigger(
                    div()
                        .child("Üzerime gel")
                        .text_color(cx.theme().primary)
                        .cursor_pointer()
                        .text_sm(),
                )
                .child(
                    v_flex()
                        .gap_1()
                        .w(px(450.))
                        .child(
                            div()
                                .child("Bu bir üzerine gelme kartıdır")
                                .font_semibold()
                                .text_sm(),
                        )
                        .child(
                            div()
                                .child("Tetikleyici öğenin üzerine gelindiğinde zengin içerik gösterebilirsiniz.")
                                .text_color(cx.theme().muted_foreground)
                                .text_sm(),
                        ),
                ),
        )
    }

    fn render_user_profile_example(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section("User Profile Preview").child(
            h_flex()
                .child("Profilini görmek için ")
                .child(
                    UzerineGelmeKarti::new("user-profile")
                        .trigger(
                            div()
                                .child("@huacnlee")
                                .cursor_pointer()
                                .text_color(cx.theme().link),
                        )
                        .content(|_, _, cx| {
                            h_flex()
                                .w(px(320.))
                                .gap_3()
                                .items_start()
                                .child(
                                    Avatar::new()
                                        .src("https://avatars.githubusercontent.com/u/5518?s=64"),
                                )
                                .child(
                                    v_flex()
                                        .gap_1()
                                        .line_height(relative(1.))
                                        .child(div().child("Jason Lee").font_semibold())
                                        .child(
                                            div()
                                                .child("@huacnlee")
                                                .text_color(cx.theme().link)
                                                .text_sm(),
                                        )
                                        .child(div().mt_1().child("Kavis UI yazarı.")),
                                )
                        }),
                )
                .child(" üzerine gelin"),
        )
    }

    /// özel timing yapılandırma örnek
    fn render_custom_timing_example(&self, _: &mut Context<Self>) -> impl IntoElement {
        section("Custom Timing").child(
            h_flex()
                .gap_4()
                .child(
                    UzerineGelmeKarti::new("fast-open")
                        .open_delay(Duration::from_millis(200))
                        .close_delay(Duration::from_millis(100))
                        .trigger(Dugme::new("fast").label("Hızlı Açılış (200ms)").outline())
                        .child(div().child("Bu kart 200ms sonra açılır").text_sm()),
                )
                .child(
                    UzerineGelmeKarti::new("slow-open")
                        .open_delay(Duration::from_secs(1))
                        .close_delay(Duration::from_secs_f32(0.5))
                        .trigger(Dugme::new("slow").label("Yavaş Açılış (1000ms)").outline())
                        .child(div().child("Bu kart 1000ms sonra açılır").text_sm()),
                ),
        )
    }

    /// Tüm konumlandırma seçenekleri.
    fn render_positioning_examples(&self, _: &mut Context<Self>) -> impl IntoElement {
        section("Positioning").child(
            v_flex()
                .gap_4()
                .items_center()
                .justify_center()
                .child(
                    h_flex()
                        .gap_4()
                        .child(
                            UzerineGelmeKarti::new("anchor-top-left")
                                .anchor(Anchor::TopLeft)
                                .trigger(Dugme::new("tl").label("Top Left").outline())
                                .child(div().child("Positioned at Top Left").text_sm()),
                        )
                        .child(
                            UzerineGelmeKarti::new("anchor-top-center")
                                .anchor(Anchor::TopCenter)
                                .trigger(Dugme::new("tc").label("Top Center").outline())
                                .child(div().child("Positioned at Top Center").text_sm()),
                        )
                        .child(
                            UzerineGelmeKarti::new("anchor-top-right")
                                .anchor(Anchor::TopRight)
                                .trigger(Dugme::new("tr").label("Top Right").outline())
                                .child(div().child("Positioned at Top Right").text_sm()),
                        ),
                )
                // Bottom row
                .child(
                    h_flex()
                        .gap_4()
                        .child(
                            UzerineGelmeKarti::new("anchor-bottom-left")
                                .anchor(Anchor::BottomLeft)
                                .trigger(Dugme::new("bl").label("Bottom Left").outline())
                                .child(div().child("Positioned at Bottom Left").text_sm()),
                        )
                        .child(
                            UzerineGelmeKarti::new("anchor-bottom-center")
                                .anchor(Anchor::BottomCenter)
                                .trigger(Dugme::new("bc").label("Bottom Center").outline())
                                .child(div().child("Positioned at Bottom Center").text_sm()),
                        )
                        .child(
                            UzerineGelmeKarti::new("anchor-bottom-right")
                                .anchor(Anchor::BottomRight)
                                .trigger(Dugme::new("br").label("Bottom Right").outline())
                                .child(div().child("Positioned at Bottom Right").text_sm()),
                        ),
                ),
        )
    }
}

impl Story for HoverCardStory {
    fn title() -> &'static str {
        "UzerineGelmeKarti"
    }

    fn description() -> &'static str {
        "A hover card displays content when hovering over a trigger element, with configurable delays."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}
