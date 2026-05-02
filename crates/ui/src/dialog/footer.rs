use gpui::{
    AnyElement, App, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, div, relative,
};

use crate::{
    EtkinTema as _, StyledExt as _,
    dialog::{CancelDialog, ConfirmDialog},
    h_flex,
};

/// Footer bölüm bir iletişim kutusu, typically içerir eylem düğmeler.
///
/// # Örnekler
///
/// ```ignore
/// IletisimAltligi::new()
///     .child(IletisimKapat::new().child(Dugme::new("cancel").label("Cancel")))
///     .child(Dugme::new("confirm").label("Confirm"))
/// ```
#[derive(IntoElement)]
pub struct IletisimAltligi {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl IletisimAltligi {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for IletisimAltligi {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for IletisimAltligi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for IletisimAltligi {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .gap_2()
            .justify_end()
            .line_height(relative(1.))
            .rounded_b(cx.theme().radius_lg)
            .refine_style(&self.style)
            .children(self.children)
    }
}

pub trait IletisimAltDugmesi {
    fn is_cancel(&self) -> bool {
        false
    }

    fn is_action(&self) -> bool {
        false
    }
}

#[derive(IntoElement)]
pub struct IletisimKapat {
    children: Vec<AnyElement>,
}

impl IletisimKapat {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for IletisimKapat {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for IletisimKapat {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .size_full()
            .id("dialog-close")
            .on_click(move |_, window, cx| window.dispatch_action(Box::new(CancelDialog), cx))
            .children(self.children)
    }
}

#[derive(IntoElement)]
pub struct IletisimEylemi {
    children: Vec<AnyElement>,
}

impl IletisimEylemi {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl ParentElement for IletisimEylemi {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for IletisimEylemi {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .size_full()
            .id("dialog-action")
            .on_click(move |_, window, cx| window.dispatch_action(Box::new(ConfirmDialog), cx))
            .children(self.children)
    }
}
