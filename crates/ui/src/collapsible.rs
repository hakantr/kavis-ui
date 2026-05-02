use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, StyleRefinement, Styled, Window,
};

use crate::{StyledExt, v_flex};

enum CollapsibleChild {
    Element(AnyElement),
    Content(AnyElement),
}

impl CollapsibleChild {
    fn is_content(&self) -> bool {
        matches!(self, CollapsibleChild::Content(_))
    }
}

/// Genişleyip daralabilen etkileşimli bir öğe.
#[derive(IntoElement)]
pub struct Daraltilabilir {
    style: StyleRefinement,
    children: Vec<CollapsibleChild>,
    open: bool,
}

impl Daraltilabilir {
    /// Yeni bir `Daraltilabilir` örneği oluşturur.
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            open: false,
            children: vec![],
        }
    }

    /// Daraltılabilir öğenin açık olup olmadığını ayarlar. Varsayılan false değeridir.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Daraltılabilir öğenin içeriğini ayarlar.
    ///
    /// `open` false ise içerik gizlenir.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.children
            .push(CollapsibleChild::Content(content.into_any_element()));
        self
    }
}

impl Styled for Daraltilabilir {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Daraltilabilir {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children
            .extend(elements.into_iter().map(|el| CollapsibleChild::Element(el)));
    }
}

impl RenderOnce for Daraltilabilir {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .refine_style(&self.style)
            .children(self.children.into_iter().filter_map(|child| {
                if child.is_content() && !self.open {
                    None
                } else {
                    match child {
                        CollapsibleChild::Element(el) => Some(el),
                        CollapsibleChild::Content(el) => Some(el),
                    }
                }
            }))
    }
}
