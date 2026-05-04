use crate::ham_gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::{StilUzantisi as _, v_flex};

/// başlık bölüm bir iletişim kutusu, typically içerir IletisimBaslikMetni ve IletisimAciklamasi.
///
/// # Örnekler
///
/// ```ignore
/// IletisimBasligi::new()
///     .child(IletisimBaslikMetni::new().child("Delete Account"))
///     .child(IletisimAciklamasi::new().child("This action cannot be undone."))
/// ```
#[derive(IntoElement)]
pub struct IletisimBasligi {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl IletisimBasligi {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for IletisimBasligi {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for IletisimBasligi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for IletisimBasligi {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .gap_2()
            .refine_style(&self.style)
            .children(self.children)
    }
}
