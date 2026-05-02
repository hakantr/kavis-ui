use crate::section;
use gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, Styled,
    Window, px,
};
use kavis_ui::{EtkinTema, divider::Ayirici, h_flex, label::Etiket, v_flex};

const DESCRIPTION: &str = "Kavis UI is a Rust GUI components for building fantastic cross-platform desktop application by using GPUI.";

pub struct DividerStory {
    focus_handle: gpui::FocusHandle,
}

impl super::Story for DividerStory {
    fn title() -> &'static str {
        "Ayirici"
    }

    fn description() -> &'static str {
        "A divider that can be either vertical or horizontal."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl DividerStory {
    pub fn view(_window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
        })
    }
}

impl Focusable for DividerStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DividerStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .child(
                section("Horizontal Dividers").child(
                    v_flex()
                        .gap_4()
                        .w_full()
                        .mt_4()
                        .child(Ayirici::horizontal())
                        .child(Ayirici::horizontal().label("With Etiket"))
                        .child(Ayirici::horizontal_dashed())
                        .child(Ayirici::horizontal_dashed().label("Dashed With Etiket")),
                ),
            )
            .child(
                section("Vertical Dividers").child(
                    h_flex()
                        .gap_4()
                        .h(px(100.))
                        .child(Ayirici::vertical())
                        .child(Ayirici::vertical().label("Solid"))
                        .child(Ayirici::vertical_dashed())
                        .child(Ayirici::vertical_dashed().label("Dashed")),
                ),
            )
            .child(
                section("Combination Dividers").child(
                    v_flex()
                        .gap_y_4()
                        .child(
                            v_flex().gap_y_2().child("Hello Kavis UI").child(
                                Etiket::new(DESCRIPTION)
                                    .text_color(cx.theme().muted_foreground)
                                    .text_sm(),
                            ),
                        )
                        .child(Ayirici::horizontal())
                        .child(
                            h_flex()
                                .gap_x_4()
                                .child("Docs")
                                .child(Ayirici::vertical().dashed())
                                .child("Github")
                                .child(Ayirici::vertical().dashed())
                                .child("Source"),
                        ),
                ),
            )
    }
}
