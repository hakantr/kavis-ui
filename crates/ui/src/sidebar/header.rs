use crate::ham_gpui::{
    AnyElement, Div, InteractiveElement, IntoElement, ParentElement, RenderOnce, StyleRefinement,
    Styled, div, prelude::FluentBuilder as _,
};

use crate::{
    Daraltilabilir, EtkinTema as _, Secilebilir, StilUzantisi, menu::AcilirMenuTetikleyici,
};

/// başlık için [`super::YanCubuk`]
#[derive(IntoElement)]
pub struct YanCubukBasligi {
    base: Div,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    selected: bool,
    collapsed: bool,
}

impl YanCubukBasligi {
    /// Yeni bir [`YanCubukBasligi`] oluşturur.
    pub fn new() -> Self {
        Self {
            base: div(),
            style: StyleRefinement::default(),
            children: Vec::new(),
            selected: false,
            collapsed: false,
        }
    }
}

impl Default for YanCubukBasligi {
    fn default() -> Self {
        Self::new()
    }
}

impl Secilebilir for YanCubukBasligi {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl Daraltilabilir for YanCubukBasligi {
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl ParentElement for YanCubukBasligi {
    fn extend(&mut self, elements: impl IntoIterator<Item = crate::ham_gpui::AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for YanCubukBasligi {
    fn style(&mut self) -> &mut crate::ham_gpui::StyleRefinement {
        &mut self.style
    }
}

impl InteractiveElement for YanCubukBasligi {
    fn interactivity(&mut self) -> &mut crate::ham_gpui::Interactivity {
        self.base.interactivity()
    }
}

impl AcilirMenuTetikleyici for YanCubukBasligi {}

impl RenderOnce for YanCubukBasligi {
    fn render(self, _: &mut crate::ham_gpui::Window, cx: &mut crate::ham_gpui::App) -> impl crate::ham_gpui::IntoElement {
        self.base
            .id("sidebar-header")
            .h_flex()
            .gap_2()
            .p_2()
            .w_full()
            .justify_between()
            .rounded(cx.theme().radius)
            .refine_style(&self.style)
            .hover(|this| {
                this.bg(cx.theme().sidebar_accent)
                    .text_color(cx.theme().sidebar_accent_foreground)
            })
            .when(self.selected, |this| {
                this.bg(cx.theme().sidebar_accent)
                    .text_color(cx.theme().sidebar_accent_foreground)
            })
            .children(self.children)
    }
}
