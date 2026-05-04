use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Window,
};
use kavis_ui::{
    BilesenBoyutu, Boyutlandirilabilir as _, Secilebilir as _, SimgeAdi,
    alert::Uyari,
    button::{Dugme, DugmeGrubu},
    dock::PanelDenetimi,
    text::markdown,
    v_flex,
};

use crate::section;

pub struct AlertStory {
    size: BilesenBoyutu,
    banner_visible: bool,
    focus_handle: gpui::FocusHandle,
}

impl AlertStory {
    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            size: BilesenBoyutu::default(),
            banner_visible: true,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn set_size(&mut self, size: BilesenBoyutu, _: &mut Window, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }
}

impl super::Story for AlertStory {
    fn title() -> &'static str {
        "Uyari"
    }

    fn description() -> &'static str {
        "Displays a callout for user attention."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn zoomable() -> Option<PanelDenetimi> {
        None
    }
}

impl Focusable for AlertStory {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AlertStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_4()
            .child(
                DugmeGrubu::new("toggle-size")
                    .outline()
                    .compact()
                    .child(
                        Dugme::new("xsmall")
                            .label("Çok Küçük")
                            .selected(self.size == BilesenBoyutu::CokKucuk),
                    )
                    .child(
                        Dugme::new("small")
                            .label("Küçük")
                            .selected(self.size == BilesenBoyutu::Kucuk),
                    )
                    .child(
                        Dugme::new("medium")
                            .label("Orta")
                            .selected(self.size == BilesenBoyutu::Orta),
                    )
                    .child(
                        Dugme::new("large")
                            .label("Büyük")
                            .selected(self.size == BilesenBoyutu::Buyuk),
                    )
                    .on_click(cx.listener(|this, selecteds: &Vec<usize>, window, cx| {
                        let size = match selecteds[0] {
                            0 => BilesenBoyutu::CokKucuk,
                            1 => BilesenBoyutu::Kucuk,
                            2 => BilesenBoyutu::Orta,
                            3 => BilesenBoyutu::Buyuk,
                            _ => unreachable!(),
                        };
                        this.set_size(size, window, cx);
                    })),
            )
            .child(
                section("Default").w_2_3().child(
                    Uyari::new(
                        "alert-default",
                        markdown(
                            "Bu, simge, başlık ve Markdown açıklaması olan bir uyarıdır.\n\
                            - Bu bir **liste** öğesidir.\n\
                            - Bu başka bir liste öğesidir.",
                        ),
                    )
                    .with_size(self.size)
                    .title("Başarılı! Değişiklikleriniz kaydedildi"),
                ),
            )
            .child(
                section("With variant").w_2_3().child(
                    v_flex()
                        .w_full()
                        .gap_3()
                        .child(
                            Uyari::info("info1", "Bu bir bilgi uyarısıdır.")
                                .with_size(self.size)
                                .title("Bilgi mesajı")
                                .on_close(cx.listener(|_, _, _, _| {
                                    println!("Bilgi uyarısı kapatıldı");
                                })),
                        )
                        .child(
                            Uyari::success(
                                "success-1",
                                "Formunuzu başarıyla gönderdiniz.\n\
                        Gönderiminiz için teşekkürler!",
                            )
                            .with_size(self.size)
                            .title("Gönderim Başarılı"),
                        )
                        .child(
                            Uyari::warning(
                                "warning-1",
                                "Bu, simgesi olan ancak başlığı olmayan bir uyarıdır.\n\
                            Bu ikinci satır, satır yüksekliğinin doğru olduğunu test eder.",
                            )
                            .with_size(self.size),
                        )
                        .child(
                            Uyari::error(
                                "error-1",
                                markdown(
                                    "Lütfen fatura bilgilerinizi doğrulayıp tekrar deneyin.\n\
                            - Kart bilgilerinizi kontrol edin\n\
                            - Yeterli bakiye olduğundan emin olun\n\
                            - Fatura adresini doğrulayın",
                                ),
                            )
                            .with_size(self.size)
                            .title("Ödemeniz işlenemedi."),
                        ),
                ),
            )
            .child(
                section("Banner").w_2_3().child(
                    v_flex()
                        .w_full()
                        .gap_2()
                        .child(
                            Uyari::new(
                                "banner-1",
                                "Bu bir banner uyarısıdır; \
                       kapsayıcının tam genişliğini kullanır.",
                            )
                            .banner()
                            .on_close(cx.listener(|this, _, _, cx| {
                                this.banner_visible = !this.banner_visible;
                                cx.notify();
                            }))
                            .visible(self.banner_visible)
                            .with_size(self.size),
                        )
                        .child(
                            Uyari::info(
                                "banner-info",
                                "Bu bir banner uyarısıdır; kapsayıcının tam genişliğini\
                    kullanır.",
                            )
                            .banner()
                            .with_size(self.size),
                        )
                        .child(
                            Uyari::success(
                                "banner-success",
                                "Bu bir banner uyarısıdır; kapsayıcının tam genişliğini\
                    kullanır.",
                            )
                            .banner()
                            .with_size(self.size),
                        )
                        .child(
                            Uyari::warning(
                                "banner-warning",
                                "Bu bir banner uyarısıdır; kapsayıcının tam genişliğini\
                    kullanır.",
                            )
                            .banner()
                            .with_size(self.size),
                        )
                        .child(
                            Uyari::error(
                                "banner-error",
                                "Bu bir banner uyarısıdır; kapsayıcının tam genişliğini\
                    kullanır.",
                            )
                            .banner()
                            .with_size(self.size),
                        ),
                ),
            )
            .child(
                section("Custom Simge").w_2_3().child(
                    Uyari::new(
                        "other-1",
                        "Custom icon with info alert with long \
                    long long long long long long long long \
                    long long long long long long long long long \
                    long long messageeeeeeeee.",
                    )
                    .title("Custom Simge")
                    .with_size(self.size)
                    .icon(SimgeAdi::Calendar),
                ),
            )
    }
}
