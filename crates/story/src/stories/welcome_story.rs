use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, Render, Styled as _, Window, px,
};

use kavis_ui::{dock::PanelDenetimi, text::markdown};

use crate::Story;

pub struct WelcomeStory {
    focus_handle: FocusHandle,
}

impl WelcomeStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Story for WelcomeStory {
    fn title() -> &'static str {
        "Giriş"
    }

    fn description() -> &'static str {
        "GPUI ile güçlü masaüstü uygulamaları oluşturmak için UI bileşenleri."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn zoomable() -> Option<PanelDenetimi> {
        None
    }

    fn paddings() -> kavis_ui::ham_gpui::Pixels {
        px(0.)
    }
}

impl Focusable for WelcomeStory {
    fn focus_handle(&self, _: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for WelcomeStory {
    fn render(
        &mut self,
        _: &mut kavis_ui::ham_gpui::Window,
        _: &mut kavis_ui::ham_gpui::Context<Self>,
    ) -> impl kavis_ui::ham_gpui::IntoElement {
        markdown(include_str!("../../../../README.md"))
            .px_4()
            .scrollable(true)
            .selectable(true)
    }
}
