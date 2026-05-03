//! GPUI API'si icin Turkce adlandirilmis erisim yuzeyi.
//!
//! Bu modul GPUI'yi uygulama crate'lerine dogrudan bagimlilik olarak
//! tasitmadan, Kavis UI uzerinden Turkce isimlerle kullanilabilir hale
//! getirir. GPUI'de yeni bir API'ye gecici olarak ihtiyac duyulursa
//! [`crate::ham_gpui`] tum ham yuzeyi yine kavis-ui icinden saglar.

use std::{fmt::Debug, ops::Range};

pub use gpui::{
    Action as Aksiyon, Along as EksenBoyunca, AnyElement as HerhangiOge,
    AnyView as HerhangiGorunum, AnyWindowHandle as HerhangiPencereTutamaci, App as Uygulama,
    AppContext as UygulamaBaglami, Application as UygulamaKurulumu, ArcCow as PaylasimliSahiplik,
    Asset as VarlikDosyasi, AssetSource as VarlikKaynagi, AvailableSpace as KullanilabilirAlan,
    Background as ArkaPlan, BorrowAppContext as OduncUygulamaBaglami, Bounds as Sinir,
    BoxShadow as KutuGolgesi, ClickEvent as TiklamaOlayi, ContentMask as IcerikMaskesi,
    Context as Baglam, Corners as Koseler, CursorStyle as ImlecStili,
    Decorations as PencereSuslemeleri, DefiniteLength as KesinUzunluk, DisplayId as EkranKimligi,
    Div as Bolme, Element as Oge, ElementId as OgeKimligi, Empty as BosOge, Entity as Varlik,
    EntityId as VarlikKimligi, EventEmitter as OlayYayici, FocusHandle as OdakTutamaci,
    Focusable as Odaklanabilir, FontFeatures as YaziTipiOzellikleri, FontStyle as YaziTipiStili,
    FontWeight as YaziTipiKalinligi, Global as Kuresel, HighlightStyle as VurguStili,
    Hsla as HslaRenk, ImageSource as GorselKaynagi, InteractiveElement as EtkilesimliOge,
    IntoElement as OgeyeDonus, KeyBinding as TusBaglamasi,
    KeyBindingContextPredicate as TusBaglamasiBaglamKosulu, Length as Uzunluk, Menu as YerelMenu,
    MenuItem as YerelMenuOgesi, MouseButton as FareDugmesi, OwnedMenu as SahipliMenu,
    OwnedMenuItem as SahipliMenuOgesi, ParentElement as EbeveynOge, Path as CizimYolu,
    PathBuilder as CizimYoluOlusturucu, Pixels as Piksel, Platform as CalismaPlatformu,
    Point as Nokta, Refineable as Inceltebilir, Rems as Rem, Render as Ciz,
    RenderOnce as BirKezCiz, ResizeEdge as BoyutlandirmaKenari, Result as GpuiSonucu,
    Rgba as RgbaRenk, SharedString as PaylasimliMetin, Stateful as Durumlu,
    StatefulInteractiveElement as DurumluEtkilesimliOge, Styled as Stilli,
    StyledText as StilliMetin, Subscription as Abonelik, Task as Gorev, TextAlign as MetinHizasi,
    TextStyle as MetinStili, Tiling as Doseme, TitlebarOptions as BaslikCubuguSecenekleri,
    VisualContext as GorselBaglam, WeakEntity as ZayifVarlik, Window as Pencere,
    WindowAppearance as PencereGorunumu, WindowBackgroundAppearance as PencereArkaPlanGorunumu,
    WindowBounds as PencereSinirlari, WindowControlArea as PencereKontrolAlani,
    WindowDecorations as PencereDekorasyonlari, WindowHandle as PencereTutamaci,
    WindowId as PencereKimligi, WindowOptions as PencereSecenekleri,
};

#[cfg(feature = "test-support")]
pub use gpui::{
    TestAppContext as TestUygulamaBaglami, TestDispatcher as TestDagiticisi,
    VisualTestContext as GorselTestBaglami,
};

/// GPUI [`gpui::Axis`] enum'unun Turkce varyantli karsiligi.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Eksen {
    /// Y ekseni; yukari/asagi yon.
    Dikey,
    /// X ekseni; sol/sag yon.
    Yatay,
}

impl Eksen {
    pub fn ters_cevir(self) -> Self {
        match self {
            Self::Dikey => Self::Yatay,
            Self::Yatay => Self::Dikey,
        }
    }
}

impl From<Eksen> for gpui::Axis {
    fn from(eksen: Eksen) -> Self {
        match eksen {
            Eksen::Dikey => Self::Vertical,
            Eksen::Yatay => Self::Horizontal,
        }
    }
}

impl From<gpui::Axis> for Eksen {
    fn from(eksen: gpui::Axis) -> Self {
        match eksen {
            gpui::Axis::Vertical => Self::Dikey,
            gpui::Axis::Horizontal => Self::Yatay,
        }
    }
}

/// Isletim sistemi tarafindan ozel ele alinan standart duzenleme aksiyonlari.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IsletimSistemiAksiyonu {
    Kes,
    Kopyala,
    Yapistir,
    TumunuSec,
    GeriAl,
    Yinele,
}

