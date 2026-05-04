use gpui::{
    App, AppContext as _, ClickEvent, Context, Entity, InteractiveElement, IntoElement,
    ParentElement as _, Render, Styled, Subscription, Window, div,
};

use crate::section;
use kavis_ui::{
    Boyutlandirilabilir as _, EtkinTema as _, PencereUzantisi as _, Simge, SimgeAdi,
    button::{Dugme, DugmeVaryantlari as _},
    h_flex,
    input::{self, Girdi, GirdiDurumu, GirdiOlayi, MaskeDeseni},
    v_flex,
};

const CODE_EXAMPLE: &str = r#"{"single_line":"code editor"}"#;

pub fn init(_: &mut App) {}

pub struct InputStory {
    input1: Entity<GirdiDurumu>,
    input2: Entity<GirdiDurumu>,
    input_esc: Entity<GirdiDurumu>,
    input_text_centered: Entity<GirdiDurumu>,
    input_text_right: Entity<GirdiDurumu>,
    mask_input: Entity<GirdiDurumu>,
    disabled_input: Entity<GirdiDurumu>,
    prefix_input1: Entity<GirdiDurumu>,
    suffix_input1: Entity<GirdiDurumu>,
    both_input1: Entity<GirdiDurumu>,
    large_input: Entity<GirdiDurumu>,
    small_input: Entity<GirdiDurumu>,
    phone_input: Entity<GirdiDurumu>,
    mask_input2: Entity<GirdiDurumu>,
    currency_input: Entity<GirdiDurumu>,
    custom_input: Entity<GirdiDurumu>,
    custom_menu_input: Entity<GirdiDurumu>,
    code_input: Entity<GirdiDurumu>,
    color_input: Entity<GirdiDurumu>,

    _subscriptions: Vec<Subscription>,
}

impl super::Story for InputStory {
    fn title() -> &'static str {
        "Girdi"
    }

    fn closable() -> bool {
        false
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl InputStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input1 = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .default_value("Hello 世界，this is GPUI component, this is a long text.")
        });

        let input2 = cx.new(|cx| GirdiDurumu::new(window, cx).placeholder("Buraya metin gir..."));
        let input_esc = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .placeholder("Metin gir ve ESC ile temizle")
                .clean_on_escape()
        });

        let mask_input = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .masked(true)
                .placeholder("Parolanızı girin...")
                .default_value("this-is-password-中文🚀🎉")
        });

        let prefix_input1 = cx.new(|cx| GirdiDurumu::new(window, cx).placeholder("Bir şey ara..."));
        let suffix_input1 = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .placeholder("Bu girdi yalnızca [a-zA-Z0-9] karakterlerini destekler.")
                .pattern(regex::Regex::new(r"^[a-zA-Z0-9]*$").unwrap())
        });
        let both_input1 = cx.new(|cx| {
            GirdiDurumu::new(window, cx).placeholder("Bu girdinin ön eki ve son eki var.")
        });

        let phone_input = cx.new(|cx| GirdiDurumu::new(window, cx).mask_pattern("(999)-999-9999"));
        let mask_input2 = cx.new(|cx| GirdiDurumu::new(window, cx).mask_pattern("AAA-###-AAA"));
        let currency_input = cx.new(|cx| {
            GirdiDurumu::new(window, cx).mask_pattern(MaskeDeseni::localized_number(Some(3)))
        });
        let custom_input = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .placeholder("Özel girdi monospace kullanır, 0123456789.")
                .baglam_menusu(false)
        });

        let custom_menu_input =
            cx.new(|cx| GirdiDurumu::new(window, cx).placeholder("Özel bağlam menülü girdi..."));

        let color_input = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .placeholder("Bir şey yaz...")
                .default_value("Özel metin rengi girdisi")
        });

        let code_input = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .code_editor("json")
                .multi_line(false)
                .show_whitespaces(true)
                .default_value(CODE_EXAMPLE)
        });

        let input_text_centered = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .placeholder("Ortalanmış yerleşimi test etmek için metin gir...")
                .default_value("Ortalanmış Metin")
        });

        let input_text_right = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .placeholder("Sağa hizalı yerleşimi test etmek için metin gir...")
                .default_value("Sağa Hizalı Metin")
        });

        let _subscriptions = vec![
            cx.subscribe_in(&input1, window, Self::on_input_event),
            cx.subscribe_in(&input2, window, Self::on_input_event),
            cx.subscribe_in(&phone_input, window, Self::on_input_event),
        ];

        Self {
            input1,
            input2,
            input_esc,
            mask_input,
            disabled_input: cx
                .new(|cx| GirdiDurumu::new(window, cx).default_value("Bu devre dışı girdidir")),
            large_input: cx.new(|cx| GirdiDurumu::new(window, cx).placeholder("Büyük girdi")),
            small_input: cx.new(|cx| {
                GirdiDurumu::new(window, cx)
                    .validate(|s, _| s.parse::<f32>().is_ok())
                    .placeholder("ondalıklı sayıyı sınırlamak için doğrula.")
            }),
            prefix_input1,
            suffix_input1,
            both_input1,
            phone_input,
            mask_input2,
            currency_input,
            custom_input,
            custom_menu_input,
            code_input,
            color_input,
            input_text_centered,
            input_text_right,
            _subscriptions,
        }
    }

    fn on_input_event(
        &mut self,
        state: &Entity<GirdiDurumu>,
        event: &GirdiOlayi,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            GirdiOlayi::Change => {
                let text = state.read(cx).value();
                if state == &self.input2 {
                    println!("Devre dışı girdi değeri ayarlandı: {}", text);
                    self.disabled_input.update(cx, |this, cx| {
                        this.set_value(text, window, cx);
                    })
                } else {
                    println!("Değişiklik: {}", text)
                }
            }
            GirdiOlayi::PressEnter { secondary } => {
                println!("Enter basıldı, ikincil: {}", secondary)
            }
            GirdiOlayi::Focus => println!("Odaklandı"),
            GirdiOlayi::Blur => println!("Odaktan çıktı"),
        };
    }

    fn on_click_reset(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.code_input.update(cx, |input_state, cx| {
            input_state.set_value(CODE_EXAMPLE, window, cx);
        });
    }
}

