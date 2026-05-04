use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement as _,
    Render, Styled as _, Window, prelude::FluentBuilder as _,
};
use kavis_ui::{
    BilesenBoyutu, Boyutlandirilabilir, Secilebilir, SimgeAdi,
    accordion::Akordeon,
    button::{Dugme, DugmeGrubu},
    checkbox::OnayKutusu,
    h_flex,
    switch::Anahtar,
    v_flex,
};

use crate::section;

pub struct AccordionStory {
    open_ixs: Vec<usize>,
    size: BilesenBoyutu,
    bordered: bool,
    disabled: bool,
    multiple: bool,
    show_icon: bool,
    focus_handle: FocusHandle,
}

impl super::Story for AccordionStory {
    fn title() -> &'static str {
        "Akordeon"
    }

    fn description() -> &'static str {
        "The accordion uses collapse internally to make it collapsible."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl AccordionStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            bordered: false,
            open_ixs: vec![0, 1, 2],
            size: BilesenBoyutu::default(),
            disabled: false,
            multiple: true,
            show_icon: false,
            focus_handle: cx.focus_handle(),
        }
    }

    fn toggle_accordion(&mut self, open_ixs: Vec<usize>, _: &mut Window, cx: &mut Context<Self>) {
        self.open_ixs = open_ixs;
        cx.notify();
    }

    fn set_size(&mut self, size: BilesenBoyutu, _: &mut Window, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }
}

impl Focusable for AccordionStory {
    fn focus_handle(&self, _: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AccordionStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_5()
            .child(
                h_flex()
                    .items_center()
                    .justify_between()
                    .gap_4()
                    .flex_wrap()
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
                        h_flex()
                            .gap_2()
                            .child(
                                OnayKutusu::new("multiple")
                                    .label("Çoklu")
                                    .checked(self.multiple)
                                    .on_click(cx.listener(|this, checked, _, cx| {
                                        this.multiple = *checked;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                OnayKutusu::new("show_icon")
                                    .label("Simge")
                                    .checked(self.show_icon)
                                    .on_click(cx.listener(|this, checked, _, cx| {
                                        this.show_icon = *checked;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                OnayKutusu::new("disabled")
                                    .label("Devre Dışı")
                                    .checked(self.disabled)
                                    .on_click(cx.listener(|this, checked, _, cx| {
                                        this.disabled = *checked;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                OnayKutusu::new("bordered")
                                    .label("Kenarlıklı")
                                    .checked(self.bordered)
                                    .on_click(cx.listener(|this, checked, _, cx| {
                                        this.bordered = *checked;
                                        cx.notify();
                                    })),
                            ),
                    ),
            )
            .child(
                section("Normal").max_w_md().child(
                    Akordeon::new("test")
                        .bordered(self.bordered)
                        .with_size(self.size)
                        .disabled(self.disabled)
                        .multiple(self.multiple)
                        .item(|this| {
                            this.open(self.open_ixs.contains(&0))
                                .when(self.show_icon, |this| this.icon(SimgeAdi::Info))
                                .title("Erişilebilir mi?")
                                .child("Evet. WAI-ARIA tasarım desenine uyar.")
                        })
                        .item(|this| {
                            this.open(self.open_ixs.contains(&1))
                            .when(self.show_icon, |this| this.icon(SimgeAdi::Inbox))
                            .title("Karmaşık öğelerle stillendirilebilir mi?")
                            .child(
                                v_flex()
                                    .gap_4()
                                    .child(
                                        "Buraya metin görünümü içeren bir v_flex gibi herhangi bir görünüm koyabiliriz.",
                                    )
                                    .child(
                                        h_flex()
                                            .gap_4()
                                            .child(Anahtar::new("switch1").label("Anahtar"))
                                            .child(
                                                OnayKutusu::new("checkbox1").label("Veya Onay Kutusu"),
                                            ),
                                    ),
                            )
                        })
                        .item(|this| {
                            this.open(self.open_ixs.contains(&2))
                                .when(self.show_icon, |this| this.icon(SimgeAdi::Moon))
                                .title("Bu üçüncü akordeondur")
                                .child(
                                    "Bu üçüncü akordeon içeriğidir. \
                                Metin görünümü veya düğme gibi herhangi bir görünüm olabilir.",
                                )
                        })
                        .on_toggle_click(cx.listener(|this, open_ixs: &[usize], window, cx| {
                            this.toggle_accordion(open_ixs.to_vec(), window, cx);
                        })),
                ),
            )
    }
}
