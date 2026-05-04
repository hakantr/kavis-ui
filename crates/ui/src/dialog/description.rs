use gpui::{
    AnyElement, App, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window, div,
};

use crate::{EtkinTema as _, StilUzantisi as _};

/// açıklama öğe için bir iletişim kutusu başlık.
///
/// Typically kullanılır içinde bir IletisimBasligi bileşen için sağlar ek bağlam.
///
/// # Örnekler
///
/// ```ignore
/// IletisimAciklamasi::new("This action cannot be undone.")
/// ```
#[derive(IntoElement)]
pub struct IletisimAciklamasi {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl IletisimAciklamasi {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: vec![],
        }
    }
}

impl ParentElement for IletisimAciklamasi {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for IletisimAciklamasi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for IletisimAciklamasi {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id("dialog-description")
            .text_sm()
            .text_color(cx.theme().muted_foreground)
            .refine_style(&self.style)
            .children(self.children)
    }
}
