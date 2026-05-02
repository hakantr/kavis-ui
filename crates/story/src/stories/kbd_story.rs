use gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, Keystroke, ParentElement, Render,
    Styled, Window,
};

use kavis_ui::{h_flex, kbd::KlavyeTusu, v_flex};

use crate::section;

pub struct KbdStory {
    focus_handle: gpui::FocusHandle,
}

impl super::Story for KbdStory {
    fn title() -> &'static str {
        "KlavyeTusu"
    }

    fn description() -> &'static str {
        "A tag style to display keyboard shortcuts"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl KbdStory {
    pub(crate) fn new(_: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}
impl Focusable for KbdStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for KbdStory {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .child(
                section("KlavyeTusu").child(
                    h_flex()
                        .gap_2()
                        .child(KlavyeTusu::new(Keystroke::parse("cmd-shift-p").unwrap()))
                        .child(KlavyeTusu::new(Keystroke::parse("cmd-ctrl-t").unwrap()))
                        .child(KlavyeTusu::new(Keystroke::parse("cmd--").unwrap()))
                        .child(KlavyeTusu::new(Keystroke::parse("cmd-+").unwrap()))
                        .child(KlavyeTusu::new(Keystroke::parse("escape").unwrap()))
                        .child(KlavyeTusu::new(Keystroke::parse("backspace").unwrap()))
                        .child(KlavyeTusu::new(Keystroke::parse("/").unwrap()))
                        .child(KlavyeTusu::new(Keystroke::parse("enter").unwrap())),
                ),
            )
            .child(
                section("Outline Style").child(
                    h_flex()
                        .gap_2()
                        .child(KlavyeTusu::new(Keystroke::parse("cmd-shift-p").unwrap()).outline())
                        .child(KlavyeTusu::new(Keystroke::parse("cmd-ctrl-t").unwrap()).outline())
                        .child(KlavyeTusu::new(Keystroke::parse("enter").unwrap()).outline()),
                ),
            )
    }
}