impl From<IsletimSistemiAksiyonu> for gpui::OsAction {
    fn from(aksiyon: IsletimSistemiAksiyonu) -> Self {
        match aksiyon {
            IsletimSistemiAksiyonu::Kes => Self::Cut,
            IsletimSistemiAksiyonu::Kopyala => Self::Copy,
            IsletimSistemiAksiyonu::Yapistir => Self::Paste,
            IsletimSistemiAksiyonu::TumunuSec => Self::SelectAll,
            IsletimSistemiAksiyonu::GeriAl => Self::Undo,
            IsletimSistemiAksiyonu::Yinele => Self::Redo,
        }
    }
}

impl From<gpui::OsAction> for IsletimSistemiAksiyonu {
    fn from(aksiyon: gpui::OsAction) -> Self {
        match aksiyon {
            gpui::OsAction::Cut => Self::Kes,
            gpui::OsAction::Copy => Self::Kopyala,
            gpui::OsAction::Paste => Self::Yapistir,
            gpui::OsAction::SelectAll => Self::TumunuSec,
            gpui::OsAction::Undo => Self::GeriAl,
            gpui::OsAction::Redo => Self::Yinele,
        }
    }
}

/// Isletim sistemi tarafindan doldurulan sistem menusu turleri.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SistemMenusuTuru {
    /// macOS uygulama menusundeki Servisler alt menusu.
    Servisler,
}

impl From<SistemMenusuTuru> for gpui::SystemMenuType {
    fn from(tur: SistemMenusuTuru) -> Self {
        match tur {
            SistemMenusuTuru::Servisler => Self::Services,
        }
    }
}

impl From<gpui::SystemMenuType> for SistemMenusuTuru {
    fn from(tur: gpui::SystemMenuType) -> Self {
        match tur {
            gpui::SystemMenuType::Services => Self::Servisler,
        }
    }
}

/// GPUI `div()` karsiligi.
#[inline]
pub fn bolme() -> gpui::Div {
    gpui::div()
}

/// GPUI `px()` karsiligi.
#[inline]
pub const fn piksel(deger: f32) -> gpui::Pixels {
    gpui::px(deger)
}

/// GPUI `rems()` karsiligi.
#[inline]
pub const fn rem(deger: f32) -> gpui::Rems {
    gpui::rems(deger)
}

/// GPUI `relative()` karsiligi.
#[inline]
pub const fn goreli(oran: f32) -> gpui::DefiniteLength {
    gpui::relative(oran)
}

/// GPUI `point()` karsiligi.
#[inline]
pub const fn nokta<T>(x: T, y: T) -> gpui::Point<T>
where
    T: Clone + Debug + Default + PartialEq,
{
    gpui::point(x, y)
}

/// GPUI `size()` karsiligi.
#[inline]
pub const fn boyut<T>(genislik: T, yukseklik: T) -> gpui::Size<T>
where
    T: Clone + Debug + Default + PartialEq,
{
    gpui::size(genislik, yukseklik)
}

/// GPUI `rgb()` karsiligi.
#[inline]
pub fn renk(hex: u32) -> gpui::Rgba {
    gpui::rgb(hex)
}

/// GPUI `rgba()` karsiligi.
#[inline]
pub fn renk_alfa(hex: u32) -> gpui::Rgba {
    gpui::rgba(hex)
}

/// GPUI `hsla()` karsiligi.
#[inline]
pub fn hsla_renk(h: f32, s: f32, l: f32, a: f32) -> gpui::Hsla {
    gpui::hsla(h, s, l, a)
}

/// GPUI `img()` karsiligi.
#[inline]
pub fn gorsel(kaynak: impl Into<gpui::ImageSource>) -> gpui::Img {
    gpui::img(kaynak)
}

/// GPUI `svg()` karsiligi.
#[inline]
pub fn svg_oge() -> gpui::Svg {
    gpui::svg()
}

/// GPUI `uniform_list()` karsiligi.
#[inline]
pub fn tekbicim_liste<R>(
    id: impl Into<gpui::ElementId>,
    item_count: usize,
    render_items: impl Fn(Range<usize>, &mut gpui::Window, &mut gpui::App) -> Vec<R> + 'static,
) -> gpui::UniformList
where
    R: gpui::IntoElement,
{
    gpui::uniform_list(id, item_count, render_items)
}

/// Sik kullanilan Turkce GPUI adlarini tek import ile getirir.
pub mod onsoz {
    pub use super::{
        Abonelik, Aksiyon, Baglam, BirKezCiz, Bolme, BoyutlandirmaKenari, CalismaPlatformu, Ciz,
        EbeveynOge, Eksen, EtkilesimliOge, FareDugmesi, GorselBaglam, HerhangiGorunum, HerhangiOge,
        IsletimSistemiAksiyonu, Nokta, OdakTutamaci, Odaklanabilir, Oge, OgeKimligi, OgeyeDonus,
        PaylasimliMetin, Pencere, PencereKontrolAlani, PencereSecenekleri, PencereTutamaci, Piksel,
        RgbaRenk, SahipliMenu, SahipliMenuOgesi, Sinir, Stilli, Uygulama, UygulamaBaglami, Varlik,
        YerelMenu, YerelMenuOgesi, ZayifVarlik, bolme, boyut, goreli, gorsel, hsla_renk, nokta,
        piksel, rem, renk, renk_alfa, svg_oge,
    };
}
