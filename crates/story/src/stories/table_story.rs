use gpui::{
    App, AppContext as _, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window, prelude::FluentBuilder as _, px,
};
use kavis_ui::{
    BilesenBoyutu, Boyutlandirilabilir, EtkinTema, Secilebilir as _,
    button::{Dugme, DugmeGrubu},
    h_flex,
    table::{
        Tablo, TabloAciklamasi, TabloAltligi, TabloBasHucre, TabloBasligi, TabloGovdesi,
        TabloHucresi, TabloSatiri,
    },
    tag::Cip,
    v_flex,
};

use crate::section;

pub struct TableStory {
    focus_handle: FocusHandle,
    size: BilesenBoyutu,
}

impl TableStory {
    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            size: BilesenBoyutu::default(),
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

impl super::Story for TableStory {
    fn title() -> &'static str {
        "Tablo"
    }

    fn description() -> &'static str {
        "A basic table component for directly rendering tabular data."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl Focusable for TableStory {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn status_tag(status: &str) -> Cip {
    match status {
        "Ödendi" => Cip::success().outline().child(status.to_string()),
        "Bekliyor" => Cip::warning().outline().child(status.to_string()),
        "Ödenmedi" => Cip::danger().outline().child(status.to_string()),
        _ => Cip::new().child(status.to_string()),
    }
    .xsmall()
}

impl Render for TableStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let invoices: Vec<(&str, &str, &str, &str, &str)> = vec![
            ("INV001", "Ödendi", "Kredi Kartı", "$250.00", "2024-01-15"),
            ("INV002", "Bekliyor", "PayPal", "$150.00", "2024-02-01"),
            (
                "INV003",
                "Ödenmedi",
                "Banka Havalesi",
                "$350.00",
                "2024-02-15",
            ),
            (
                "INV004",
                "Ödendi",
                "Kredi Kartı\nMaster Card / Visa",
                "$450.00",
                "2024-03-01",
            ),
            ("INV005", "Ödendi", "PayPal", "$550.00", "2024-03-15"),
            (
                "INV006",
                "Bekliyor",
                "Banka Havalesi",
                "$200.00",
                "2024-04-01",
            ),
            ("INV007", "Ödenmedi", "Kredi Kartı", "$300.00", "2024-04-15"),
        ];

        v_flex()
            .size_full()
            .gap_6()
            .child(
                h_flex().gap_3().child(
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
                ),
            )
            .child(
                section("Tablo").child(
                    Tablo::new()
                        .with_size(self.size)
                        .child(
                            TabloBasligi::new().child(
                                TabloSatiri::new()
                                    .child(TabloBasHucre::new().w(px(150.)).child("Fatura"))
                                    .child(TabloBasHucre::new().col_span(2).child("Durum"))
                                    .child(TabloBasHucre::new().text_right().child("Tutar"))
                                    .child(TabloBasHucre::new().text_right().child("Tarih")),
                            ),
                        )
                        .child(TabloGovdesi::new().children(invoices.iter().map(
                            |(invoice, status, method, amount, date)| {
                                TabloSatiri::new()
                                    .child(
                                        TabloHucresi::new().w(px(150.)).child(invoice.to_string()),
                                    )
                                    .child(TabloHucresi::new().child(status_tag(status)))
                                    .child(TabloHucresi::new().child(method.to_string()))
                                    .child(
                                        TabloHucresi::new().text_right().child(amount.to_string()),
                                    )
                                    .child(TabloHucresi::new().text_right().child(date.to_string()))
                            },
                        )))
                        .child(
                            TabloAltligi::new().child(
                                TabloSatiri::new()
                                    .child(TabloHucresi::new().col_span(3).child("Toplam"))
                                    .child(
                                        TabloHucresi::new()
                                            .col_span(2)
                                            .text_right()
                                            .child("$2,250.00"),
                                    ),
                            ),
                        )
                        .child(TabloAciklamasi::new().child("Son faturalarınızın listesi.")),
                ),
            )
            .child(
                section("With Border").child(
                    Tablo::new()
                        .with_size(self.size)
                        .border_1()
                        .border_color(cx.theme().border)
                        .rounded(cx.theme().radius)
                        .child(
                            TabloBasligi::new().child(
                                TabloSatiri::new()
                                    .child(TabloBasHucre::new().w(px(100.)).child("Fatura"))
                                    .child(TabloBasHucre::new().child("Yöntem"))
                                    .child(TabloBasHucre::new().text_right().child("Tutar"))
                                    .child(TabloBasHucre::new().text_right().child("Tarih")),
                            ),
                        )
                        .child(TabloGovdesi::new().children(
                            invoices.iter().enumerate().take(6).map(
                                |(ix, (invoice, _, method, amount, date))| {
                                    TabloSatiri::new()
                                        .when(ix % 2 != 0, |this| this.bg(cx.theme().table_even))
                                        .child(
                                            TabloHucresi::new()
                                                .w(px(100.))
                                                .child(invoice.to_string()),
                                        )
                                        .child(TabloHucresi::new().child(method.to_string()))
                                        .child(
                                            TabloHucresi::new()
                                                .text_right()
                                                .child(amount.to_string()),
                                        )
                                        .child(
                                            TabloHucresi::new()
                                                .text_right()
                                                .child(date.to_string()),
                                        )
                                },
                            ),
                        )),
                ),
            )
    }
}
