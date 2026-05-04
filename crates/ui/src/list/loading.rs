use super::ListeOgesi;
use crate::ham_gpui::{IntoElement, ParentElement as _, RenderOnce, Styled};
use crate::{skeleton::Iskelet, v_flex};

#[derive(IntoElement)]
pub struct Loading;

#[derive(IntoElement)]
struct LoadingItem;

impl RenderOnce for LoadingItem {
    fn render(
        self,
        _window: &mut crate::ham_gpui::Window,
        _cx: &mut crate::ham_gpui::App,
    ) -> impl IntoElement {
        ListeOgesi::new("skeleton").disabled(true).child(
            v_flex()
                .gap_1p5()
                .overflow_hidden()
                .child(Iskelet::new().h_5().w_48().max_w_full())
                .child(Iskelet::new().secondary().h_3().w_64().max_w_full()),
        )
    }
}

impl RenderOnce for Loading {
    fn render(
        self,
        _window: &mut crate::ham_gpui::Window,
        _cx: &mut crate::ham_gpui::App,
    ) -> impl IntoElement {
        v_flex()
            .py_2p5()
            .gap_3()
            .child(LoadingItem)
            .child(LoadingItem)
            .child(LoadingItem)
    }
}
