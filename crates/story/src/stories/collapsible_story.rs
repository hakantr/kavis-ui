use gpui::div;
use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Window, prelude::FluentBuilder as _,
};

use kavis_ui::group_box::{GrupKutusu, GrupKutusuVaryantlari as _};
use kavis_ui::label::Etiket;
use kavis_ui::tag::Cip;
use kavis_ui::{
    Boyutlandirilabilir,
    button::{Dugme, DugmeVaryantlari},
    collapsible::Daraltilabilir,
    v_flex,
};
use kavis_ui::{EtkinTema, SimgeAdi, StilUzantisi, h_flex};

use crate::section;

pub struct CollapsibleStory {
    focus_handle: FocusHandle,
    item1_open: bool,
    item2_open: bool,
}

impl super::Story for CollapsibleStory {
    fn title() -> &'static str {
        "Daraltilabilir"
    }

    fn description() -> &'static str {
        "An interactive element that expands/collapses."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl CollapsibleStory {
    pub(crate) fn new(_: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            item1_open: false,
            item2_open: false,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Focusable for CollapsibleStory {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CollapsibleStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let items = [
            ["TSLA.US", "$423.00", "+30.25%"],
            ["NVDA.US", "$312.00", "+12.12%"],
            ["AAPL.US", "$145.00", "-8.50%"],
        ];

        v_flex()
            .gap_6()
            .child(
                section("Expland Paragraphs").v_flex().child(
                    Daraltilabilir::new()
                        .max_w_128()
                        .gap_1()
                        .open(self.item1_open)
                        .child(
                            "Bu katlanabilir bir bileşendir. \
            İçeriği genişletmek veya daraltmak için başlığa tıklayın.",
                        )
                        .content(
                            "Bu, Katlanabilir bileşeninin tam içeriğidir. \
                        Yalnızca bileşen genişletildiğinde görünür. \n\
                        Buraya metin, görsel veya başka UI öğeleri dahil \
                        istediğiniz içeriği koyabilirsiniz.
                        ",
                        )
                        .child(
                            h_flex().justify_center().child(
                                Dugme::new("toggle1")
                                    .icon(SimgeAdi::ChevronDown)
                                    .label("Daha fazla göster")
                                    .when(self.item1_open, |this| {
                                        this.icon(SimgeAdi::ChevronUp).label("Daha az göster")
                                    })
                                    .xsmall()
                                    .link()
                                    .on_click({
                                        cx.listener(move |this, _, _, cx| {
                                            this.item1_open = !this.item1_open;
                                            cx.notify();
                                        })
                                    }),
                            ),
                        ),
                ),
            )
            .child(
                section("Card").child(
                    GrupKutusu::new()
                        .outline()
                        .w_80()
                        .title("Kart İçinde Katlanabilir")
                        .child(
                            Daraltilabilir::new()
                                .gap_1()
                                .open(self.item2_open)
                                .child(
                                    h_flex()
                                        .justify_between()
                                        .child(
                                            v_flex().child("Toplam Getiri").child(
                                                h_flex()
                                                    .gap_1()
                                                    .child(
                                                        Etiket::new("123.5%")
                                                            .text_2xl()
                                                            .font_semibold(),
                                                    )
                                                    .child(
                                                        Cip::info()
                                                            .child("+4.5%")
                                                            .outline()
                                                            .rounded_full()
                                                            .small(),
                                                    ),
                                            ),
                                        )
                                        .child(
                                            Dugme::new("toggle2")
                                                .small()
                                                .outline()
                                                .icon(SimgeAdi::ChevronDown)
                                                .label("Ayrıntılar")
                                                .when(self.item2_open, |this| {
                                                    this.icon(SimgeAdi::ChevronUp)
                                                })
                                                .on_click({
                                                    cx.listener(move |this, _, _, cx| {
                                                        this.item2_open = !this.item2_open;
                                                        cx.notify();
                                                    })
                                                }),
                                        ),
                                )
                                .content(v_flex().gap_2().children(items.iter().map(|item| {
                                    let is_up = item[2].starts_with('+');

                                    h_flex().justify_between().child(item[0]).child(
                                        h_flex()
                                            .flex_1()
                                            .justify_end()
                                            .gap_4()
                                            .child(div().w_16().justify_end().child(item[1]))
                                            .child(
                                                Etiket::new(item[2])
                                                    .text_xs()
                                                    .w_16()
                                                    .justify_end()
                                                    .when(is_up, |this| {
                                                        this.text_color(cx.theme().green)
                                                    })
                                                    .when(!is_up, |this| {
                                                        this.text_color(cx.theme().red)
                                                    }),
                                            ),
                                    )
                                }))),
                        ),
                ),
            )
    }
}
