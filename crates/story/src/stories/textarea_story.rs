use gpui::{
    App, AppContext as _, ClickEvent, Context, Entity, Focusable, IntoElement, ParentElement as _,
    Render, Styled, Window, px,
};

use crate::section;
use kavis_ui::{
    Sizable,
    button::Dugme,
    h_flex,
    input::{Input, InputState},
    v_flex,
};

pub fn init(_: &mut App) {}

pub struct TextareaStory {
    textarea: Entity<InputState>,
    textarea_auto_grow: Entity<InputState>,
    textarea_no_wrap: Entity<InputState>,
    textarea_auto_grow_no_wrap: Entity<InputState>,
}

impl super::Story for TextareaStory {
    fn title() -> &'static str {
        "Textarea"
    }

    fn description() -> &'static str {
        "Input with multi-line mode."
    }

    fn closable() -> bool {
        false
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl TextareaStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let textarea = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(10)
                .placeholder("Metni buraya girin...")
                .searchable(true)
                .default_value(
                    unindent::unindent(
                        r#"Merhaba 世界, bu Kavis UI.

                    Kavis UI, GPUI framework için hazırlanmış UI bileşenleri koleksiyonudur.

                    Dugme, Input, OnayKutusu, Radyo, Dropdown, Sekme ve daha fazlasını içerir...

                    Kavis UI kullanılarak oluşturulmuş bir uygulama örneği aşağıdadır.

                    > Bu uygulama hâlâ geliştirme aşamasındadır, henüz yayımlanmamıştır.

                    ![image](https://github.com/user-attachments/assets/559a648d-19df-4b5a-b563-b78cc79c8894)

                    ![image](https://github.com/user-attachments/assets/5e06ad5d-7ea0-43db-8d13-86a240da4c8d)

                    ## Demo

                    Demoyu görmek isterseniz burada bazı demo uygulamaları var.
                    "#,
                    )
                )
        });

        let textarea_no_wrap = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .rows(6)
                .soft_wrap(false)
                .default_value("Bu, yatay kaydırma işlevinin düzgün çalışıp çalışmadığını test etmek için çok uzun bir metin satırıdır; otomatik olarak sarılmamalı ve yatay kaydırma çubuğu göstermelidir.\nİkinci satır da çok uzun bir metindir; birden fazla satırda yatay kaydırma etkisini test etmek için kullanılır ve test için daha fazla içerik girebilirsiniz.\nÜçüncü satır: Buraya yatay kaydırma gerektiren başka uzun metinler girebilirsiniz.\n")
        });

        let textarea_auto_grow = cx.new(|cx| {
            InputState::new(window, cx)
                .auto_grow(1, 5)
                .placeholder("Metni buraya girin...")
                .default_value(
                    "Merhaba 世界, bu yatay kaydırma işlevinin düzgün çalışıp \
                    çalışmadığını test etmek için çok uzun bir metin satırıdır; \
                    otomatik olarak sarılmamalı ve yatay kaydırma çubuğu \
                    göstermelidir.\n\
                    İkinci satır da çok uzun bir metindir; birden fazla satırda \
                    yatay kaydırma etkisini test etmek için kullanılır ve test \
                    için daha fazla içerik girebilirsiniz.\nÜçüncü satır: Buraya \
                    yatay kaydırma gerektiren başka uzun metinler girebilirsiniz.\n",
                )
        });

        let textarea_auto_grow_no_wrap = cx.new(|cx| {
            InputState::new(window, cx)
                .auto_grow(1, 5)
                .soft_wrap(false)
                .placeholder("Metni buraya girin...")
                .default_value("Merhaba 世界, bu Kavis UI.")
        });

        Self {
            textarea,
            textarea_auto_grow,
            textarea_no_wrap,
            textarea_auto_grow_no_wrap,
        }
    }

    fn on_insert_text_to_textarea(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.textarea.update(cx, |input, cx| {
            input.insert("Hello 你好", window, cx);
        });
    }

    fn on_replace_text_to_textarea(
        &mut self,
        _: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.textarea.update(cx, |input, cx| {
            input.replace("Hello 你好", window, cx);
        });
    }
}

impl Focusable for TextareaStory {
    fn focus_handle(&self, cx: &gpui::App) -> gpui::FocusHandle {
        self.textarea.focus_handle(cx)
    }
}

impl Render for TextareaStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let loc = self.textarea.read(cx).cursor_position();

        v_flex()
            .w_full()
            .gap_3()
            .child(
                section("Textarea").child(
                    v_flex()
                        .gap_2()
                        .w_full()
                        .child(Input::new(&self.textarea).h(px(320.)))
                        .child(
                            h_flex()
                                .justify_between()
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            Dugme::new("btn-insert-text")
                                                .outline()
                                                .xsmall()
                                                .label("Insert Text")
                                                .on_click(
                                                    cx.listener(Self::on_insert_text_to_textarea),
                                                ),
                                        )
                                        .child(
                                            Dugme::new("btn-replace-text")
                                                .outline()
                                                .xsmall()
                                                .label("Replace Text")
                                                .on_click(
                                                    cx.listener(Self::on_replace_text_to_textarea),
                                                ),
                                        ),
                                )
                                .child(format!("{}:{}", loc.line, loc.character)),
                        ),
                ),
            )
            .child(
                section("No Wrap")
                    .max_w_md()
                    .child(Input::new(&self.textarea_no_wrap).h(px(200.))),
            )
            .child(
                section("Auto Grow")
                    .max_w_md()
                    .child(Input::new(&self.textarea_auto_grow)),
            )
            .child(
                section("Auto Grow with No Wrap")
                    .max_w_md()
                    .child(Input::new(&self.textarea_auto_grow_no_wrap)),
            )
    }
}
