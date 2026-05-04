use std::rc::Rc;

use crate::ham_gpui::{
    App, Axis, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
};

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, EksenUzantisi, StilUzantisi as _, stepper::AdimlayiciOgesi,
};

/// Bir step-ile-step ilerleme için kullanıcılar için navigate üzerinden bir seri steps veya stages.
#[derive(IntoElement)]
pub struct Adimlayici {
    id: ElementId,
    style: StyleRefinement,
    items: Vec<AdimlayiciOgesi>,
    step: usize,
    layout: Axis,
    disabled: bool,
    size: BilesenBoyutu,
    text_center: bool,
    on_click: Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>,
}

impl Adimlayici {
    /// Yeni bir adımlayıcı ile verilen ID oluşturur.
    ///
    /// Varsayılan kullanımda yatay yerleşim ve seçili 0. adım kullanılır.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            items: Vec::new(),
            step: 0,
            layout: Axis::Horizontal,
            disabled: false,
            size: BilesenBoyutu::default(),
            text_center: false,
            on_click: Rc::new(|_, _, _| {}),
        }
    }

    /// Her adımlayıcı öğesindeki metnin ortalanıp ortalanmayacağını ayarlar.
    pub fn text_center(mut self, center: bool) -> Self {
        self.text_center = center;
        self
    }

    /// Adımlayıcı yerleşimini ayarlar. Varsayılan yataydır.
    pub fn layout(mut self, layout: Axis) -> Self {
        self.layout = layout;
        self
    }

    /// Adımlayıcı yerleşimini dikey olarak ayarlar.
    pub fn vertical(mut self) -> Self {
        self.layout = Axis::Vertical;
        self
    }

    /// Adımlayıcının seçili indeksini ayarlar. Varsayılan 0dır.
    pub fn selected_index(mut self, index: usize) -> Self {
        self.step = index;
        self
    }

    /// bir adımlayıcı öğe için adımlayıcı. ekler.
    pub fn item(mut self, item: AdimlayiciOgesi) -> Self {
        self.items.push(item);
        self
    }

    /// Birden çok adımlayıcı öğeler için adımlayıcı ekler.
    pub fn items(mut self, items: impl IntoIterator<Item = AdimlayiciOgesi>) -> Self {
        self.items.extend(items);
        self
    }

    /// Adımlayıcının devre dışı durumunu ayarlar. Varsayılan false değeridir.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Bir adım tıklandığında çağrılacak on_click işleyicisini ekler.
    ///
    /// İlk parametre, tıklanan `step` öğesidir.
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn(&usize, &mut Window, &mut App) + 'static,
    {
        self.on_click = Rc::new(f);
        self
    }
}

impl Boyutlandirilabilir for Adimlayici {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for Adimlayici {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Adimlayici {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let total_items = self.items.len();
        div()
            .id(self.id)
            .w_full()
            .when(self.layout.yatay_mi(), |this| this.h_flex())
            .when(self.layout.dikey_mi(), |this| this.v_flex())
            .refine_style(&self.style)
            .children(self.items.into_iter().enumerate().map(|(step, item)| {
                let is_last = step + 1 == total_items;
                item.step(step)
                    .with_size(self.size)
                    .checked_step(self.step)
                    .layout(self.layout)
                    .text_center(self.text_center)
                    .when(self.disabled, |this| this.disabled(true))
                    .is_last(is_last)
                    .on_click({
                        let on_click = self.on_click.clone();
                        move |_, window, cx| {
                            on_click(&step, window, cx);
                        }
                    })
            }))
    }
}
