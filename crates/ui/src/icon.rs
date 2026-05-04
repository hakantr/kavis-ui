use crate::{BilesenBoyutu, Boyutlandirilabilir, EtkinTema};
use gpui::{
    AnyElement, App, AppContext, Context, Entity, Hsla, IntoElement, Radians, Render, RenderOnce,
    SharedString, StyleRefinement, Styled, Svg, Transformation, Window,
    prelude::FluentBuilder as _, svg,
};
use kavis_ui_macros::simge_adli;

/// Bu özelliği uygulayan tipler otomatik olarak [`Simge`] tipine dönüştürülebilir.
///
/// [`SimgeAdi`] için diğer kullanıcı arayüzü bileşenlerinde doğrudan kullanılabilecek özel bir sürüm tanımlamanıza izin verir.
/// Bu sürüm diğer kullanıcı arayüzü bileşenlerinin beklediği yerde kullanılabilir.
pub trait AdliSimge {
    /// Gömülü simge yolunu döndürür.
    fn path(self) -> SharedString;
}

impl<T: AdliSimge> From<T> for Simge {
    fn from(value: T) -> Self {
        Simge::build(value)
    }
}

simge_adli!(SimgeAdi, "../assets/assets/icons");

impl SimgeAdi {
    /// Simgeyi Entity<Simge> olarak döndürür.
    pub fn view(self, cx: &mut App) -> Entity<Simge> {
        Simge::build(self).view(cx)
    }
}

impl From<SimgeAdi> for AnyElement {
    fn from(val: SimgeAdi) -> Self {
        Simge::build(val).into_any_element()
    }
}

impl RenderOnce for SimgeAdi {
    fn render(self, _: &mut Window, _cx: &mut App) -> impl IntoElement {
        Simge::build(self)
    }
}

#[derive(IntoElement)]
pub struct Simge {
    base: Svg,
    style: StyleRefinement,
    path: SharedString,
    text_color: Option<Hsla>,
    size: Option<BilesenBoyutu>,
    rotation: Option<Radians>,
}

impl Default for Simge {
    fn default() -> Self {
        Self {
            base: svg().flex_none().size_4(),
            style: StyleRefinement::default(),
            path: "".into(),
            text_color: None,
            size: None,
            rotation: None,
        }
    }
}

impl Clone for Simge {
    fn clone(&self) -> Self {
        let mut this = Self::default().path(self.path.clone());
        this.style = self.style.clone();
        this.rotation = self.rotation;
        this.size = self.size;
        this.text_color = self.text_color;
        this
    }
}

impl Simge {
    pub fn new(icon: impl Into<Simge>) -> Self {
        icon.into()
    }

    fn build(name: impl AdliSimge) -> Self {
        Self::default().path(name.path())
    }

    /// Varlık paketindeki simge yolunu ayarlar.
    ///
    /// Örneğin: `simgeler/foo.svg`
    pub fn path(mut self, path: impl Into<SharedString>) -> Self {
        self.path = path.into();
        self
    }

    /// Simge için yeni bir görünüm oluşturur.
    pub fn view(self, cx: &mut App) -> Entity<Simge> {
        cx.new(|_| self)
    }

    pub fn transform(mut self, transformation: gpui::Transformation) -> Self {
        self.base = self.base.with_transformation(transformation);
        self
    }

    pub fn empty() -> Self {
        Self::default()
    }

    /// Simgeyi verilen açıyla döndürür.
    pub fn rotate(mut self, radians: impl Into<Radians>) -> Self {
        self.base = self
            .base
            .with_transformation(Transformation::rotate(radians));
        self
    }
}

impl Styled for Simge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }

    fn text_color(mut self, color: impl Into<Hsla>) -> Self {
        self.text_color = Some(color.into());
        self
    }
}

impl Boyutlandirilabilir for Simge {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl RenderOnce for Simge {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text_color = self.text_color.unwrap_or_else(|| window.text_style().color);
        let text_size = window.text_style().font_size.to_pixels(window.rem_size());
        let has_base_size = self.style.size.width.is_some() || self.style.size.height.is_some();

        let mut base = self.base;
        *base.style() = self.style;

        base.flex_shrink_0()
            .text_color(text_color)
            .when(!has_base_size, |this| this.size(text_size))
            .when_some(self.size, |this, size| match size {
                BilesenBoyutu::Ozel(px) => this.size(px),
                BilesenBoyutu::CokKucuk => this.size_3(),
                BilesenBoyutu::Kucuk => this.size_3p5(),
                BilesenBoyutu::Orta => this.size_4(),
                BilesenBoyutu::Buyuk => this.size_6(),
            })
            .path(self.path)
    }
}

impl From<Simge> for AnyElement {
    fn from(val: Simge) -> Self {
        val.into_any_element()
    }
}

impl Render for Simge {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let text_color = self.text_color.unwrap_or_else(|| cx.theme().foreground);
        let text_size = window.text_style().font_size.to_pixels(window.rem_size());
        let has_base_size = self.style.size.width.is_some() || self.style.size.height.is_some();

        let mut base = svg().flex_none();
        *base.style() = self.style.clone();

        base.flex_shrink_0()
            .text_color(text_color)
            .when(!has_base_size, |this| this.size(text_size))
            .when_some(self.size, |this, size| match size {
                BilesenBoyutu::Ozel(px) => this.size(px),
                BilesenBoyutu::CokKucuk => this.size_3(),
                BilesenBoyutu::Kucuk => this.size_3p5(),
                BilesenBoyutu::Orta => this.size_4(),
                BilesenBoyutu::Buyuk => this.size_6(),
            })
            .path(self.path.clone())
            .when_some(self.rotation, |this, rotation| {
                this.with_transformation(Transformation::rotate(rotation))
            })
    }
}
