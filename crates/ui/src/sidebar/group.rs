use crate::{Daraltilabilir, EtkinTema, h_flex, sidebar::YanCubukOgesi, v_flex};
use crate::ham_gpui::{
    App, ElementId, IntoElement, ParentElement, SharedString, Styled as _, Window, div,
    prelude::FluentBuilder as _,
};

/// Bir grup öğeler içinde [`super::YanCubuk`].
#[derive(Clone)]
pub struct YanCubukGrubu<E: YanCubukOgesi + 'static> {
    label: SharedString,
    collapsed: bool,
    children: Vec<E>,
}

impl<E: YanCubukOgesi> YanCubukGrubu<E> {
    /// Yeni bir [`YanCubukGrubu`] ile verilen etiket oluşturur.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            collapsed: false,
            children: Vec::new(),
        }
    }

    /// Bir alt için sidebar grup, alt olmalı implement [`YanCubukOgesi`] ekler.
    pub fn child(mut self, child: E) -> Self {
        self.children.push(child);
        self
    }

    /// Birden çok alt öğeler için sidebar grup ekler.
    ///
    /// Bakınız ayrıca [`YanCubukGrubu::alt`].
    pub fn children(mut self, children: impl IntoIterator<Item = E>) -> Self {
        self.children.extend(children);
        self
    }
}

impl<E: YanCubukOgesi> Daraltilabilir for YanCubukGrubu<E> {
    fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }
}

impl<E: YanCubukOgesi> YanCubukOgesi for YanCubukGrubu<E> {
    fn render(
        self,
        id: impl Into<ElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let id = id.into();

        v_flex()
            .relative()
            .when(!self.collapsed, |this| {
                this.child(
                    h_flex()
                        .flex_shrink_0()
                        .px_2()
                        .rounded(cx.theme().radius)
                        .text_xs()
                        .text_color(cx.theme().sidebar_foreground.opacity(0.7))
                        .h_8()
                        .child(self.label),
                )
            })
            .child(
                div()
                    .gap_2()
                    .flex_col()
                    .children(self.children.into_iter().enumerate().map(|(ix, child)| {
                        child
                            .collapsed(self.collapsed)
                            .render(format!("{}-{}", id, ix), window, cx)
                            .into_any_element()
                    })),
            )
    }
}
