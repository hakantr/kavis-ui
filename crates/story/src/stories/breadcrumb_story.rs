use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Window, prelude::FluentBuilder as _,
};

use kavis_ui::{
    breadcrumb::{GezintiYolu, GezintiYoluOgesi},
    v_flex,
};

use crate::section;

pub struct BreadcrumbStory {
    focus_handle: kavis_ui::ham_gpui::FocusHandle,
    clicked_item: Option<String>,
}

impl BreadcrumbStory {
    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            clicked_item: None,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl super::Story for BreadcrumbStory {
    fn title() -> &'static str {
        "GezintiYolu"
    }

    fn description() -> &'static str {
        "A breadcrumb navigation element that shows the current location in a hierarchy."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl Focusable for BreadcrumbStory {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for BreadcrumbStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_6()
            .child(
                section("Basic GezintiYolu").max_w_md().child(
                    GezintiYolu::new()
                        .child("Ana Sayfa")
                        .child("Belgeler")
                        .child("Projeler"),
                ),
            )
            .child(
                section("Click Handlers").max_w_md().child(
                    v_flex()
                        .gap_4()
                        .items_center()
                        .child(
                            GezintiYolu::new()
                                .child("Ana Sayfa")
                                .child(GezintiYoluOgesi::new("Belgeler").on_click(cx.listener(
                                    |this, _, _, cx| {
                                        this.clicked_item = Some("Belgeler".to_string());
                                        cx.notify();
                                    },
                                )))
                                .child(GezintiYoluOgesi::new("Projeler").on_click(cx.listener(
                                    |this, _, _, cx| {
                                        this.clicked_item = Some("Projeler".to_string());
                                        cx.notify();
                                    },
                                )))
                                .child(GezintiYoluOgesi::new("Geçerli").on_click(cx.listener(
                                    |this, _, _, cx| {
                                        this.clicked_item = Some("Geçerli".to_string());
                                        cx.notify();
                                    },
                                ))),
                        )
                        .when_some(self.clicked_item.clone(), |this, item| {
                            this.child(format!("Tıklandı: {}", item))
                        }),
                ),
            )
    }
}
