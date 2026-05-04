// crates/ui/src/geometry.rs
use std::fmt::{self, Debug, Display, Formatter};

use gpui::{AbsoluteLength, Axis, Length, Pixels};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Bir enum için defining yerleşim öğe.
///
/// Sol veya sağ tarafı tanımlamanız gerekiyorsa ayrıca [`Side`] tipine bakın.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum Placement {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}

impl Display for Placement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Placement::Top => write!(f, "Top"),
            Placement::Bottom => write!(f, "Bottom"),
            Placement::Left => write!(f, "Left"),
            Placement::Right => write!(f, "Right"),
        }
    }
}

impl Placement {
    #[inline]
    pub fn yatay_mi(&self) -> bool {
        match self {
            Placement::Left | Placement::Right => true,
            _ => false,
        }
    }

    #[inline]
    pub fn dikey_mi(&self) -> bool {
        match self {
            Placement::Top | Placement::Bottom => true,
            _ => false,
        }
    }

    #[inline]
    pub fn axis(&self) -> Axis {
        match self {
            Placement::Top | Placement::Bottom => Axis::Vertical,
            Placement::Left | Placement::Right => Axis::Horizontal,
        }
    }
}

// The local Anchor enum has been removed. Use gpui::Anchor instead.

/// Bir enum için defining taraf öğe.
///
/// Dört kenarı tanımlamanız gerekiyorsa ayrıca [`Placement`] tipine bakın.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}

impl Side {
    /// Taraf sol ise true döndürür.
    #[inline]
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left)
    }

    /// Taraf sağ ise true döndürür.
    #[inline]
    pub fn is_right(&self) -> bool {
        matches!(self, Self::Right)
    }
}

/// Bir özellik için extend [`Axis`] enum ile utility yöntemler.
pub trait EksenUzantisi {
    fn yatay_mi(self) -> bool;
    fn dikey_mi(self) -> bool;
}

impl EksenUzantisi for Axis {
    #[inline]
    fn yatay_mi(self) -> bool {
        self == Axis::Horizontal
    }

    #[inline]
    fn dikey_mi(self) -> bool {
        self == Axis::Vertical
    }
}

/// Bir özellik için extend [`Length`] enum ile utility yöntemler.
pub trait UzunlukUzantisi {
    /// [`Length`] için [`Pixels`] temelli üzerinde bir verilen `base_size` ve `rem_size`. dönüştürür.
    ///
    /// [`Length`] değeri [`Length::Auto`] ise `None` döndürür.
    fn piksele_cevir(&self, base_size: AbsoluteLength, rem_size: Pixels) -> Option<Pixels>;
}

impl UzunlukUzantisi for Length {
    fn piksele_cevir(&self, base_size: AbsoluteLength, rem_size: Pixels) -> Option<Pixels> {
        match self {
            Length::Auto => None,
            Length::Definite(len) => Some(len.to_pixels(base_size, rem_size)),
        }
    }
}

/// Bir struct için defining edges bir öğe.
///
/// Bir extend version [`gpui::Edges`] için serialize/deserialize.
#[allow(dead_code)]
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Eq, PartialEq)]
#[repr(C)]
pub struct Edges<T: Clone + Debug + Default + PartialEq> {
    /// boyut üst kenar.
    pub top: T,
    /// boyut sağ kenar.
    pub right: T,
    /// boyut alt kenar.
    pub bottom: T,
    /// boyut sol kenar.
    pub left: T,
}

impl<T> Edges<T>
where
    T: Clone + Debug + Default + PartialEq,
{
    #[allow(dead_code)]
    /// Yeni bir `Edges` örnek ile tüm edges ayarlar için aynı değer oluşturur.
    pub fn all(value: T) -> Self {
        Self {
            top: value.clone(),
            right: value.clone(),
            bottom: value.clone(),
            left: value,
        }
    }
}
