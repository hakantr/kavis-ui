use kavis_ui::ham_gpui::{
    Anchor, App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement as _,
    IntoElement, ParentElement, Render, Styled, Window,
};

use kavis_ui::{
    EtkinTema, PencereUzantisi as _, Tema,
    button::{Dugme, DugmeVaryantlari},
    h_flex,
    menu::{AcilirMenuOgesi, AcilirMenuTetikleyici as _},
    notification::{Bildirim, BildirimTuru},
    text::markdown,
    v_flex,
};

use crate::section;

const NOTIFICATION_MARKDOWN: &str = r#"
Bu özel bir bildirimdir.
- Liste öğesi 1
- Liste öğesi 2
- [Buraya tıklayın](https://github.com/hakantr/kavis-ui)
"#;

pub struct NotificationStory {
    focus_handle: FocusHandle,
}

impl super::Story for NotificationStory {
    fn title() -> &'static str {
        "Bildirim"
    }

    fn description() -> &'static str {
        "Push notifications to display a message at the top right of the window"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl NotificationStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for NotificationStory {
    fn focus_handle(&self, _cx: &kavis_ui::ham_gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for NotificationStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const ANCHORS: [Anchor; 6] = [
            Anchor::TopLeft,
            Anchor::TopCenter,
            Anchor::TopRight,
            Anchor::BottomLeft,
            Anchor::BottomCenter,
            Anchor::BottomRight,
        ];

        let view = cx.entity();

        v_flex()
            .id("notification-story")
            .track_focus(&self.focus_handle)
            .size_full()
            .gap_3()
            .child(
                h_flex().gap_3().child(
                    Dugme::new("placement")
                        .outline()
                        .label(format!("{:?}", cx.theme().notification.placement))
                        .acilir_menu(move |menu, window, cx| {
                            let menu = ANCHORS.into_iter().fold(menu, |menu, placement| {
                                menu.item(
                                    AcilirMenuOgesi::new(format!("{:?}", placement))
                                        .checked(cx.theme().notification.placement == placement)
                                        .on_click(window.listener_for(
                                            &view,
                                            move |_, _, _, cx| {
                                                Tema::global_mut(cx).notification.placement =
                                                    placement;
                                                cx.notify();
                                            },
                                        )),
                                )
                            });

                            menu
                        }),
                ),
            )
            .child(
                section("Simple Bildirim").child(
                    Dugme::new("show-notify-0")
                        .outline()
                        .label("Bildirim Göster")
                        .on_click(cx.listener(|_, _, window, cx| {
                            window.push_notification("Bu bir bildirimdir.", cx)
                        })),
                ),
            )
            .child(
                section("Bildirim with Type")
                    .child(
                        Dugme::new("show-notify-info")
                            .info()
                            .label("Bilgi")
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.push_notification(
                                    (
                                        BildirimTuru::Info,
                                        "Dosya başarıyla kaydedildi.",
                                    ),
                                    cx,
                                )
                            })),
                    )
                    .child(
                        Dugme::new("show-notify-error")
                            .danger()
                            .label("Hata")
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.push_notification(
                                    (
                                        BildirimTuru::Error,
                                        "Bir hata oluştu. Lütfen daha sonra tekrar deneyin.",
                                    ),
                                    cx,
                                )
                            })),
                    )
                    .child(
                        Dugme::new("show-notify-success")
                            .success()
                            .label("Başarılı")
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.push_notification(
                                    (
                                        BildirimTuru::Success,
                                        "Ödemeniz başarıyla alındı.",
                                    ),
                                    cx,
                                )
                            })),
                    )
                    .child(
                        Dugme::new("show-notify-warning")
                            .warning()
                            .label("Uyarı")
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.push_notification(
                                    (
                                        BildirimTuru::Warning,
                                        "Ağ bağlantısı kararlı değil. Lütfen bağlantınızı kontrol edin.",
                                    ),
                                    cx,
                                )
                            })),
                    ),
            )
            .child(
                section("Benzersiz Bildirim").child(
                    Dugme::new("show-notify-unique")
                        .outline()
                        .label("Benzersiz Bildirim")
                        .on_click(cx.listener(|_, _, window, cx| {
                            window.push_notification(
                                Bildirim::info("Bu benzersiz bir bildirimdir.")
                                    .id::<NotificationStory>()
                                    .message("Bu benzersiz bir bildirimdir.")
                                    .on_close(|_, _| {
                                        println!("Bildirim kapatıldı");
                                    }),
                                cx,
                            )
                        })),
                ),
            )
            .child(
                section("Unique with Key").child(
                    h_flex()
                        .gap_3()
                        .child(
                            Dugme::new("show-notify-unique-key0")
                                .outline()
                                .label("A Bildirimi")
                                .on_click(cx.listener(|_, _, window, cx| {
                                    window.push_notification(
                                        Bildirim::info("Bu A benzersiz bildirimidir.")
                                            .id1::<NotificationStory>(1),
                                        cx,
                                    )
                                })),
                        )
                        .child(
                            Dugme::new("show-notify-unique-key1")
                                .outline()
                                .label("B Bildirimi")
                                .on_click(cx.listener(|_, _, window, cx| {
                                    window.push_notification(
                                        Bildirim::info("Bu B benzersiz bildirimidir.")
                                            .id1::<NotificationStory>(2),
                                        cx,
                                    )
                                })),
                        ),
                ),
            )
            .child(
                section("With title and action").child(
                    Dugme::new("show-notify-with-title")
                        .outline()
                        .label("Başlıklı Bildirim")
                        .on_click(cx.listener(|_, _, window, cx| {
                            struct TestNotification;

                            window.push_notification(
                                Bildirim::new()
                                    .id::<TestNotification>()
                                    .title("Bir şeyler ters gitti.")
                                    .message("İsteğiniz işlenirken bir sorun oluştu.")
                                    .action(|_, _, cx| {
                                        Dugme::new("try-again")
                                            .primary()
                                            .label("Tekrar Dene")
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                println!("Tekrar dene eylemine tıklandı.");
                                                this.dismiss(window, cx);
                                            }))
                                    })
                                    .on_click(cx.listener(|_, _, _, cx| {
                                        println!("Bildirime tıklandı");
                                        cx.notify();
                                    })),
                                cx,
                            )
                        })),
                ),
            )
            .child(
                section("Custom Bildirim").child(
                    Dugme::new("show-notify-custom")
                        .outline()
                        .label("Özel Bildirim Göster")
                        .on_click(cx.listener(|_, _, window, cx| {
                            window.push_notification(
                                Bildirim::new().content(|_, _, _| {
                                    markdown(NOTIFICATION_MARKDOWN).into_any_element()
                                }),
                                cx,
                            )
                        })),
                ),
            )
            .child({
                struct ManualOpenNotification;

                section("Manual Close Bildirim")
                    .child(
                        Dugme::new("manual-open-notify")
                            .outline()
                            .label("Göster")
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.push_notification(
                                    Bildirim::new()
                                        .id::<ManualOpenNotification>()
                                        .message(
                                            "Bu bildirimi Kapat düğmesine tıklayarak \
                                            kapatabilirsiniz.",
                                        )
                                        .autohide(false),
                                    cx,
                                );
                            })),
                    )
                    .child(
                        Dugme::new("manual-close-notify")
                            .outline()
                            .label("Tümünü Kapat")
                            .on_click(cx.listener(|_, _, window, cx| {
                                window.remove_notification::<ManualOpenNotification>(cx);
                            })),
                    )
            })
    }
}
