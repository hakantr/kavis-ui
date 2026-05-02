use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _,
};

use crate::{EtkinTema, Simge, SimgeAdi, StyledExt, h_flex};

/// Breadcrumb gezinme öğesi.
#[derive(IntoElement)]
pub struct GezintiYolu {
    style: StyleRefinement,
    items: Vec<GezintiYoluOgesi>,
}

/// öğe için [`GezintiYolu`].
#[derive(IntoElement)]
pub struct GezintiYoluOgesi {
    id: ElementId,
    style: StyleRefinement,
    label: SharedString,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    disabled: bool,
    is_last: bool,
}

impl GezintiYoluOgesi {
    /// Yeni bir GezintiYoluOgesi ile verilen id ve etiket oluşturur.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            id: ElementId::Integer(0),
            style: StyleRefinement::default(),
            label: label.into(),
            on_click: None,
            disabled: false,
            is_last: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// For dahili kullanım yalnızca.
    fn is_last(mut self, is_last: bool) -> Self {
        self.is_last = is_last;
        self
    }
}

impl Styled for GezintiYoluOgesi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl From<&'static str> for GezintiYoluOgesi {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl From<String> for GezintiYoluOgesi {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<SharedString> for GezintiYoluOgesi {
    fn from(value: SharedString) -> Self {
        Self::new(value)
    }
}

impl RenderOnce for GezintiYoluOgesi {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .child(self.label)
            .text_color(cx.theme().muted_foreground)
            .when(self.is_last, |this| this.text_color(cx.theme().foreground))
            .when(self.disabled, |this| {
                this.text_color(cx.theme().muted_foreground)
            })
            .refine_style(&self.style)
            .when(!self.disabled, |this| {
                this.when_some(self.on_click, |this, on_click| {
                    this.cursor_pointer().on_click(move |event, window, cx| {
                        on_click(event, window, cx);
                    })
                })
            })
    }
}

impl GezintiYolu {
    /// Yeni bir breadcrumb oluşturur.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    /// bir [`GezintiYoluOgesi`] için breadcrumb. ekler.
    pub fn child(mut self, item: impl Into<GezintiYoluOgesi>) -> Self {
        self.items.push(item.into());
        self
    }

    /// Birden çok [`GezintiYoluOgesi`] öğeler için breadcrumb ekler.
    pub fn children(
        mut self,
        items: impl IntoIterator<Item = impl Into<GezintiYoluOgesi>>,
    ) -> Self {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }
}

#[derive(IntoElement)]
struct BreadcrumbSeparator;
impl RenderOnce for BreadcrumbSeparator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        Simge::new(SimgeAdi::ChevronRight)
            .text_color(cx.theme().muted_foreground)
            .size_3p5()
            .into_any_element()
    }
}

impl Styled for GezintiYolu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for GezintiYolu {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let items_count = self.items.len();

        let mut children = vec![];
        for (ix, item) in self.items.into_iter().enumerate() {
            let is_last = ix == items_count - 1;

            let item = item.id(ix);
            children.push(item.is_last(is_last).into_any_element());
            if !is_last {
                children.push(BreadcrumbSeparator.into_any_element());
            }
        }

        h_flex()
            .gap_1p5()
            .text_sm()
            .text_color(cx.theme().muted_foreground)
            .refine_style(&self.style)
            .children(children)
    }
}
