use gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, Styled,
    Window, px,
};
use kavis_ui::{EtkinTema as _, SimgeAdi, Sizable, spinner::DonerGosterge, v_flex};

use crate::section;

pub struct SpinnerStory {
    focus_handle: gpui::FocusHandle,
    value: f32,
}

impl super::Story for SpinnerStory {
    fn title() -> &'static str {
        "DonerGosterge"
    }

    fn description() -> &'static str {
        "Displays an spinner showing the completion progress of a task."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl SpinnerStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            value: 50.,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

impl Focusable for SpinnerStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SpinnerStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_3()
            .child(
                section("DonerGosterge")
                    .gap_x_2()
                    .child(DonerGosterge::new()),
            )
            .child(
                section("DonerGosterge with color")
                    .gap_x_2()
                    .child(DonerGosterge::new().color(cx.theme().blue))
                    .child(DonerGosterge::new().color(cx.theme().green)),
            )
            .child(
                section("DonerGosterge with size")
                    .gap_x_2()
                    .child(DonerGosterge::new().with_size(px(64.)))
                    .child(DonerGosterge::new().large())
                    .child(DonerGosterge::new())
                    .child(DonerGosterge::new().small())
                    .child(DonerGosterge::new().xsmall()),
            )
            .child(
                section("DonerGosterge with Simge")
                    .gap_x_2()
                    .child(DonerGosterge::new().icon(SimgeAdi::LoaderCircle))
                    .child(
                        DonerGosterge::new()
                            .icon(SimgeAdi::LoaderCircle)
                            .large()
                            .color(cx.theme().cyan),
                    ),
            )
    }
}