impl Render for InputStory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("input-story")
            .size_full()
            .justify_start()
            .gap_3()
            .child(
                section("Normal Girdi")
                    .max_w_md()
                    .child(Girdi::new(&self.input1).cleanable(true))
                    .child(Girdi::new(&self.input2)),
            )
            .child(
                section("Girdi State")
                    .max_w_md()
                    .child(Girdi::new(&self.disabled_input).disabled(true))
                    .child(Girdi::new(&self.mask_input).mask_toggle().cleanable(true)),
            )
            .child(
                section("Text Align").max_w_lg().child(
                    h_flex()
                        .w_full()
                        .gap_4()
                        .flex_wrap()
                        .child(Girdi::new(&self.input_text_centered).text_center().flex_1())
                        .child(Girdi::new(&self.input_text_right).text_right().flex_1()),
                ),
            )
            .child(
                section("Prefix and Suffix")
                    .max_w_md()
                    .child(
                        Girdi::new(&self.prefix_input1)
                            .cleanable(true)
                            .prefix(Simge::new(SimgeAdi::Search).small()),
                    )
                    .child(
                        Girdi::new(&self.both_input1)
                            .cleanable(true)
                            .prefix(div().child(Simge::new(SimgeAdi::Search).small()))
                            .suffix(Dugme::new("info").ghost().icon(SimgeAdi::Info).xsmall()),
                    )
                    .child(
                        Girdi::new(&self.suffix_input1)
                            .cleanable(true)
                            .suffix(Dugme::new("info").ghost().icon(SimgeAdi::Info).xsmall()),
                    ),
            )
            .child(
                section("Currency Girdi with thousands separator")
                    .max_w_md()
                    .child(Girdi::new(&self.currency_input))
                    .child(
                        div().child(format!("Değer: {:?}", self.currency_input.read(cx).value())),
                    ),
            )
            .child(
                section("Girdi with mask pattern: (999)-999-9999")
                    .max_w_md()
                    .child(Girdi::new(&self.phone_input))
                    .child(
                        v_flex()
                            .child(format!("Değer: {:?}", self.phone_input.read(cx).value()))
                            .child(format!(
                                "Maskesiz Değer: {:?}",
                                self.phone_input.read(cx).unmask_value()
                            )),
                    ),
            )
            .child(
                section("Girdi with mask pattern: AAA-###-AAA")
                    .max_w_md()
                    .child(Girdi::new(&self.mask_input2))
                    .child(
                        v_flex()
                            .child(format!("Değer: {:?}", self.mask_input2.read(cx).value()))
                            .child(format!(
                                "Maskesiz Değer: {:?}",
                                self.mask_input2.read(cx).unmask_value()
                            )),
                    ),
            )
            .child(
                section("Girdi Size")
                    .max_w_md()
                    .child(Girdi::new(&self.large_input).large())
                    .child(Girdi::new(&self.small_input).small()),
            )
            .child(
                section("Cleanable and ESC to clean")
                    .max_w_md()
                    .child(Girdi::new(&self.input_esc).cleanable(true)),
            )
            .child(
                section("Focused Girdi")
                    .max_w_md()
                    .whitespace_normal()
                    .overflow_hidden()
                    .child(div().child(format!(
                        "Değer: {:?}",
                        window.focused_input(cx).map(|input| input.read(cx).value())
                    ))),
            )
            .child(
                section("Custom Appearance").max_w_md().child(
                    div()
                        .border_b_2()
                        .px_6()
                        .py_3()
                        .font_family(cx.theme().mono_font_family.clone())
                        .border_color(cx.theme().border)
                        .bg(cx.theme().secondary)
                        .text_color(cx.theme().secondary_foreground)
                        .w_full()
                        .child(Girdi::new(&self.custom_input).appearance(false)),
                ),
            )
            .child(section("Custom Context Menu").max_w_md().child(
                Girdi::new(&self.custom_menu_input).baglam_menusu(|menu, _, _| {
                    menu.menu("Özel Eylem", Box::new(input::SelectAll))
                        .separator()
                        .menu("Kopyala", Box::new(input::Copy))
                        .menu("Yapıştır", Box::new(input::Paste))
                }),
            ))
            .child(
                section("Custom Text Color")
                    .max_w_md()
                    .child(Girdi::new(&self.color_input).text_color(cx.theme().info)),
            )
            .child(
                section("Single line code editor").max_w_md().child(
                    Girdi::new(&self.code_input).suffix(
                        Dugme::new("code-reset")
                            .ghost()
                            .label("Sıfırla")
                            .xsmall()
                            .on_click(cx.listener(Self::on_click_reset)),
                    ),
                ),
            )
    }
}
