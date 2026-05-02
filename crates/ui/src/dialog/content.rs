use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::{EtkinTema as _, StyledExt as _, v_flex};

/// içerik kapsayıcı için bir iletişim kutusu.
#[derive(IntoElement)]
pub struct IletisimIcerigi {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl IletisimIcerigi {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for IletisimIcerigi {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for IletisimIcerigi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for IletisimIcerigi {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .flex_1()
            .rounded(cx.theme().radius_lg)
            .refine_style(&self.style)
            .children(self.children)
    }
}
