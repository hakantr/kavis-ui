use gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, SharedString,
    Styled, Window,
};

use kavis_ui::{
    PencereUzantisi,
    clipboard::Pano,
    h_flex,
    input::{Girdi, GirdiDurumu},
    label::Etiket,
    v_flex,
};

use crate::section;

pub struct ClipboardStory {
    focus_handle: gpui::FocusHandle,
    url_state: Entity<GirdiDurumu>,
    masked: bool,
}

impl super::Story for ClipboardStory {
    fn title() -> &'static str {
        "Pano"
    }

    fn description() -> &'static str {
        "A button that helps you copy text or other content to your clipboard."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl ClipboardStory {
    pub(crate) fn new(window: &mut Window, cx: &mut App) -> Self {
        let url_state =
            cx.new(|cx| GirdiDurumu::new(window, cx).default_value("https://github.com"));

        Self {
            url_state,
            focus_handle: cx.focus_handle(),
            masked: false,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}
impl Focusable for ClipboardStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}
impl Render for ClipboardStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .justify_start()
            .gap_3()
            .child(
                section("Pano").max_w_md().child(
                    h_flex()
                        .gap_2()
                        .child(Etiket::new("A clipboard button"))
                        .child(
                            Pano::new("clipboard1")
                                .value_fn({
                                    let view = cx.entity().clone();
                                    move |_, cx| {
                                        SharedString::from(format!(
                                            "masked :{}",
                                            view.read(cx).masked
                                        ))
                                    }
                                })
                                .on_copied(|value, window, cx| {
                                    window.push_notification(
                                        format!("Değer kopyalandı: {}", value),
                                        cx,
                                    )
                                }),
                        ),
                ),
            )
            .child(
                section("With in an Girdi").max_w_md().child(
                    Girdi::new(&self.url_state).suffix(
                        Pano::new("clipboard2")
                            .value_fn({
                                let state = self.url_state.clone();
                                move |_, cx| state.read(cx).value()
                            })
                            .on_copied(|value, window, cx| {
                                window.push_notification(format!("Değer kopyalandı: {}", value), cx)
                            }),
                    ),
                ),
            )
    }
}
