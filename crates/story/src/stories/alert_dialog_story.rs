use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement as _, IntoElement,
    ParentElement, Render, Styled, Window, div, px,
};

use kavis_ui::{
    EtkinTema, PencereUzantisi as _, Simge, SimgeAdi, StyledExt,
    button::{Dugme, DugmeVaryanti, DugmeVaryantlari},
    dialog::{
        IletisimAciklamasi, IletisimAltligi, IletisimBasligi, IletisimBaslikMetni,
        IletisimDugmesiOzellikleri, IletisimEylemi, IletisimKapat, UyariIletisimKutusu,
    },
    v_flex,
};

use crate::section;

pub struct AlertDialogStory {
    focus_handle: FocusHandle,
}

impl super::Story for AlertDialogStory {
    fn title() -> &'static str {
        "UyariIletisimKutusu"
    }

    fn description() -> &'static str {
        "A modal dialog that interrupts the user with important content"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl AlertDialogStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for AlertDialogStory {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AlertDialogStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().id("alert-dialog-story").track_focus(&self.focus_handle).size_full().child(
            v_flex()
                .gap_6()
                .child(
                    section("UyariIletisimKutusu").child(
                        UyariIletisimKutusu::new(cx)
                            .p_0()
                            .trigger(Dugme::new("info-alert").outline().label("Bilgi Uyarısı Göster"))
                            .on_ok(|_, window, cx| {
                                window.push_notification("Uyarıyı onayladınız", cx);
                                true
                            })
                            .on_cancel(|_, window, cx| {
                                window.push_notification("Uyarıyı iptal ettiniz", cx);
                                true
                            })
                            .content(|content, _, cx| {
                                content
                                    .child(IletisimBasligi::new().p_4().child(IletisimBaslikMetni::new().child("Tamamen emin misiniz?")).child(
                                        IletisimAciklamasi::new().child(
                                            "Bu işlem geri alınamaz. \
                                            Hesabınız sunucularımızdan kalıcı olarak silinir.",
                                        ),
                                    ))
                                    .child(IletisimAltligi::new()
                                        .p_4()
                                        .border_t_1()
                                        .border_color(cx.theme().border)
                                        .bg(cx.theme().muted)
                                        .child(
                                            IletisimKapat::new().child(
                                                Dugme::new("cancel").outline().label("İptal")
                                            )
                                        )
                                        .child(
                                            IletisimEylemi::new().child(
                                                Dugme::new("ok").label("Devam Et").primary()
                                            )
                                        )
                                    )
                            }),
                    ),
                )
                .child(section("With open_alert_dialog").child(
                    Dugme::new("confirm-alert").outline().label("Onay Göster").on_click(cx.listener(
                        |_, _, window, cx| {
                            use kavis_ui::dialog::IletisimDugmesiOzellikleri;

                            window.open_alert_dialog(cx, |alert, _, cx| {
                                alert
                                    .icon(Simge::new(SimgeAdi::Info).text_color(cx.theme().danger))
                                    .title("Dosyayı Sil")
                                    .description(
                                        "Bu dosyayı silmek istediğinizden emin misiniz? \
                                                Bu işlem geri alınamaz.",
                                    )
                                    .button_props(
                                        IletisimDugmesiOzellikleri::default()
                                            .ok_variant(DugmeVaryanti::Danger)
                                            .ok_text("Sil")
                                            .cancel_text("İptal")
                                            .show_cancel(true),
                                    )
                                    .on_ok(|_, window, cx| {
                                        window.push_notification("Dosya silindi", cx);
                                        true
                                    })
                            });
                        },
                    )),
                ))
                .child(section("With Simge").child(
                    UyariIletisimKutusu::new(cx).w(px(320.)).trigger(
                        Dugme::new("icon-alert").outline().label("İzin İste"),
                    ).on_ok(|_, window, cx| {
                        window.push_notification("Ağ erişimine izin verdiğiniz için teşekkürler", cx);
                        true
                    })
                    .content(|content, _, cx| {
                        content
                            .child(
                                IletisimBasligi::new()
                                    .items_center()
                                    .child(Simge::new(SimgeAdi::TriangleAlert).size_10().text_color(cx.theme().warning))
                                    .child(
                                        IletisimBaslikMetni::new().child("Ağ İzni Gerekli"),
                                    ).child(
                                        IletisimAciklamasi::new().child(
                                            "We need your permission to access the network to provide better services. \
                                            Please allow network access in your system settings.",
                                        ),
                                    ),
                            )
                            .child(
                                IletisimAltligi::new()
                                    .v_flex()
                                    .child(
                                        IletisimEylemi::new().child(
                                            Dugme::new("agree").w_full().primary().label("İzin Ver")
                                        )
                                    )
                                    .child(
                                        IletisimKapat::new().child(
                                            Dugme::new("disagree").w_full().outline().label("İzin Verme")
                                        )
                                    )
                            )
                    }),
                ))
                .child(
                    section("Destructive Action").child(
                        UyariIletisimKutusu::new(cx)
                            .trigger(Dugme::new("destructive-action").outline().danger().label("Hesabı Sil"))
                            .on_ok(|_, window, cx| {
                                window.push_notification("Hesabınız silindi", cx);
                                true
                            })
                            .content(|content, _, _| {
                                content
                                    .child(IletisimBasligi::new().child(IletisimBaslikMetni::new().child("Hesabı Sil")).child(
                                        IletisimAciklamasi::new().child(
                                            "This will permanently delete your account \
                                                    and all associated data. This action cannot be undone.",
                                        ),
                                    ))
                                    .child(
                                        IletisimAltligi::new()
                                            .child(
                                                IletisimKapat::new().child(
                                                    Dugme::new("cancel").flex_1().outline().label("İptal")
                                                )
                                            )
                                            .child(
                                                IletisimEylemi::new().child(
                                                    Dugme::new("delete")
                                                        .flex_1()
                                                        .outline()
                                                        .danger()
                                                        .label("Kalıcı Olarak Sil")
                                                )
                                            )
                                    )
                            }),
                    ),
                )
                .child(section("Without Title").child(
                    Dugme::new("without-title").outline().label("Başlıksız İletişim Kutusu").on_click(cx.listener(
                        |_, _, window, cx| {
                            window.open_alert_dialog(cx, |alert, _, _| {
                                alert
                                    .confirm()
                                    .child("This is a UyariIletisimKutusu with `confirm` mode.\
                                        Will have OK, CANCEL buttons.")
                            });
                        },
                    )),
                ))
                .child(section("Oturum Zaman Aşımı").child(
                    Dugme::new("session-timeout").outline().label("Oturum Zaman Aşımı").on_click(cx.listener(
                        |_, _, window, cx| {
                            window.open_alert_dialog(cx, |alert, _, _| {
                                alert
                                    .on_ok(|_, window, cx| {
                                        window.push_notification("Oturum açma sayfasına yönlendiriliyor...", cx);
                                        true
                                    })
                                    .title("Oturum Süresi Doldu")
                                    .description("Your session has expired due to inactivity.\
                                        Please log in again to continue.")
                                    .footer(
                                        IletisimAltligi::new().child(
                                            Dugme::new("sign-in").label("Oturum Aç").primary().flex_1().on_click(
                                                move |_, window, cx| {
                                                    window.push_notification("Oturum açma sayfasına yönlendiriliyor...", cx);
                                                    window.close_dialog(cx);
                                                },
                                            ),
                                        )
                                    )
                            });
                        },
                    )),
                ))
                .child(section("Güncelleme Var").child(
                    UyariIletisimKutusu::new(cx)
                        .trigger(Dugme::new("update").outline().label("Güncelleme Var"))
                        .on_cancel(|_, window, cx| {
                            window.push_notification("Güncelleme ertelendi", cx);
                            true
                        })
                        .on_ok(|_, window, cx| {
                            window.push_notification("Güncelleme başlatılıyor...", cx);
                            true
                        })
                        .content(
                        |content, _, cx| {
                            content
                                .child(IletisimBasligi::new().child(IletisimBaslikMetni::new().child("Güncelleme Var")).child(
                                    IletisimAciklamasi::new().child(
                                        "A new version (v2.0.0) is available.\
                                                This update includes new features and bug fixes.",
                                    ),
                                ))
                                .child(
                                    IletisimAltligi::new()
                                        .bg(cx.theme().muted)
                                        .child(
                                            IletisimKapat::new().child(
                                                Dugme::new("later").flex_1().outline().label("Sonra")
                                            ),
                                        )
                                        .child(
                                            IletisimEylemi::new().child(
                                                Dugme::new("update-now").flex_1().primary().label("Şimdi Güncelle")
                                            )
                                        )
                                )
                        },
                    ),
                ))
                .child(section("Klavye Devre Dışı").child(
                    Dugme::new("keyboard-disabled").outline().label("Klavye Devre Dışı").on_click(cx.listener(
                        |_, _, window, cx| {
                            window.open_alert_dialog(cx, |alert, _, _| {
                                alert
                                    .title("Önemli Bildirim")
                                    .description(
                                        "Please read this important notice \
                                                carefully before proceeding.",
                                    )
                                    .keyboard(false)
                            });
                        },
                    )),
                ))
                .child(section("With confirm mode").child(
                    Dugme::new("overlay-closable").outline().label("Onay Modu").on_click(cx.listener(
                        |_, _, window, cx| {
                            window.open_alert_dialog(cx, |alert, _, _| {
                                alert
                                    .confirm()
                                    .title("Emin misiniz?")
                                    .child("This is a UyariIletisimKutusu with `confirm` mode.\
                                        Will have OK, CANCEL buttons.")
                            });
                        },
                    )),
                ))
                .child(section("Kaplama Kapatılabilir").child(
                    Dugme::new("overlay-closable").outline().label("Kaplama Kapatılabilir").on_click(cx.listener(
                        |_, _, window, cx| {
                            window.open_alert_dialog(cx, |alert, _, _| {
                                alert
                                    .title("Kaplama Kapatılabilir")
                                    .description("Kapatmak için iletişim kutusunun dışına tıklayın veya ESC'ye basın.")
                                    .overlay_closable(true)
                            });
                        },
                    )),
                ))
                .child(section("Kapatmayı Önle").child(
                    Dugme::new("prevent-close").outline().label("Kapatmayı Önle").on_click(cx.listener(
                        |_, _, window, cx| {
                            window.open_alert_dialog(cx, |alert, _, _| {
                                alert
                                    .title("İşleniyor")
                                    .close_button(true)
                                    .description(
                                        "Bir işlem çalışıyor. \
                                                Durdurmak için Devam Et'e, beklemek için İptal'e tıklayın.",
                                    )
                                    .button_props(IletisimDugmesiOzellikleri::default().ok_text("Devam Et").show_cancel(true))
                                    .on_ok(|_, window, cx| {
                                        // Return false to prevent closing
                                        window.push_notification("Kapatılamıyor: işlem hâlâ çalışıyor", cx);
                                        false
                                    })
                                    .on_cancel(|_, window, cx| {
                                        window.push_notification("Bekleniyor...", cx);
                                        false
                                    })
                            });
                        },
                    )),
                ))
        )
    }
}
