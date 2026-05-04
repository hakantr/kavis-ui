use gpui::{
    Div, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled,
    prelude::FluentBuilder as _,
};

use crate::{Daraltilabilir, EtkinTema as _, Secilebilir, h_flex, menu::AcilirMenuTetikleyici};

/// Footer için [`super::YanCubuk`].
#[derive(IntoElement)]
pub struct YanCubukAltligi {
    base: Div,
    selected: bool,
    collapsed: bool,
}

impl YanCubukAltligi {
    /// Yeni bir [`YanCubukAltligi`] oluşturur.
    pub fn new() -> Self {
        Self {
            base: h_flex().gap_2().w_full(),
            selected: false,
            collapsed: false,
        }
    }
}

impl Secilebilir for YanCubukAltligi {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl Daraltilabilir for YanCubukAltligi {
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl ParentElement for YanCubukAltligi {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for YanCubukAltligi {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for YanCubukAltligi {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl AcilirMenuTetikleyici for YanCubukAltligi {}

impl RenderOnce for YanCubukAltligi {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        h_flex()
            .id("sidebar-footer")
            .gap_2()
            .p_2()
            .w_full()
            .justify_between()
            .rounded(cx.theme().radius)
            .hover(|this| {
                this.bg(cx.theme().sidebar_accent)
                    .text_color(cx.theme().sidebar_accent_foreground)
            })
            .when(self.selected, |this| {
                this.bg(cx.theme().sidebar_accent)
                    .text_color(cx.theme().sidebar_accent_foreground)
            })
            .child(self.base)
    }
}
