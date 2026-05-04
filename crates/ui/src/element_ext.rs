use gpui::{
    AnyElement, App, Bounds, IntoElement, ParentElement, Pixels, Styled as _, Window, canvas,
};

use crate::{BilesenBoyutu, Boyutlandirilabilir};

#[derive(Default)]
struct ChildElementOptions {
    ix: usize,
    size: BilesenBoyutu,
}

#[allow(patterns_in_fns_without_body)]
pub trait ChildElement: Boyutlandirilabilir + IntoElement {
    fn with_ix(mut self, ix: usize) -> Self;
}

/// Çizilmeden önce [`AnyChildElementOptions`] kabul edebilen tipi silinmiş öğe.
pub struct AnyChildElement(Box<dyn FnOnce(ChildElementOptions) -> AnyElement>);

impl AnyChildElement {
    pub fn new(element: impl ChildElement + 'static) -> Self {
        Self(Box::new(|options| {
            element
                .with_ix(options.ix)
                .with_size(options.size)
                .into_any_element()
        }))
    }

    pub fn into_any(self, ix: usize, size: BilesenBoyutu) -> AnyElement {
        (self.0)(ChildElementOptions { ix, size })
    }
}

/// Bir özellik için extend [`gpui::öğe`] ile ek işlev.
pub trait ElementExt: ParentElement + Sized {
    /// Bir prepaint geri çağrı için öğe ekler.
    ///
    /// Öğe çizildikten sonra sınırlarını almak için yardımcı yöntem.
    ///
    /// İlk argüman öğenin piksel cinsinden sınırlarıdır.
    ///
    /// Bakınız ayrıca [`gpui::canvas`].
    fn on_prepaint<F>(self, f: F) -> Self
    where
        F: FnOnce(Bounds<Pixels>, &mut Window, &mut App) + 'static,
    {
        self.child(
            canvas(
                move |bounds, window, cx| f(bounds, window, cx),
                |_, _, _, _| {},
            )
            .absolute()
            .size_full(),
        )
    }
}

impl<T: ParentElement> ElementExt for T {}
