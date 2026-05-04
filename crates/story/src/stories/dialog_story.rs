use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement as _, IntoElement,
    ParentElement, Render, SharedString, Styled, Window, div, px,
};

use kavis_ui::{
    EtkinTema, PencereUzantisi as _, Simge, SimgeAdi,
    button::{Dugme, DugmeVaryantlari as _},
    checkbox::OnayKutusu,
    date_picker::{TarihSecici, TarihSeciciDurumu},
    dialog::{
        IletisimAciklamasi, IletisimAltligi, IletisimBasligi, IletisimBaslikMetni, IletisimEylemi,
        IletisimKapat, IletisimKutusu,
    },
    h_flex,
    input::{Girdi, GirdiDurumu},
    select::{Secim, SecimDurumu},
    table::{Column, TabloDurumu, TabloTemsilcisi, VeriTablosu},
    text::markdown,
    v_flex,
};

use crate::{TestAction, section};

pub struct DialogStory {
    focus_handle: FocusHandle,
    selected_value: Option<SharedString>,
    input1: Entity<GirdiDurumu>,
    input2: Entity<GirdiDurumu>,
    date: Entity<TarihSeciciDurumu>,
    select: Entity<SecimDurumu<Vec<String>>>,
    table: Entity<TabloDurumu<MyTable>>,
    dialog_overlay: bool,
    close_button: bool,
    keyboard: bool,
    overlay_closable: bool,
}

struct MyTable {
    columns: Vec<Column>,
}
impl MyTable {
    fn new(_: &mut App) -> Self {
        let columns = vec![
            Column::new("id", "ID").width(px(50.)),
            Column::new("name", "Ad").width(px(150.)),
            Column::new("email", "E-posta").width(px(250.)),
            Column::new("role", "Rol").width(px(150.)),
            Column::new("status", "Durum").width(px(100.)),
        ];

        Self { columns }
    }
}
impl TabloTemsilcisi for MyTable {
    fn columns_count(&self, _: &App) -> usize {
        5
    }

    fn rows_count(&self, _: &App) -> usize {
        200
    }

    fn column(&self, col_ix: usize, _: &App) -> Column {
        self.columns[col_ix].clone()
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        _: &mut Context<TabloDurumu<Self>>,
    ) -> impl IntoElement {
        match col_ix {
            0 => format!("{}", row_ix).into_any_element(),
            1 => format!("Kullanıcı {}", row_ix).into_any_element(),
            2 => format!("user-{}@mail.com", row_ix).into_any_element(),
            3 => "Kullanıcı".into_any_element(),
            4 => "Etkin".into_any_element(),
            _ => panic!("Invalid column index"),
        }
    }
}

