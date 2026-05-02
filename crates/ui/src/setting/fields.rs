mod bool;
mod dropdown;
mod element;
mod number;
mod string;

pub(crate) use bool::*;
pub(crate) use dropdown::*;
pub(crate) use element::*;
pub(crate) use number::*;
pub(crate) use string::*;

pub use element::AyarAlaniOgesi;
pub use number::NumberFieldOptions;

use gpui::{AnyElement, App, IntoElement, SharedString, StyleRefinement, Styled, Window};
use std::{any::Any, rc::Rc};

use crate::setting::RenderOptions;

pub(crate) trait AyarAlaniCizimi {
    #[allow(clippy::too_many_arguments)]
    fn render(
        &self,
        field: Rc<dyn HerhangiBirAyarAlani>,
        options: &RenderOptions,
        style: &StyleRefinement,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement;
}

pub(crate) fn get_value<T: Clone + 'static>(
    field: &Rc<dyn HerhangiBirAyarAlani>,
    cx: &mut App,
) -> T {
    let setting_field = field
        .as_any()
        .downcast_ref::<AyarAlani<T>>()
        .expect("Ayar alanı aşağı türe dönüştürülemedi");
    (setting_field.value)(cx)
}

pub(crate) fn set_value<T: Clone + 'static>(
    field: &Rc<dyn HerhangiBirAyarAlani>,
    _cx: &mut App,
) -> Rc<dyn Fn(T, &mut App)> {
    let setting_field = field
        .as_any()
        .downcast_ref::<AyarAlani<T>>()
        .expect("Ayar alanı aşağı türe dönüştürülemedi");
    setting_field.set_value.clone()
}

/// tip ayar alan çizmek için.
#[derive(Clone)]
pub enum AyarAlaniTuru {
    Anahtar,
    OnayKutusu,
    NumberInput {
        options: NumberFieldOptions,
    },
    Input,
    Dropdown {
        options: Vec<(SharedString, SharedString)>,
        scrollable: bool,
    },
    Element {
        element: Rc<dyn AyarAlaniOgesi<Element = AnyElement>>,
    },
}

impl AyarAlaniTuru {
    #[inline]
    pub(crate) fn is_switch(&self) -> bool {
        matches!(self, AyarAlaniTuru::Anahtar)
    }

    #[inline]
    pub(crate) fn is_number_input(&self) -> bool {
        matches!(self, AyarAlaniTuru::NumberInput { .. })
    }

    #[inline]
    pub(crate) fn is_input(&self) -> bool {
        matches!(self, AyarAlaniTuru::Input)
    }

    #[inline]
    pub(crate) fn is_dropdown(&self) -> bool {
        matches!(self, AyarAlaniTuru::Dropdown { .. })
    }

    #[inline]
    pub(crate) fn is_element(&self) -> bool {
        matches!(self, AyarAlaniTuru::Element { .. })
    }

    #[inline]
    pub(super) fn dropdown_options(&self) -> Option<&Vec<(SharedString, SharedString)>> {
        match self {
            AyarAlaniTuru::Dropdown { options, .. } => Some(options),
            _ => None,
        }
    }

    #[inline]
    pub(super) fn dropdown_scrollable(&self) -> bool {
        match self {
            AyarAlaniTuru::Dropdown { scrollable, .. } => *scrollable,
            _ => false,
        }
    }

    #[inline]
    pub(super) fn number_input_options(&self) -> Option<&NumberFieldOptions> {
        match self {
            AyarAlaniTuru::NumberInput { options } => Some(options),
            _ => None,
        }
    }

    #[inline]
    pub(super) fn element(&self) -> Rc<dyn AyarAlaniOgesi<Element = AnyElement>> {
        match self {
            AyarAlaniTuru::Element { element } => element.clone(),
            _ => unreachable!("element_render called on non-element field"),
        }
    }
}

/// App içinde T tipinde değer alıp ayarlayabilen ayar alanı.
pub struct AyarAlani<T> {
    pub(crate) field_type: AyarAlaniTuru,
    pub(crate) style: StyleRefinement,
    /// fonksiyon almak için değer için bu alan.
    pub(crate) value: Rc<dyn Fn(&App) -> T>,
    /// fonksiyon ayarlamak için değer için bu alan.
    pub(crate) set_value: Rc<dyn Fn(T, &mut App)>,
    pub(crate) default_value: Option<T>,
}

impl AyarAlani<bool> {
    /// Yeni bir Anahtar alan oluşturur.
    pub fn switch<V, S>(value: V, set_value: S) -> Self
    where
        V: Fn(&App) -> bool + 'static,
        S: Fn(bool, &mut App) + 'static,
    {
        Self::new(AyarAlaniTuru::Anahtar, value, set_value)
    }

    /// Yeni bir OnayKutusu alan oluşturur.
    pub fn checkbox<V, S>(value: V, set_value: S) -> Self
    where
        V: Fn(&App) -> bool + 'static,
        S: Fn(bool, &mut App) + 'static,
    {
        Self::new(AyarAlaniTuru::OnayKutusu, value, set_value)
    }
}

impl AyarAlani<SharedString> {
    /// Yeni bir girdi alan oluşturur.
    pub fn input<V, S>(value: V, set_value: S) -> Self
    where
        V: Fn(&App) -> SharedString + 'static,
        S: Fn(SharedString, &mut App) + 'static,
    {
        Self::new(AyarAlaniTuru::Input, value, set_value)
    }

