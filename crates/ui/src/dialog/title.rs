use gpui::{
    AnyElement, App, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window, div, relative,
};

use crate::StilUzantisi as _;

/// başlık öğe için bir iletişim kutusu başlık.
#[derive(IntoElement)]
pub struct IletisimBaslikMetni {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl IletisimBaslikMetni {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: vec![],
        }
    }
}

impl ParentElement for IletisimBaslikMetni {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for IletisimBaslikMetni {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for IletisimBaslikMetni {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id("dialog-title")
            .text_base()
            .font_semibold()
            .line_height(relative(1.))
            .refine_style(&self.style)
            .children(self.children)
    }
}