impl super::Story for DialogStory {
    fn title() -> &'static str {
        "IletisimKutusu"
    }

    fn description() -> &'static str {
        "A dialog dialog"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl DialogStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input1 = cx.new(|cx| GirdiDurumu::new(window, cx).placeholder("Adınız"));
        let input2 = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .placeholder("İletişim kutusu kapanınca odağın geri dönmesini test etmek için.")
        });
        let date = cx.new(|cx| TarihSeciciDurumu::new(window, cx));
        let select = cx.new(|cx| {
            SecimDurumu::new(
                vec![
                    "Seçenek 1".to_string(),
                    "Seçenek 2".to_string(),
                    "Seçenek 3".to_string(),
                ],
                None,
                window,
                cx,
            )
        });

        let table = cx.new(|cx| TabloDurumu::new(MyTable::new(cx), window, cx));

        Self {
            focus_handle: cx.focus_handle(),
            selected_value: None,
            input1,
            input2,
            date,
            select,
            dialog_overlay: true,
            close_button: true,
            keyboard: true,
            overlay_closable: true,
            table,
        }
    }

    fn on_action_test_action(
        &mut self,
        _: &TestAction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.push_notification("TestAction eylemine tıkladınız.", cx);
    }

    fn render_basic_dialog(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_overlay = self.dialog_overlay;
        let overlay_closable = self.overlay_closable;
        let input1 = self.input1.clone();
        let date = self.date.clone();
        let select = self.select.clone();
        let view = cx.entity();

        section("Temel İletişim Kutusu").child(
            IletisimKutusu::new(cx)
                .trigger(
                    Dugme::new("show-dialog")
                        .outline()
                        .label("İletişim Kutusunu Aç"),
                )
                .overlay(dialog_overlay)
                .keyboard(self.keyboard)
                .close_button(self.close_button)
                .overlay_closable(overlay_closable)
                .on_ok({
                    let view = view.clone();
                    let input1 = input1.clone();
                    let date = date.clone();
                    move |_, window, cx| {
                        view.update(cx, |view, cx| {
                            view.selected_value = Some(
                                format!(
                                    "Merhaba, {}, tarih: {}",
                                    input1.read(cx).value(),
                                    date.read(cx).date()
                                )
                                .into(),
                            )
                        });
                        window.push_notification("Onayla düğmesine bastınız.", cx);
                        true
                    }
                })
                .p_0()
                .content({
                    move |content, _, cx| {
                        content
                            .child(
                                IletisimBasligi::new()
                                    .p_4()
                                    .child(
                                        IletisimBaslikMetni::new().child("Temel İletişim Kutusu"),
                                    )
                                    .child(IletisimAciklamasi::new().child(
                                        "Bu, bildirime dayalı API ile oluşturulmuş \
                                        temel bir iletişim kutusudur.",
                                    )),
                            )
                            .child(
                                v_flex()
                                    .px_4()
                                    .pb_4()
                                    .gap_3()
                                    .child(
                                        "Bu bir iletişim kutusudur; \
                                        buraya herhangi bir içerik koyabilirsiniz.",
                                    )
                                    .child(Girdi::new(&input1))
                                    .child(Secim::new(&select))
                                    .child(TarihSecici::new(&date).placeholder("Doğum Tarihi")),
                            )
                            .child(
                                IletisimAltligi::new()
                                    .p_4()
                                    .bg(cx.theme().muted)
                                    .justify_between()
                                    .child(
                                        Dugme::new("new-dialog")
                                            .label("Başka İletişim Kutusu Aç")
                                            .outline()
                                            .on_click(move |_, window, cx| {
                                                window.open_dialog(cx, move |dialog, _, _| {
                                                    dialog
                                                        .title("Diğer İletişim Kutusu")
                                                        .child("Bu başka bir iletişim kutusudur.")
                                                        .min_h(px(100.))
                                                        .overlay_closable(overlay_closable)
                                                });
                                            }),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(IletisimKapat::new().child(
                                                Dugme::new("cancel").label("İptal").outline(),
                                            ))
                                            .child(IletisimEylemi::new().child(
                                                Dugme::new("confirm").primary().label("Onayla"),
                                            )),
                                    ),
                            )
                    }
                }),
        )
    }

    fn render_focus_back_test(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        section("Focus back test")
            .max_w_md()
            .child(Girdi::new(&self.input2))
            .child(
                Dugme::new("test-action")
                    .outline()
                    .label("Test Eylemi")
                    .flex_shrink_0()
                    .on_click(|_, window, cx| {
                        window.dispatch_action(Box::new(TestAction), cx);
                    })
                    .tooltip(
                        "Bu düğme eylem göndermeyi test eder; \
                        iletişim kutusu kapandığında bile\
                        \neylemin işlenebildiğini doğrular.",
                    ),
            )
    }

    fn render_dialog_without_title(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_overlay = self.dialog_overlay;
        let overlay_closable = self.overlay_closable;

        section("Başlıksız İletişim Kutusu").child(
            Dugme::new("dialog-no-title")
                .outline()
                .label("Başlıksız İletişim Kutusu")
                .on_click(cx.listener(move |_, _, window, cx| {
                    window.open_dialog(cx, move |dialog, _, _| {
                        dialog
                            .overlay(dialog_overlay)
                            .overlay_closable(overlay_closable)
                            .child(
                                "Bu başlıksız bir iletişim kutusudur; \
                                başlık gerekmediğinde kullanabilirsiniz.",
                            )
                    });
                })),
        )
    }

    fn render_custom_buttons(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_overlay = self.dialog_overlay;
        let overlay_closable = self.overlay_closable;

        section("Custom buttons").child(
            Dugme::new("confirm-dialog1")
                .outline()
                .label("Özel Düğmeler")
                .on_click(cx.listener(move |_, _, window, cx| {
                    window.open_dialog(cx, move |dialog, _, cx| {
                        dialog
                            .rounded(cx.theme().radius_lg)
                            .overlay(dialog_overlay)
                            .overlay_closable(overlay_closable)
                            .child(
                                v_flex()
                                    .gap_3()
                                    .items_center()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .rounded(cx.theme().radius_lg)
                                            .bg(cx.theme().warning.opacity(0.2))
                                            .size_12()
                                            .text_color(cx.theme().warning)
                                            .child(Simge::new(SimgeAdi::TriangleAlert).size_8()),
                                    )
                                    .child(
                                        "Güncelleme başarılı, \
                                        uygulamayı yeniden başlatmamız gerekiyor.",
                                    ),
                            )
                            .footer(
                                IletisimAltligi::new()
                                    .child(
                                        IletisimKapat::new()
                                            .child(Dugme::new("cancel").label("Sonra").outline()),
                                    )
                                    .child(IletisimEylemi::new().child(
                                        Dugme::new("ok").label("Şimdi Yeniden Başlat").primary(),
                                    )),
                            )
                            .on_ok(|_, window, cx| {
                                window.push_notification("Yeniden başlat düğmesine bastınız.", cx);
                                true
                            })
                            .on_cancel(|_, window, cx| {
                                window.push_notification("Sonra düğmesine bastınız.", cx);
                                true
                            })
                    });
                })),
        )
    }

    fn render_scrollable_dialog(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_overlay = self.dialog_overlay;
        let overlay_closable = self.overlay_closable;

        section("Kaydırılabilir İletişim Kutusu").child(
            Dugme::new("scrollable-dialog")
                .outline()
                .label("Kaydırılabilir İletişim Kutusu")
                .on_click(cx.listener(move |_, _, window, cx| {
                    window.open_dialog(cx, move |dialog, _, _| {
                        dialog
                            .w(px(720.))
                            .h(px(600.))
                            .overlay(dialog_overlay)
                            .overlay_closable(overlay_closable)
                            .title("Kaydırma çubuklu iletişim kutusu")
                            .child(markdown(include_str!("../../../../README.md")))
                            .footer(
                                IletisimAltligi::new()
                                    .gap_2()
                                    .child(
                                        IletisimKapat::new()
                                            .child(Dugme::new("cancel").label("İptal").outline()),
                                    )
                                    .child(
                                        IletisimEylemi::new()
                                            .child(Dugme::new("confirm").label("Onayla").primary()),
                                    ),
                            )
                    });
                })),
        )
    }

    fn render_table_in_dialog(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_overlay = self.dialog_overlay;
        let overlay_closable = self.overlay_closable;

        section("Tablo in IletisimKutusu").child(
            Dugme::new("table-dialog")
                .outline()
                .label("Tablo İletişim Kutusu")
                .on_click(cx.listener({
                    move |this, _, window, cx| {
                        window.open_dialog(cx, {
                            let table = this.table.clone();
                            move |dialog, _, _| {
                                dialog
                                    .w(px(800.))
                                    .h(px(600.))
                                    .overlay(dialog_overlay)
                                    .overlay_closable(overlay_closable)
                                    .title("Tablolu İletişim Kutusu")
                                    .child(
                                        v_flex()
                                            .size_full()
                                            .gap_3()
                                            .child("Bu iletişim kutusu bir tablo bileşeni içerir.")
                                            .child(VeriTablosu::new(&table)),
                                    )
                            }
                        });
                    }
                })),
        )
    }

    fn render_custom_paddings(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section("Özel Dolgular").child(
            Dugme::new("custom-dialog-paddings")
                .outline()
                .label("Özel Dolgular")
                .on_click(cx.listener(move |_, _, window, cx| {
                    window.open_dialog(cx, move |dialog, _, _| {
                        dialog.p_3().title("Özel İletişim Kutusu Başlığı").child(
                            "Bu özel iletişim kutusu içeriğidir; dolgu değerleriyle \
                            iletişim kutusunun yerleşimini ve boşluklarını \
                            kontrol edebiliriz.",
                        )
                    });
                })),
        )
    }

    fn render_custom_style(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section("Custom Style").child(
            Dugme::new("custom-dialog-style")
                .outline()
                .label("Özel İletişim Kutusu Stili")
                .on_click(cx.listener(move |_, _, window, cx| {
                    window.open_dialog(cx, move |dialog, _, cx| {
                        dialog
                            .rounded(cx.theme().radius_lg)
                            .bg(cx.theme().cyan)
                            .text_color(cx.theme().info_foreground)
                            .title("Özel İletişim Kutusu Başlığı")
                            .child("Bu özel iletişim kutusu içeriğidir.")
                    });
                })),
        )
    }

    fn render_dialog_with_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        section("Open IletisimKutusu with IletisimIcerigi")
            .sub_title("Declarative API")
            .child(
                Dugme::new("custom-width-dialog-btn")
                    .outline()
                    .label("Özel Genişlik (400px)")
                    .on_click(cx.listener(move |_, _, window, cx| {
                        window.open_dialog(cx, move |dialog, _, _| {
                            dialog.w(px(400.)).content(|content, _, _| {
                                content
                                    .child(
                                        IletisimBasligi::new()
                                            .child(
                                                IletisimBaslikMetni::new().child("Özel Genişlik"),
                                            )
                                            .child(IletisimAciklamasi::new().child(
                                                "Bu iletişim kutusunun özel genişliği 400px.",
                                            )),
                                    )
                                    .child(
                                        "Özel genişlik yapılandırmasına sahip içerik alanı; \
                                            alt bölümde düğmeler flex 1 genişliği kullanır.",
                                    )
                                    .child(
                                        IletisimAltligi::new()
                                            .justify_center()
                                            .child(
                                                Dugme::new("cancel")
                                                    .flex_1()
                                                    .outline()
                                                    .label("İptal")
                                                    .on_click(|_, window, cx| {
                                                        window.close_dialog(cx);
                                                    }),
                                            )
                                            .child(
                                                Dugme::new("done")
                                                    .flex_1()
                                                    .primary()
                                                    .label("Bitti")
                                                    .on_click(|_, window, cx| {
                                                        window.close_dialog(cx);
                                                    }),
                                            ),
                                    )
                            })
                        })
                    })),
            )
    }
}

