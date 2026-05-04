use std::{rc::Rc, time::Duration};

use crate::ham_gpui::{
    App, ClipboardItem, ElementId, IntoElement, RenderOnce, SharedString, Window,
    prelude::FluentBuilder,
};

use crate::{
    Boyutlandirilabilir as _, SimgeAdi,
    button::{Dugme, DugmeVaryantlari as _},
};

/// Panoya kopyalama işlevi sağlayan bir öğe.
#[derive(IntoElement)]
pub struct Pano {
    id: ElementId,
    value: SharedString,
    value_fn: Option<Rc<dyn Fn(&mut Window, &mut App) -> SharedString>>,
    on_copied: Option<Rc<dyn Fn(SharedString, &mut Window, &mut App)>>,
    tooltip_text: Option<SharedString>,
}

impl Pano {
    /// Verilen kimlikle yeni bir Pano öğesi oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: SharedString::default(),
            value_fn: None,
            on_copied: None,
            tooltip_text: None,
        }
    }

    /// Pano düğmesi için araç ipucu metnini ayarlar.
    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip_text = Some(tooltip.into());
        self
    }

    /// Panoya kopyalanacak değeri ayarlar. Varsayılan boş metindir.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    /// Pano değerini verilen fonksiyonun sonucuna ayarlar. Varsayılan None değeridir.
    ///
    /// Bu kullanıldığında kopyalama değeri fonksiyonun sonucunu kullanır.
    pub fn value_fn(
        mut self,
        value: impl Fn(&mut Window, &mut App) -> SharedString + 'static,
    ) -> Self {
        self.value_fn = Some(Rc::new(value));
        self
    }

    /// İçerik panoya kopyalandığında çağrılacak geri çağrıyı ayarlar.
    pub fn on_copied<F>(mut self, handler: F) -> Self
    where
        F: Fn(SharedString, &mut Window, &mut App) + 'static,
    {
        self.on_copied = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Pano {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| ClipboardState::default());

        let value = self.value.clone();
        let clipboard_id = self.id.clone();
        let copied = state.read(cx).copied;
        let value_fn = self.value_fn.clone();

        Dugme::new(clipboard_id)
            .icon(if copied {
                SimgeAdi::Check
            } else {
                SimgeAdi::Copy
            })
            .ghost()
            .xsmall()
            .when_some(self.tooltip_text, |this, text| this.tooltip(text))
            .when(!copied, |this| {
                this.on_click({
                    let state = state.clone();
                    let on_copied = self.on_copied.clone();
                    move |_, window, cx| {
                        cx.stop_propagation();
                        let value = value_fn
                            .as_ref()
                            .map(|f| f(window, cx))
                            .unwrap_or_else(|| value.clone());
                        cx.write_to_clipboard(ClipboardItem::new_string(value.to_string()));
                        state.update(cx, |state, cx| {
                            state.copied = true;
                            cx.notify();
                        });

                        let state = state.clone();
                        cx.spawn(async move |cx| {
                            cx.background_executor().timer(Duration::from_secs(2)).await;
                            _ = state.update(cx, |state, cx| {
                                state.copied = false;
                                cx.notify();
                            });
                        })
                        .detach();

                        if let Some(on_copied) = &on_copied {
                            on_copied(value.clone(), window, cx);
                        }
                    }
                })
            })
    }
}

#[doc(hidden)]
#[derive(Default)]
struct ClipboardState {
    copied: bool,
}
