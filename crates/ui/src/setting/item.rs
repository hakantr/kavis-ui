use gpui::{
    AnyElement, App, Axis, Div, InteractiveElement as _, IntoElement, ParentElement, SharedString,
    Stateful, Styled, Window, div, prelude::FluentBuilder as _,
};
use std::{any::TypeId, ops::Deref, rc::Rc};

use crate::{
    AxisExt, EtkinTema as _, StilUzantisi as _,
    label::Etiket,
    setting::{
        ElementField, HerhangiBirAyarAlani, RenderOptions,
        fields::{AyarAlaniCizimi, BoolField, DropdownField, NumberField, StringField},
    },
    text::Text,
    v_flex,
};

/// ayar öğe.
#[derive(Clone)]
pub enum AyarOgesi {
    /// Bir normal ayar öğe ile bir başlık, açıklama, ve alan.
    Item {
        title: SharedString,
        description: Option<Text>,
        layout: Axis,
        field: Rc<dyn HerhangiBirAyarAlani>,
    },
    /// Bir tam özel öğe çizmek için.
    Element {
        render: Rc<dyn Fn(&RenderOptions, &mut Window, &mut App) -> AnyElement + 'static>,
    },
}

impl AyarOgesi {
    /// Yeni bir ayar öğe oluşturur.
    pub fn new<F>(title: impl Into<SharedString>, field: F) -> Self
    where
        F: HerhangiBirAyarAlani + 'static,
    {
        AyarOgesi::Item {
            title: title.into(),
            description: None,
            layout: Axis::Horizontal,
            field: Rc::new(field),
        }
    }

    /// Yeni özel öğe ayar öğe ile bir çizer kapanış oluşturur.
    pub fn render<R, E>(render: R) -> Self
    where
        E: IntoElement,
        R: Fn(&RenderOptions, &mut Window, &mut App) -> E + 'static,
    {
        AyarOgesi::Element {
            render: Rc::new(move |options, window, cx| {
                render(options, window, cx).into_any_element()
            }),
        }
    }

    /// açıklama ayar öğe ayarlar.
    ///
    /// Yalnızca applies için [`AyarOgesi::öğe`].
    pub fn description(mut self, description: impl Into<Text>) -> Self {
        match &mut self {
            AyarOgesi::Item { description: d, .. } => {
                *d = Some(description.into());
            }
            AyarOgesi::Element { .. } => {}
        }
        self
    }

    /// yerleşim ayar öğe ayarlar.
    ///
    /// Yalnızca applies için [`AyarOgesi::öğe`].
    pub fn layout(mut self, layout: Axis) -> Self {
        match &mut self {
            AyarOgesi::Item { layout: l, .. } => {
                *l = layout;
            }
            AyarOgesi::Element { .. } => {}
        }
        self
    }

    pub(crate) fn is_match(&self, query: &str, cx: &App) -> bool {
        match self {
            AyarOgesi::Item {
                title, description, ..
            } => {
                title.to_lowercase().contains(&query.to_lowercase())
                    || description.as_ref().map_or(false, |d| {
                        d.get_text(cx)
                            .to_lowercase()
                            .contains(&query.to_lowercase())
                    })
            }
            // We need to show all custom elements when not searching.
            AyarOgesi::Element { .. } => query.is_empty(),
        }
    }

    pub(crate) fn is_resettable(&self, cx: &App) -> bool {
        match self {
            AyarOgesi::Item { field, .. } => field.is_resettable(cx),
            AyarOgesi::Element { .. } => false,
        }
    }

    pub(crate) fn reset(&self, window: &mut Window, cx: &mut App) {
        match self {
            AyarOgesi::Item { field, .. } => field.reset(window, cx),
            AyarOgesi::Element { .. } => {}
        }
    }

    fn render_field(
        field: Rc<dyn HerhangiBirAyarAlani>,
        options: RenderOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let field_type = field.field_type();
        let style = field.style().clone();
        let type_id = field.deref().type_id();
        let renderer: Box<dyn AyarAlaniCizimi> = match type_id {
            t if t == std::any::TypeId::of::<bool>() => {
                Box::new(BoolField::new(field_type.is_switch()))
            }
            t if t == TypeId::of::<f64>() && field_type.is_number_input() => {
                Box::new(NumberField::new(field_type.number_input_options()))
            }
            t if t == TypeId::of::<SharedString>() && field_type.is_input() => {
                Box::new(StringField::<SharedString>::new())
            }
            t if t == TypeId::of::<String>() && field_type.is_input() => {
                Box::new(StringField::<String>::new())
            }
            t if t == TypeId::of::<SharedString>() && field_type.is_dropdown() => {
                Box::new(DropdownField::<SharedString>::new(
                    field_type.dropdown_options(),
                    field_type.dropdown_scrollable(),
                ))
            }
            t if t == TypeId::of::<String>() && field_type.is_dropdown() => {
                Box::new(DropdownField::<String>::new(
                    field_type.dropdown_options(),
                    field_type.dropdown_scrollable(),
                ))
            }
            _ if field_type.is_element() => Box::new(ElementField::new(field_type.element())),
            _ => unimplemented!("Unsupported setting type: {}", field.deref().type_name()),
        };

        renderer.render(field, &options, &style, window, cx)
    }

    pub(super) fn render_item(
        self,
        options: &RenderOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> Stateful<Div> {
        div()
            .id(SharedString::from(format!("item-{}", options.item_ix)))
            .w_full()
            .child(match self {
                AyarOgesi::Item {
                    title,
                    description,
                    layout,
                    field,
                } => div()
                    .w_full()
                    .overflow_hidden()
                    .map(|this| {
                        if layout.is_horizontal() {
                            this.h_flex().justify_between().items_start()
                        } else {
                            this.v_flex()
                        }
                    })
                    .gap_3()
                    .child(
                        v_flex()
                            .map(|this| {
                                if layout.is_horizontal() {
                                    this.flex_1().max_w_3_5()
                                } else {
                                    this.w_full()
                                }
                            })
                            .gap_1()
                            .child(Etiket::new(title.clone()).text_sm())
                            .when_some(description.clone(), |this, description| {
                                this.child(
                                    div()
                                        .size_full()
                                        .text_sm()
                                        .text_color(cx.theme().muted_foreground)
                                        .child(description),
                                )
                            }),
                    )
                    .child(div().id("field").child(Self::render_field(
                        field,
                        RenderOptions { layout, ..*options },
                        window,
                        cx,
                    )))
                    .into_any_element(),
                AyarOgesi::Element { render } => (render)(&options, window, cx).into_any_element(),
            })
    }
}
