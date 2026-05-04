use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Window, div, px,
};

use kavis_ui::{
    BilesenBoyutu, Boyutlandirilabilir, EtkinTema, Simge, SimgeAdi, StilUzantisi,
    badge::Rozet,
    button::{Dugme, DugmeVaryantlari},
    divider::Ayirici,
    h_flex,
    scroll::KaydirilabilirOge,
    tag::Cip,
    v_flex,
};

use crate::Story;

pub struct ZedWorkspaceStory {
    focus_handle: FocusHandle,
}

impl ZedWorkspaceStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    fn render_thread(
        &self,
        title: &'static str,
        meta: &'static str,
        icon: SimgeAdi,
        state: &'static str,
        active: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let tag = match state {
            "Running" => Cip::info()
                .outline()
                .with_size(BilesenBoyutu::Kucuk)
                .child(state),
            "Review" => Cip::warning()
                .outline()
                .with_size(BilesenBoyutu::Kucuk)
                .child(state),
            "Done" => Cip::success()
                .outline()
                .with_size(BilesenBoyutu::Kucuk)
                .child(state),
            _ => Cip::secondary()
                .outline()
                .with_size(BilesenBoyutu::Kucuk)
                .child(state),
        };

        v_flex()
            .gap_1()
            .p_2()
            .rounded(cx.theme().radius)
            .border_1()
            .border_color(if active {
                cx.theme().ring
            } else {
                cx.theme().transparent
            })
            .bg(if active {
                cx.theme().sidebar_accent
            } else {
                cx.theme().transparent
            })
            .hover(|this| this.bg(cx.theme().sidebar_accent.opacity(0.65)))
            .child(
                h_flex()
                    .min_w_0()
                    .gap_2()
                    .child(Simge::new(icon).with_size(BilesenBoyutu::Kucuk))
                    .child(div().flex_1().truncate().child(title))
                    .child(tag),
            )
            .child(
                h_flex()
                    .gap_1()
                    .pl_5()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(meta),
            )
    }

    fn render_change_row(
        &self,
        path: &'static str,
        detail: &'static str,
        additions: usize,
        deletions: usize,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .gap_2()
            .py_1()
            .px_2()
            .rounded(cx.theme().radius / 2.)
            .hover(|this| this.bg(cx.theme().accent))
            .child(Simge::new(SimgeAdi::File).with_size(BilesenBoyutu::Kucuk))
            .child(
                v_flex()
                    .min_w_0()
                    .flex_1()
                    .child(div().truncate().child(path))
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .truncate()
                            .child(detail),
                    ),
            )
            .child(
                h_flex()
                    .gap_1()
                    .text_xs()
                    .child(
                        div()
                            .text_color(cx.theme().success)
                            .child(format!("+{additions}")),
                    )
                    .child(
                        div()
                            .text_color(cx.theme().danger)
                            .child(format!("-{deletions}")),
                    ),
            )
    }

    fn render_agent_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .flex_1()
            .min_w(px(360.))
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius)
            .overflow_hidden()
            .bg(cx.theme().background)
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .gap_3()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Simge::new(SimgeAdi::Bot))
                            .child("Ajan Paneli")
                            .child(
                                Cip::info()
                                    .outline()
                                    .with_size(BilesenBoyutu::Kucuk)
                                    .child("ACP"),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_1()
                            .child(
                                Dugme::new("zed-workspace-stop")
                                    .ghost()
                                    .icon(SimgeAdi::Pause),
                            )
                            .child(Dugme::new("zed-workspace-new").ghost().icon(SimgeAdi::Plus)),
                    ),
            )
            .child(
                v_flex()
                    .gap_3()
                    .p_3()
                    .child(
                        v_flex()
                            .gap_2()
                            .p_3()
                            .rounded(cx.theme().radius)
                            .bg(cx.theme().muted)
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Cip::success()
                                            .outline()
                                            .with_size(BilesenBoyutu::Kucuk)
                                            .child("Düşünme: Yüksek"),
                                    )
                                    .child(
                                        Cip::secondary()
                                            .outline()
                                            .with_size(BilesenBoyutu::Kucuk)
                                            .child("Vercel AI Gateway"),
                                    ),
                            )
                            .child("Ayarlar yüzeyini ajanlı iş akışları için düzenle.")
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(
                                        "Bağlamda @branch-diff, seçili terminal çıktısı ve bağlı \
                                        çalışma ağacı var.",
                                    ),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Simge::new(SimgeAdi::LoaderCircle))
                                    .child("Paralel alt ajan UI incelemesi yapıyor")
                                    .child(
                                        Cip::warning()
                                            .outline()
                                            .with_size(BilesenBoyutu::Kucuk)
                                            .child("İzin gerekli"),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(
                                        "Araç izin kuralları güvenli okumaları otomatik onaylayıp \
                                        yazma işlemleri için onay isteyebilir.",
                                    ),
                            ),
                    )
                    .child(Ayirici::horizontal())
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                h_flex().justify_between().child("Dal farkı").child(
                                    Cip::secondary()
                                        .outline()
                                        .with_size(BilesenBoyutu::Kucuk)
                                        .child("bölünmüş görünüm"),
                                ),
                            )
                            .child(self.render_change_row(
                                "crates/story/src/stories/settings_story.rs",
                                "Ajan ayarları ve sağlayıcı kontrolleri",
                                42,
                                8,
                                cx,
                            ))
                            .child(self.render_change_row(
                                "crates/story/src/stories/sidebar_story.rs",
                                "Oturum kenar çubuğu terminolojisi",
                                31,
                                12,
                                cx,
                            )),
                    ),
            )
    }

    fn render_right_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w(px(260.))
            .gap_3()
            .child(
                v_flex()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded(cx.theme().radius)
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .justify_between()
                            .px_3()
                            .py_2()
                            .border_b_1()
                            .border_color(cx.theme().border)
                            .child(h_flex().gap_2().child(SimgeAdi::FolderOpen).child("Proje"))
                            .child(
                                Cip::secondary()
                                    .outline()
                                    .with_size(BilesenBoyutu::Kucuk)
                                    .child("sağ dock"),
                            ),
                    )
                    .child(
                        v_flex()
                            .p_2()
                            .text_sm()
                            .gap_1()
                            .child(h_flex().gap_2().child(SimgeAdi::Folder).child("crates"))
                            .child(
                                h_flex()
                                    .gap_2()
                                    .pl_4()
                                    .child(SimgeAdi::Folder)
                                    .child("story"),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .pl_8()
                                    .child(SimgeAdi::File)
                                    .child("zed_workspace_story.rs"),
                            ),
                    ),
            )
            .child(
                v_flex()
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded(cx.theme().radius)
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .justify_between()
                            .px_3()
                            .py_2()
                            .border_b_1()
                            .border_color(cx.theme().border)
                            .child(h_flex().gap_2().child(SimgeAdi::Github).child("Git"))
                            .child(
                                Rozet::new().count(4).child(
                                    Simge::new(SimgeAdi::Bell).with_size(BilesenBoyutu::Kucuk),
                                ),
                            ),
                    )
                    .child(
                        v_flex()
                            .p_2()
                            .gap_2()
                            .text_sm()
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child("Commit'i görüntüle")
                                    .child(SimgeAdi::ExternalLink),
                            )
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child("Tümünü seç")
                                    .child(SimgeAdi::Check),
                            )
                            .child(
                                h_flex().justify_between().child("Yer imleri").child(
                                    Cip::success()
                                        .outline()
                                        .with_size(BilesenBoyutu::Kucuk)
                                        .child("kalıcı"),
                                ),
                            ),
                    ),
            )
    }
}

