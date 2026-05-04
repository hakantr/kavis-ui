use gpui::{
    App, AppContext, Axis, Context, Entity, FocusHandle, Focusable, InteractiveElement,
    IntoElement, ParentElement as _, Render, Styled, Window, div, prelude::FluentBuilder as _, px,
};
use kavis_ui::{
    BilesenBoyutu, Boyutlandirilabilir, EksenUzantisi, EtkinTema, IndexPath, Secilebilir,
    button::{Dugme, DugmeGrubu},
    checkbox::OnayKutusu,
    color_picker::{RenkSecici, RenkSeciciDurumu},
    date_picker::{TarihSecici, TarihSeciciDurumu},
    divider::Ayirici,
    form::{field, v_form},
    h_flex,
    input::{Girdi, GirdiDurumu},
    select::{Secim, SecimDurumu},
    switch::Anahtar,
    v_flex,
};

pub struct FormStory {
    focus_handle: FocusHandle,
    name_prefix_state: Entity<SecimDurumu<Vec<String>>>,
    name_input: Entity<GirdiDurumu>,
    email_input: Entity<GirdiDurumu>,
    bio_input: Entity<GirdiDurumu>,
    color_state: Entity<RenkSeciciDurumu>,
    subscribe_email: bool,
    date: Entity<TarihSeciciDurumu>,
    layout: Axis,
    size: BilesenBoyutu,
    columns: usize,
}

impl super::Story for FormStory {
    fn title() -> &'static str {
        "Form"
    }

    fn description() -> &'static str {
        "Form to collect multiple inputs."
    }

    fn closable() -> bool {
        false
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl FormStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name_prefix_state = cx.new(|cx| {
            SecimDurumu::new(
                vec![
                    "Mr.".to_string(),
                    "Mrs.".to_string(),
                    "Ms.".to_string(),
                    "Dr.".to_string(),
                ],
                Some(IndexPath::default()),
                window,
                cx,
            )
        });

        let name_input = cx.new(|cx| GirdiDurumu::new(window, cx).default_value("Jason Lee"));
        let color_state = cx.new(|cx| RenkSeciciDurumu::new(window, cx));

        let email_input =
            cx.new(|cx| GirdiDurumu::new(window, cx).placeholder("Buraya metin gir..."));
        let bio_input = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .auto_grow(5, 20)
                .placeholder("Buraya metin gir...")
                .default_value("Hello 世界，this is GPUI component.")
        });
        let date = cx.new(|cx| TarihSeciciDurumu::new(window, cx));

        Self {
            focus_handle: cx.focus_handle(),
            name_prefix_state,
            name_input,
            email_input,
            bio_input,
            date,
            color_state,
            subscribe_email: false,
            layout: Axis::Vertical,
            size: BilesenBoyutu::default(),
            columns: 1,
        }
    }
}

impl Focusable for FormStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for FormStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_multi_column = self.columns > 1;
        let is_horizontal = self.layout.yatay_mi();

        v_flex()
            .id("form-story")
            .size_full()
            .p_4()
            .justify_start()
            .gap_3()
            .child(
                h_flex()
                    .gap_3()
                    .flex_wrap()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_x_3()
                            .child(
                                Anahtar::new("layout")
                                    .checked(self.layout.yatay_mi())
                                    .label("Yatay")
                                    .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                        if *checked {
                                            this.layout = Axis::Horizontal;
                                        } else {
                                            this.layout = Axis::Vertical;
                                        }
                                        cx.notify();
                                    })),
                            )
                            .child(
                                Anahtar::new("column")
                                    .checked(self.columns > 1)
                                    .label("Çoklu Sütun")
                                    .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                        if *checked {
                                            this.columns = 2;
                                        } else {
                                            this.columns = 1;
                                        }
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        DugmeGrubu::new("size")
                            .outline()
                            .small()
                            .child(
                                Dugme::new("large")
                                    .selected(self.size == BilesenBoyutu::Buyuk)
                                    .child("Büyük"),
                            )
                            .child(
                                Dugme::new("medium")
                                    .child("Orta")
                                    .selected(self.size == BilesenBoyutu::Orta),
                            )
                            .child(
                                Dugme::new("small")
                                    .child("Küçük")
                                    .selected(self.size == BilesenBoyutu::Kucuk),
                            )
                            .on_click(cx.listener(|this, selecteds: &Vec<usize>, _, cx| {
                                if selecteds.contains(&0) {
                                    this.size = BilesenBoyutu::Buyuk;
                                } else if selecteds.contains(&1) {
                                    this.size = BilesenBoyutu::Orta;
                                } else if selecteds.contains(&2) {
                                    this.size = BilesenBoyutu::Kucuk;
                                }
                                cx.notify();
                            })),
                    ),
            )
            .child(Ayirici::horizontal())
            .child(
                v_form()
                    .layout(self.layout)
                    .with_size(self.size)
                    .columns(self.columns)
                    .label_width(px(if is_multi_column { 100. } else { 140. }))
                    .child(
                        field().label_fn(|_, _| "Ad").child(
                            h_flex()
                                .gap_2()
                                .border_1()
                                .border_color(cx.theme().input)
                                .bg(cx.theme().input_background())
                                .rounded(cx.theme().radius)
                                .child(div().w(px(90.)).child(
                                    Secim::new(&self.name_prefix_state).pr_0().appearance(false),
                                ))
                                .child(
                                    div().flex_1().child(
                                        Girdi::new(&self.name_input).pl_0().appearance(false),
                                    ),
                                ),
                        ),
                    )
                    .child(
                        field()
                            .label("E-posta")
                            .child(Girdi::new(&self.email_input))
                            .required(true),
                    )
                    .child(
                        field()
                            .label("Biyografi")
                            .when(self.layout.dikey_mi(), |this| this.items_start())
                            .child(Girdi::new(&self.bio_input))
                            .description_fn(|_, _| {
                                div().child("Kendinizi anlatmak için en fazla 100 kelime kullanın.")
                            }),
                    )
                    .child(
                        field()
                            .label_indent(false)
                            .when(is_multi_column, |this| this.col_span(2))
                            .child("Bu tam genişlikte bir form alanıdır."),
                    )
                    .child(
                        field()
                            .label("Lütfen doğum gününüzü seçin")
                            .description("Doğum gününüzü seçin, size bir hediye göndereceğiz.")
                            .child(TarihSecici::new(&self.date)),
                    )
                    .child(
                        field()
                            .when(is_horizontal && is_multi_column, |this| {
                                this.label_indent(false)
                            })
                            .when(is_multi_column, |this| this.col_start(1))
                            .child(
                                Anahtar::new("subscribe-newsletter")
                                    .label("Bültenimize abone ol")
                                    .checked(self.subscribe_email)
                                    .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                        this.subscribe_email = *checked;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        field()
                            .when(is_horizontal && is_multi_column, |this| {
                                this.label_indent(false)
                            })
                            .child(
                                RenkSecici::new(&self.color_state)
                                    .small()
                                    .label("Tema rengi"),
                            ),
                    )
                    .child(
                        field()
                            .when(is_horizontal && is_multi_column, |this| {
                                this.label_indent(false)
                            })
                            .child(
                                OnayKutusu::new("use-vertical-layout")
                                    .label("Dikey yerleşim")
                                    .checked(self.layout.dikey_mi())
                                    .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                        this.layout = if *checked {
                                            Axis::Vertical
                                        } else {
                                            Axis::Horizontal
                                        };
                                        cx.notify();
                                    })),
                            ),
                    ),
            )
    }
}
