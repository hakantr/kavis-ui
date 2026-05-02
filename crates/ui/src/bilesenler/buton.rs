use std::rc::Rc;

use crate::{
    App, ClickEvent, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    StatefulInteractiveElement as _, Styled, Window, div, px, rgb,
};

type TiklamaAksiyonu = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Kucuk uygulama ekranlari icin basit Turkce buton bileseni.
///
/// Daha kapsamli durum, boyut ve varyant ihtiyaclarinda [`crate::button::Dugme`]
/// kullanilabilir.
pub struct KavisButon {
    etiket: String,
    tiklama_aksiyonu: Option<TiklamaAksiyonu>,
}

impl KavisButon {
    pub fn yeni(etiket: impl Into<String>) -> Self {
        Self {
            etiket: etiket.into(),
            tiklama_aksiyonu: None,
        }
    }

    pub fn tiklandiginda(
        mut self,
        aksiyon: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.tiklama_aksiyonu = Some(Rc::new(aksiyon));
        self
    }
}

impl RenderOnce for KavisButon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut kutu = div()
            .id(self.etiket.clone())
            .px_3()
            .py_2()
            .rounded(px(6.))
            .bg(rgb(0x3B82F6))
            .text_color(rgb(0xFFFFFF))
            .cursor_pointer()
            .hover(|stil| stil.bg(rgb(0x2563EB)))
            .child(self.etiket);

        if let Some(aksiyon) = self.tiklama_aksiyonu {
            kutu = kutu.on_click(move |olay, window, cx| {
                aksiyon(olay, window, cx);
            });
        }

        kutu
    }
}