impl Story for ZedWorkspaceStory {
    fn title() -> &'static str {
        "Zed 1.0 Çalışma Alanı"
    }

    fn description() -> &'static str {
        "Paralel ajan oturumları, bölünmüş diff bağlamı ve sağa sabitlenen Proje/Git panelleri içeren Zed esinli çalışma alanı düzeni."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn paddings() -> kavis_ui::ham_gpui::Pixels {
        px(0.)
    }
}

impl Focusable for ZedWorkspaceStory {
    fn focus_handle(&self, _: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ZedWorkspaceStory {
    fn render(
        &mut self,
        _: &mut kavis_ui::ham_gpui::Window,
        cx: &mut kavis_ui::ham_gpui::Context<Self>,
    ) -> impl kavis_ui::ham_gpui::IntoElement {
        div()
            .size_full()
            .overflow_y_scrollbar()
            .bg(cx.theme().background)
            .child(
                v_flex()
                    .gap_4()
                    .p_4()
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .flex_wrap()
                            .gap_4()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .text_lg()
                                            .font_semibold()
                                            .child("Zed 1.0 ajanlı düzen"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(
                                                "Oturumlar solda kalır, Ajan ortada yer alır ve \
                                                Proje/Git panelleri sağa sabitlenir.",
                                            ),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(Cip::success().with_size(BilesenBoyutu::Kucuk).child("120 fps"))
                                    .child(
                                        Cip::info()
                                            .outline()
                                            .with_size(BilesenBoyutu::Kucuk)
                                            .child("Paralel ajanlar"),
                                    )
                                    .child(
                                        Cip::secondary()
                                            .outline()
                                            .with_size(BilesenBoyutu::Kucuk)
                                            .child("Zed 1.0"),
                                    ),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_3()
                            .items_stretch()
                            .flex_wrap()
                            .child(
                                v_flex()
                                    .w(px(270.))
                                    .border_1()
                                    .border_color(cx.theme().sidebar_border)
                                    .rounded(cx.theme().radius)
                                    .bg(cx.theme().sidebar)
                                    .text_color(cx.theme().sidebar_foreground)
                                    .overflow_hidden()
                                    .child(
                                        h_flex()
                                            .justify_between()
                                            .items_center()
                                            .px_3()
                                            .py_2()
                                            .border_b_1()
                                            .border_color(cx.theme().sidebar_border)
                                            .child(
                                                h_flex()
                                                    .gap_2()
                                                    .child(Simge::new(SimgeAdi::Bot))
                                                    .child("Oturumlar"),
                                            )
                                            .child(
                                                Dugme::new("zed-workspace-new-thread")
                                                    .ghost()
                                                    .icon(SimgeAdi::Plus),
                                            ),
                                    )
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .p_2()
                                            .child(
                                                div()
                                                    .px_1()
                                                    .py_1()
                                                    .text_xs()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child("kavis-ui"),
                                            )
                                            .child(self.render_thread(
                                                "Ayarlar sayfasını düzenle",
                                                "bağlı çalışma ağacı / ui-settings",
                                                SimgeAdi::Bot,
                                                "Running",
                                                true,
                                                cx,
                                            ))
                                            .child(self.render_thread(
                                                "Bölünmüş diff durumlarını incele",
                                                "dal farkı / 2 dosya değişti",
                                                SimgeAdi::Github,
                                                "Review",
                                                false,
                                                cx,
                                            ))
                                            .child(self.render_thread(
                                                "Yer imi kullanımını ekle",
                                                "editör kenarı / kalıcı",
                                                SimgeAdi::Star,
                                                "Done",
                                                false,
                                                cx,
                                            )),
                                    )
                                    .child(Ayirici::horizontal())
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .p_2()
                                            .child(
                                                div()
                                                    .px_1()
                                                    .py_1()
                                                    .text_xs()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child("harici ajanlar"),
                                            )
                                            .child(self.render_thread(
                                                "Codex takip işi",
                                                "ACP oturum geçmişi geri yüklendi",
                                                SimgeAdi::SquareTerminal,
                                                "Idle",
                                                false,
                                                cx,
                                            )),
                                    ),
                            )
                            .child(self.render_agent_panel(cx))
                            .child(self.render_right_panel(cx)),
                    )
                    .child(
                        h_flex()
                            .justify_between()
                            .gap_3()
                            .flex_wrap()
                            .px_3()
                            .py_2()
                            .rounded(cx.theme().radius)
                            .bg(cx.theme().muted)
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(SimgeAdi::CircleCheck)
                                    .child("Yer imleri, çok sözcüklü bulanık arama ve komut paleti eylemleri kompakt bileşen durumları olarak gösterilir."),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(SimgeAdi::PanelLeft)
                                    .child("Oturum Kenar Çubuğu"),
                            ),
                    ),
            )
    }
}
