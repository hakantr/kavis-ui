use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, Styled,
    Window, div, px,
};

use kavis_ui::{
    Boyutlandirilabilir, EtkinTema, h_flex,
    radio::{Radyo, RadyoGrubu},
    v_flex,
};

use crate::section;

pub struct RadioStory {
    focus_handle: kavis_ui::ham_gpui::FocusHandle,
    radio_check1: bool,
    radio_check2: bool,
    radio_group_checked: Option<usize>,
}

impl super::Story for RadioStory {
    fn title() -> &'static str {
        "Radyo"
    }

    fn description() -> &'static str {
        "A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl RadioStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            radio_check1: false,
            radio_check2: true,
            radio_group_checked: Some(1),
        }
    }
}

impl Focusable for RadioStory {
    fn focus_handle(&self, _: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RadioStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .child(
                section("Radyo")
                    .max_w_md()
                    .child(
                        Radyo::new("radio1")
                            .checked(self.radio_check1)
                            .on_click(cx.listener(|this, checked, _, _| {
                                this.radio_check1 = *checked;
                            })),
                    )
                    .child(
                        Radyo::new("radio2")
                            .label("Radyo 2")
                            .checked(self.radio_check2)
                            .on_click(cx.listener(|this, checked, _, _| {
                                this.radio_check2 = *checked;
                            })),
                    ),
            )
            .child(
                section("Devre Dışı")
                    .child(Radyo::new("a").label("Devre Dışı").disabled(true))
                    .child(
                        Radyo::new("b")
                            .label("Seçiliyken Devre Dışı")
                            .checked(true)
                            .disabled(true),
                    ),
            )
            .child(
                section("Multi-line Etiket").child(
                    Radyo::new("radio3")
                        .label("Çok uzun etiket metni.")
                        .child(
                            div()
                                .text_color(cx.theme().muted_foreground)
                                .child("Metin çok uzun olduğunda bu satır sarmalanmalıdır."),
                        )
                        .w(px(300.))
                        .checked(true)
                        .disabled(true),
                ),
            )
            .child(
                section("Sizeable").child(
                    h_flex()
                        .h_full()
                        .gap_x_4()
                        .child(
                            Radyo::new("xsmall")
                                .label("Küçük")
                                .xsmall()
                                .checked(self.radio_check2)
                                .on_click(cx.listener(|this, v, _, _| {
                                    this.radio_check2 = *v;
                                })),
                        )
                        .child(
                            Radyo::new("large")
                                .label("Büyük")
                                .large()
                                .checked(self.radio_check2)
                                .on_click(cx.listener(|this, v, _, _| {
                                    this.radio_check2 = *v;
                                })),
                        ),
                ),
            )
            .child(
                section("Radyo Group").max_w_md().child(
                    v_flex().child(
                        RadyoGrubu::horizontal("radio_group_1")
                            .children(["One", "Two", "Three"])
                            .selected_index(self.radio_group_checked)
                            .on_click(cx.listener(|this, selected_ix: &usize, _, cx| {
                                this.radio_group_checked = Some(*selected_ix);
                                cx.notify();
                            })),
                    ),
                ),
            )
            .child(
                section("Radyo Group Vertical (With container style)")
                    .max_w_md()
                    .child(
                        v_flex().items_center().content_center().child(
                            RadyoGrubu::vertical("radio_group_2")
                                .w(px(220.))
                                .p_2()
                                .border_1()
                                .border_color(cx.theme().border)
                                .rounded(cx.theme().radius)
                                .disabled(true)
                                .child(Radyo::new("one1").label("Amerika Birleşik Devletleri"))
                                .child(Radyo::new("one2").label("Kanada"))
                                .child(Radyo::new("one3").label("Meksika"))
                                .selected_index(Some(1)),
                        ),
                    ),
            )
    }
}
