use std::rc::Rc;

use crate::ham_gpui::{
    AnyElement, App, IntoElement, ParentElement as _, StyleRefinement, Window, div,
};
use crate::{
    Boyutlandirilabilir, StilUzantisi,
    checkbox::OnayKutusu,
    setting::{
        HerhangiBirAyarAlani, RenderOptions,
        fields::{AyarAlaniCizimi, get_value, set_value},
    },
    switch::Anahtar,
};

pub(crate) struct BoolField {
    use_switch: bool,
}

impl BoolField {
    pub(crate) fn new(use_switch: bool) -> Self {
        Self { use_switch }
    }
}

impl AyarAlaniCizimi for BoolField {
    fn render(
        &self,
        field: Rc<dyn HerhangiBirAyarAlani>,
        options: &RenderOptions,
        style: &StyleRefinement,
        _: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let checked = get_value::<bool>(&field, cx);
        let set_value = set_value::<bool>(&field, cx);

        div()
            .refine_style(style)
            .child(if self.use_switch {
                Anahtar::new("check")
                    .checked(checked)
                    .with_size(options.size)
                    .on_click(move |checked: &bool, _, cx: &mut App| {
                        set_value(*checked, cx);
                    })
                    .into_any_element()
            } else {
                OnayKutusu::new("check")
                    .checked(checked)
                    .with_size(options.size)
                    .on_click(move |checked: &bool, _, cx: &mut App| {
                        set_value(*checked, cx);
                    })
                    .into_any_element()
            })
            .into_any_element()
    }
}
