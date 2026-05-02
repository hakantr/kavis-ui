use gpui::{
    App, AppContext, Context, Div, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, SharedString, Styled, Window, px,
};

use kavis_ui::{
    Disableable as _, EtkinTema, Sizable, h_flex, label::Etiket, switch::Anahtar, v_flex,
};

use crate::section;

pub struct SwitchStory {
    focus_handle: FocusHandle,
    switch1: bool,
    switch2: bool,
    switch3: bool,
    switch4: bool,
    switch5: bool,
}

impl super::Story for SwitchStory {
    fn title() -> &'static str {
        "Anahtar"
    }

    fn description() -> &'static str {
        "A control that allows the user to toggle between checked and not checked."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl SwitchStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            switch1: true,
            switch2: false,
            switch3: true,
            switch4: true,
            switch5: false,
        }
    }
}

impl Focusable for SwitchStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SwitchStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        fn title(title: impl Into<SharedString>) -> Div {
            v_flex()
                .flex_1()
                .gap_2()
                .child(Etiket::new(title).text_xl())
        }

        fn card(cx: &Context<SwitchStory>) -> Div {
            h_flex()
                .items_center()
                .gap_4()
                .p_4()
                .w_full()
                .rounded(cx.theme().radius)
                .border_1()
                .border_color(cx.theme().border)
        }

        v_flex()
            .w_full()
            .gap_3()
            .child(
                card(cx)
                    .child(
                        title("Marketing emails").child(
                            Etiket::new("Receive emails about new products, features, and more.")
                                .text_color(theme.muted_foreground),
                        ),
                    )
                    .child(
                        h_flex().gap_2().child("Abone ol").child(
                            Anahtar::new("switch1")
                                .checked(self.switch1)
                                .on_click(cx.listener(move |view, checked, _, cx| {
                                    view.switch1 = *checked;
                                    cx.notify();
                                })),
                        ),
                    ),
            )
            .child(
                card(cx)
                    .child(
                        title("Security emails").child(
                            Etiket::new(
                                "Receive emails about your account security. \
                                    When turn off, you never receive email again.",
                            )
                            .text_color(theme.muted_foreground),
                        ),
                    )
                    .child(
                        Anahtar::new("switch2")
                            .checked(self.switch2)
                            .on_click(cx.listener(move |view, checked, _, cx| {
                                view.switch2 = *checked;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                section("Devre Dışı")
                    .child(Anahtar::new("switch3").disabled(true).on_click(|v, _, _| {
                        println!("Anahtar değeri değişti: {:?}", v);
                    }))
                    .child(
                        Anahtar::new("switch3_1")
                            .w(px(200.))
                            .label("Uçak Modu")
                            .checked(true)
                            .disabled(true)
                            .on_click(|ev, _, _| {
                                println!("Anahtar değeri değişti: {:?}", ev);
                            }),
                    ),
            )
            .child(
                section("Custom Color").child(
                    h_flex()
                        .gap_4()
                        .child(
                            Anahtar::new("switch4")
                                .checked(self.switch4)
                                .label("Başarılı")
                                .color(theme.success)
                                .on_click(cx.listener(|view, checked, _, cx| {
                                    view.switch4 = *checked;
                                    cx.notify();
                                })),
                        )
                        .child(
                            Anahtar::new("switch5")
                                .checked(self.switch5)
                                .label("Yıkıcı")
                                .color(theme.danger)
                                .on_click(cx.listener(|view, checked, _, cx| {
                                    view.switch5 = *checked;
                                    cx.notify();
                                })),
                        )
                        .child(
                            Anahtar::new("switch4_disabled")
                                .checked(true)
                                .label("Devre Dışı")
                                .color(theme.success)
                                .disabled(true),
                        ),
                ),
            )
            .child(
                section("Küçük Boyut").child(
                    Anahtar::new("switch3")
                        .checked(self.switch3)
                        .label("Küçük Boyut")
                        .small()
                        .on_click(cx.listener(move |view, checked, _, cx| {
                            view.switch3 = *checked;
                            cx.notify();
                        })),
                ),
            )
    }
}
