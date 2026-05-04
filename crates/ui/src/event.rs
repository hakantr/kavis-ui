use gpui::{App, ClickEvent, InteractiveElement, Stateful, Window};

pub trait EtkilesimliOgeUzantisi: InteractiveElement {
    /// dinleyici için bir double tıklama olay ayarlar.
    fn cift_tiklamada(
        mut self,
        listener: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self
    where
        Self: Sized,
    {
        self.interactivity().on_click(move |event, window, cx| {
            if event.click_count() == 2 {
                listener(event, window, cx);
            }
        });
        self
    }
}

impl<E: InteractiveElement> EtkilesimliOgeUzantisi for Stateful<E> {}
