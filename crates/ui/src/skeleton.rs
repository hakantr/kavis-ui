use crate::{EtkinTema, StilUzantisi};
use crate::ham_gpui::{
    Animation, AnimationExt, IntoElement, RenderOnce, StyleRefinement, Styled, bounce, div,
    ease_in_out,
};
use instant::Duration;

/// Bir skeleton yükleme yer tutucu öğe.
#[derive(IntoElement)]
pub struct Iskelet {
    style: StyleRefinement,
    secondary: bool,
}

impl Iskelet {
    /// Yeni bir Iskelet öğe oluşturur.
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            secondary: false,
        }
    }

    /// kullanım ikincil renk ayarlar.
    pub fn secondary(mut self) -> Self {
        self.secondary = true;
        self
    }
}

impl Styled for Iskelet {
    fn style(&mut self) -> &mut crate::ham_gpui::StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Iskelet {
    fn render(self, _: &mut crate::ham_gpui::Window, cx: &mut crate::ham_gpui::App) -> impl IntoElement {
        div()
            .w_full()
            .h_4()
            .bg(if self.secondary {
                cx.theme().skeleton.opacity(0.5)
            } else {
                cx.theme().skeleton
            })
            .refine_style(&self.style)
            .with_animation(
                "skeleton",
                Animation::new(Duration::from_secs(2))
                    .repeat()
                    .with_easing(bounce(ease_in_out)),
                move |this, delta| {
                    let v = 1.0 - delta * 0.5;
                    this.opacity(v)
                },
            )
    }
}
