use std::{panic::Location, rc::Rc};

use crate::{StilUzantisi, scroll::KaydirmaCubuguTutamaci};

use super::{KaydirmaCubugu, KaydirmaCubuguEkseni};
use gpui::{
    App, Div, Element, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    ScrollHandle, Stateful, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder,
};

/// Bir özellik için öğeler olan olabilir made scrollable ile scrollbars.
pub trait KaydirilabilirOge: InteractiveElement + Styled + ParentElement + Element {
    /// bir kaydırma çubuğu için öğe. ekler.
    #[track_caller]
    fn scrollbar<H: KaydirmaCubuguTutamaci + Clone>(
        self,
        scroll_handle: &H,
        axis: impl Into<KaydirmaCubuguEkseni>,
    ) -> Self {
        self.child(ScrollbarLayer {
            id: "scrollbar_layer".into(),
            axis: axis.into(),
            scroll_handle: Rc::new(scroll_handle.clone()),
        })
    }

    /// bir dikey kaydırma çubuğu için öğe. ekler.
    #[track_caller]
    fn vertical_scrollbar<H: KaydirmaCubuguTutamaci + Clone>(self, scroll_handle: &H) -> Self {
        self.scrollbar(scroll_handle, KaydirmaCubuguEkseni::Vertical)
    }
    /// bir yatay kaydırma çubuğu için öğe. ekler.
    #[track_caller]
    fn horizontal_scrollbar<H: KaydirmaCubuguTutamaci + Clone>(self, scroll_handle: &H) -> Self {
        self.scrollbar(scroll_handle, KaydirmaCubuguEkseni::Horizontal)
    }

    /// Almost equivalent için [`StatefulInteractiveElement::overflow_scroll`], ama adds scrollbars.
    #[track_caller]
    fn overflow_scrollbar(self) -> Kaydirilabilir<Self> {
        Kaydirilabilir::new(self, KaydirmaCubuguEkseni::Both)
    }

    /// [`StatefulInteractiveElement::overflow_x_scroll`] ile neredeyse aynıdır, ancak yatay kaydırma çubuğu ekler.
    #[track_caller]
    fn overflow_x_scrollbar(self) -> Kaydirilabilir<Self> {
        Kaydirilabilir::new(self, KaydirmaCubuguEkseni::Horizontal)
    }

    /// [`StatefulInteractiveElement::overflow_y_scroll`] ile neredeyse aynıdır, ancak dikey kaydırma çubuğu ekler.
    #[track_caller]
    fn overflow_y_scrollbar(self) -> Kaydirilabilir<Self> {
        Kaydirilabilir::new(self, KaydirmaCubuguEkseni::Vertical)
    }
}

/// Etkileşimli öğeye kaydırma çubukları ekleyen kaydırılabilir öğe sarmalayıcısı.
#[derive(IntoElement)]
pub struct Kaydirilabilir<E: InteractiveElement + Styled + ParentElement + Element> {
    id: ElementId,
    element: E,
    axis: KaydirmaCubuguEkseni,
}

impl<E> Kaydirilabilir<E>
where
    E: InteractiveElement + Styled + ParentElement + Element,
{
    #[track_caller]
    fn new(element: E, axis: impl Into<KaydirmaCubuguEkseni>) -> Self {
        let caller = Location::caller();
        Self {
            id: ElementId::CodeLocation(*caller),
            element,
            axis: axis.into(),
        }
    }
}

impl<E> Styled for Kaydirilabilir<E>
where
    E: InteractiveElement + Styled + ParentElement + Element,
{
    fn style(&mut self) -> &mut StyleRefinement {
        self.element.style()
    }
}

impl<E> ParentElement for Kaydirilabilir<E>
where
    E: InteractiveElement + Styled + ParentElement + Element,
{
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.element.extend(elements)
    }
}

impl InteractiveElement for Kaydirilabilir<Div> {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.element.interactivity()
    }
}

impl InteractiveElement for Kaydirilabilir<Stateful<Div>> {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.element.interactivity()
    }
}

impl<E> RenderOnce for Kaydirilabilir<E>
where
    E: InteractiveElement + Styled + ParentElement + Element + 'static,
{
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let scroll_handle = window
            .use_keyed_state(self.id.clone(), cx, |_, _| ScrollHandle::default())
            .read(cx)
            .clone();

        // Inherit the size from the element style.
        let style = StyleRefinement {
            size: self.element.style().size.clone(),
            ..Default::default()
        };

        div()
            .id(self.id)
            .size_full()
            .refine_style(&style)
            .relative()
            .child(
                div()
                    .id("scroll-area")
                    .flex()
                    .size_full()
                    .track_scroll(&scroll_handle)
                    .map(|this| match self.axis {
                        KaydirmaCubuguEkseni::Vertical => this.flex_col().overflow_y_scroll(),
                        KaydirmaCubuguEkseni::Horizontal => this.flex_row().overflow_x_scroll(),
                        KaydirmaCubuguEkseni::Both => this.overflow_scroll(),
                    })
                    .child(
                        self.element
                            // Refine element size to `flex_1`.
                            .size_auto()
                            .flex_1(),
                    ),
            )
            .child(render_scrollbar(
                "scrollbar",
                &scroll_handle,
                self.axis,
                window,
                cx,
            ))
    }
}

impl KaydirilabilirOge for Div {}
impl<E> KaydirilabilirOge for Stateful<E>
where
    E: ParentElement + Styled + Element,
    Self: InteractiveElement,
{
}

#[derive(IntoElement)]
struct ScrollbarLayer<H: KaydirmaCubuguTutamaci + Clone> {
    id: ElementId,
    axis: KaydirmaCubuguEkseni,
    scroll_handle: Rc<H>,
}

impl<H> RenderOnce for ScrollbarLayer<H>
where
    H: KaydirmaCubuguTutamaci + Clone + 'static,
{
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        render_scrollbar(self.id, self.scroll_handle.as_ref(), self.axis, window, cx)
    }
}

#[inline]
#[track_caller]
fn render_scrollbar<H: KaydirmaCubuguTutamaci + Clone>(
    id: impl Into<ElementId>,
    scroll_handle: &H,
    axis: KaydirmaCubuguEkseni,
    window: &mut Window,
    cx: &mut App,
) -> Div {
    // Do not render scrollbar when inspector is picking elements,
    // to allow us to pick the background elements.
    let is_inspector_picking = window.is_inspector_picking(cx);
    if is_inspector_picking {
        return div();
    }

    div()
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .bottom_0()
        .child(KaydirmaCubugu::new(scroll_handle).id(id).axis(axis))
}