    /// Yeni bir Dropdown alan ile verilen seçenekler oluşturur.
    ///
    /// açılır pencere menü does değil kaydırma. For long seçenek lists olan may exceed
    /// görünüm alanı, kullanım [`Self::scrollable_dropdown`] bunun yerine.
    pub fn dropdown<V, S>(
        options: Vec<(SharedString, SharedString)>,
        value: V,
        set_value: S,
    ) -> Self
    where
        V: Fn(&App) -> SharedString + 'static,
        S: Fn(SharedString, &mut App) + 'static,
    {
        Self::new(
            AyarAlaniTuru::Dropdown {
                options,
                scrollable: false,
            },
            value,
            set_value,
        )
    }

    /// Yeni bir Dropdown alan whose açılır pencere menü scrolls olduğunda onun içerik oluşturur.
    /// exceeds görünüm alanı. Kullanım bu için long seçenek lists yerde
    /// non-kaydırma [`Self::dropdown`] olurdu push öğeler below katlama.
    pub fn scrollable_dropdown<V, S>(
        options: Vec<(SharedString, SharedString)>,
        value: V,
        set_value: S,
    ) -> Self
    where
        V: Fn(&App) -> SharedString + 'static,
        S: Fn(SharedString, &mut App) + 'static,
    {
        Self::new(
            AyarAlaniTuru::Dropdown {
                options,
                scrollable: true,
            },
            value,
            set_value,
        )
    }

    /// [`AyarAlaniOgesi`] traitini uygulayan özel öğeyle yeni bir ayar alanı oluşturur.
    ///
    /// Bakınız ayrıca [`AyarAlani::render`] için simply building ile bir çizer kapanış.
    pub fn element<E>(element: E) -> Self
    where
        E: AyarAlaniOgesi + 'static,
    {
        Self::new(
            AyarAlaniTuru::Element {
                element: Rc::new(HerhangiBirAyarAlaniOgesi(element)),
            },
            |_| SharedString::default(),
            |_, _| {},
        )
    }

    /// Yeni bir ayar alan ile verilen öğe çizer kapanış oluşturur.
    ///
    /// Bakınız ayrıca [`AyarAlani::öğe`] için building ile özel alan için daha fazla complex scenarios.
    pub fn render<E, R>(element_render: R) -> Self
    where
        E: IntoElement + 'static,
        R: Fn(&RenderOptions, &mut Window, &mut App) -> E + 'static,
    {
        Self::element(
            move |options: &RenderOptions, window: &mut Window, cx: &mut App| {
                (element_render)(options, window, cx).into_any_element()
            },
        )
    }
}

impl AyarAlani<f64> {
    /// Yeni bir Sayı girdi alan ile verilen seçenekler oluşturur.
    pub fn number_input<V, S>(options: NumberFieldOptions, value: V, set_value: S) -> Self
    where
        V: Fn(&App) -> f64 + 'static,
        S: Fn(f64, &mut App) + 'static,
    {
        Self::new(AyarAlaniTuru::NumberInput { options }, value, set_value)
    }
}

impl<T> AyarAlani<T> {
    /// Yeni bir ayar alan ile verilen alır ve ayarlar fonksiyonlar oluşturur.
    fn new<V, S>(field_type: AyarAlaniTuru, value: V, set_value: S) -> Self
    where
        V: Fn(&App) -> T + 'static,
        S: Fn(T, &mut App) + 'static,
    {
        Self {
            field_type,
            style: StyleRefinement::default(),
            value: Rc::new(value),
            set_value: Rc::new(set_value),
            default_value: None,
        }
    }

    /// Bu ayar alanının varsayılan değerini ayarlar. Varsayılan None değeridir.
    ///
    /// ayarlar ise, bu değer olabilir için kullanılır sıfırlar ayar için onun varsayılan durum.
    /// değil ayarlar ise, ayar olamaz sıfırlar.
    pub fn default_value(mut self, default_value: impl Into<T>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }
}

impl<T> Styled for AyarAlani<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// Dinamik tiplemeye izin veren ayar alanları için trait.
pub trait HerhangiBirAyarAlani {
    fn as_any(&self) -> &dyn std::any::Any;
    fn type_name(&self) -> &'static str;
    fn type_id(&self) -> std::any::TypeId;
    fn field_type(&self) -> &AyarAlaniTuru;
    fn style(&self) -> &StyleRefinement;
    fn is_resettable(&self, cx: &App) -> bool;
    fn reset(&self, window: &mut Window, cx: &mut App);
}

impl<T: Clone + PartialEq + Send + Sync + 'static> HerhangiBirAyarAlani for AyarAlani<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<T>()
    }

    fn field_type(&self) -> &AyarAlaniTuru {
        &self.field_type
    }

    fn style(&self) -> &StyleRefinement {
        &self.style
    }

    fn is_resettable(&self, cx: &App) -> bool {
        let Some(default_value) = self.default_value.as_ref() else {
            return false;
        };

        &(self.value)(cx) != default_value
    }

    fn reset(&self, _: &mut Window, cx: &mut App) {
        let Some(default_value) = self.default_value.as_ref() else {
            return;
        };

        (self.set_value)(default_value.clone(), cx)
    }
}
