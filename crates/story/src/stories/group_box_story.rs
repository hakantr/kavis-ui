use gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render,
    StyleRefinement, Styled, Window, relative,
};

use kavis_ui::{
    EtkinTema as _, StyledExt,
    button::{Dugme, DugmeVaryantlari},
    checkbox::OnayKutusu,
    group_box::{GrupKutusu, GrupKutusuVaryantlari as _},
    h_flex,
    radio::{Radyo, RadyoGrubu},
    switch::Anahtar,
    text::markdown,
    v_flex,
};

use crate::section;

pub struct GroupBoxStory {
    focus_handle: gpui::FocusHandle,
}

impl super::Story for GroupBoxStory {
    fn title() -> &'static str {
        "GrupKutusu"
    }

    fn description() -> &'static str {
        "A styled container element that with an optional title \
        to groups related content together."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl GroupBoxStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for GroupBoxStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for GroupBoxStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .justify_center()
            .gap_4()
            .child(
                section("Default Style").w_128().child(
                    GrupKutusu::new()
                        .child("Abonelikler")
                        .child(OnayKutusu::new("all").label("Tümü"))
                        .child(OnayKutusu::new("news-letter").label("Bülten"))
                        .child(OnayKutusu::new("account-activity").label("Hesap Etkinliği"))
                        .child(Dugme::new("ok").primary().label("Abonelikleri Güncelle")),
                ),
            )
            .child(
                section("Fill Style").w_128().child(
                    GrupKutusu::new()
                        .id("activity")
                        .fill()
                        .title("Katkılar ve etkinlik")
                        .child(
                            h_flex()
                                .justify_between()
                                .child("Profili gizli yap ve etkinliği gizle")
                                .child(Anahtar::new("toggle-0").checked(true)),
                        )
                        .child(
                            h_flex()
                                .justify_between()
                                .child("Özel katkıları profilime dahil et")
                                .child(Anahtar::new("toggle-1").checked(false)),
                        )
                        .child(Dugme::new("btn-1").primary().label("Kaydet")),
                ),
            )
            .child(
                section("Outline Style").w_128().child(
                    GrupKutusu::new()
                        .id("appearance")
                        .outline()
                        .title("Görünüm")
                        .child(
                            RadyoGrubu::vertical("theme")
                                .child(Radyo::new("light").label("Açık"))
                                .child(Radyo::new("dark").label("Koyu"))
                                .child(Radyo::new("system").label("Sistem")),
                        ),
                ),
            )
            .child(
                section("Without Title").w_128().child(
                    GrupKutusu::new().outline().child(
                        h_flex()
                            .justify_between()
                            .child("Profili gizli yap ve etkinliği gizle")
                            .child(Anahtar::new("toggle-1").checked(true)),
                    ),
                ),
            )
            .child(
                section("Custom style").w_128().child(
                    GrupKutusu::new()
                        .outline()
                        .bg(cx.theme().group_box)
                        .rounded_xl()
                        .p_5()
                        .title("Bu özel bir stildir")
                        .title_style(
                            StyleRefinement::default()
                                .font_semibold()
                                .line_height(relative(1.0))
                                .px_3(),
                        )
                        .content_style(
                            StyleRefinement::default()
                                .rounded_xl()
                                .py_3()
                                .px_4()
                                .border_2(),
                        )
                        .child(markdown(
                            "You can use `title_style` to customize the style \
                                of the title. \n \
                                And any style in `GrupKutusu` will apply to the content container.",
                        )),
                ),
            )
    }
}
