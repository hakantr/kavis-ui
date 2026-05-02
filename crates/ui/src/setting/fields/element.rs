use gpui::{AnyElement, App, IntoElement, StyleRefinement, Window};
use std::rc::Rc;

use crate::setting::{HerhangiBirAyarAlani, RenderOptions, fields::AyarAlaniCizimi};

/// Özel ayar alanı öğelerini çizmek için trait.
///
/// For [`crate::ayar::AyarAlani::öğe`] yöntem.
pub trait AyarAlaniOgesi {
    type Element: IntoElement + 'static;

    fn render_field(
        &self,
        options: &RenderOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::Element;
}

impl<F, E> AyarAlaniOgesi for F
where
    E: IntoElement + 'static,
    F: Fn(&RenderOptions, &mut Window, &mut App) -> E,
{
    type Element = E;

    fn render_field(
        &self,
        options: &RenderOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::Element {
        (self)(options, window, cx)
    }
}

pub(crate) struct HerhangiBirAyarAlaniOgesi<T>(pub(crate) T);
impl<T> AyarAlaniOgesi for HerhangiBirAyarAlaniOgesi<T>
where
    T: AyarAlaniOgesi,
{
    type Element = AnyElement;

    fn render_field(
        &self,
        options: &RenderOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::Element {
        self.0.render_field(options, window, cx).into_any_element()
    }
}
impl AyarAlaniOgesi for Rc<dyn AyarAlaniOgesi<Element = AnyElement>> {
    type Element = AnyElement;

    fn render_field(
        &self,
        options: &RenderOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::Element {
        self.as_ref().render_field(options, window, cx)
    }
}

pub(crate) struct ElementField {
    element_render: Rc<dyn AyarAlaniOgesi<Element = AnyElement>>,
}

impl ElementField {
    pub(crate) fn new<E>(element_render: E) -> Self
    where
        E: AyarAlaniOgesi<Element = AnyElement> + 'static,
    {
        Self {
            element_render: Rc::new(element_render),
        }
    }
}

impl AyarAlaniCizimi for ElementField {
    fn render(
        &self,
        _: Rc<dyn HerhangiBirAyarAlani>,
        options: &RenderOptions,
        _style: &StyleRefinement,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        (self.element_render).render_field(options, window, cx)
    }
}