impl Focusable for DialogStory {
    fn focus_handle(&self, _cx: &kavis_ui::ham_gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DialogStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("dialog-story")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_action_test_action))
            .size_full()
            .child(
                v_flex()
                    .gap_6()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_3()
                            .child(
                                OnayKutusu::new("dialog-overlay")
                                    .label("İletişim Kutusu Kaplaması")
                                    .checked(self.dialog_overlay)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.dialog_overlay = !view.dialog_overlay;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                OnayKutusu::new("overlay-closable")
                                    .label("Kaplama Kapatılabilir")
                                    .checked(self.overlay_closable)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.overlay_closable = !view.overlay_closable;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                OnayKutusu::new("dialog-show-close")
                                    .label("Modal Kapatma Düğmesi")
                                    .checked(self.close_button)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.close_button = !view.close_button;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                OnayKutusu::new("dialog-keyboard")
                                    .label("Klavye")
                                    .checked(self.keyboard)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.keyboard = !view.keyboard;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(self.render_basic_dialog(cx))
                    .child(self.render_focus_back_test(cx))
                    .child(self.render_custom_buttons(cx))
                    .child(self.render_scrollable_dialog(cx))
                    .child(self.render_table_in_dialog(cx))
                    .child(self.render_dialog_without_title(cx))
                    .child(self.render_custom_paddings(cx))
                    .child(self.render_custom_style(cx))
                    .child(self.render_dialog_with_content(cx)),
            )
    }
}
