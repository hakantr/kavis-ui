use std::ops::Deref;

mod async_util;
pub mod bilesenler;
pub mod cekirdek;
mod element_ext;
mod event;
mod focus_trap;
mod geometry;
pub mod global_state;
pub mod gpui_turkce;
mod i18n;
mod icon;
mod index_path;
#[cfg(any(feature = "inspector", debug_assertions))]
mod inspector;
mod root;
mod styled;
mod time;
mod title_bar;
mod virtual_list;
mod window_border;
mod window_ext;

pub(crate) mod actions;

pub mod accordion;
pub mod alert;
pub mod animation;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod chart;
pub mod checkbox;
pub mod clipboard;
pub mod collapsible;
pub mod color_picker;
pub mod description_list;
pub mod dialog;
pub mod divider;
pub mod dock;
pub mod form;
pub mod group_box;
pub mod highlighter;
pub mod history;
pub mod hover_card;
pub mod input;
pub mod kbd;
pub mod label;
pub mod link;
pub mod list;
pub mod menu;
pub mod notification;
pub mod pagination;
pub mod plot;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod rating;
pub mod resizable;
pub mod scroll;
pub mod select;
pub mod setting;
pub mod sheet;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod stepper;
pub mod switch;
pub mod tab;
pub mod table;
pub mod tag;
pub mod text;
pub mod theme;
pub mod tooltip;
pub mod tree;

pub use crate::DevreDisiBirakilabilir;
pub use bilesenler::*;
pub use cekirdek::*;
pub use element_ext::*;
pub use event::EtkilesimliOgeUzantisi;
pub use focus_trap::OdakTuzagiOgesi;
pub use geometry::*;
pub use global_state::KureselDurum;
pub use icon::*;
pub use index_path::IndexPath;
pub use input::{Rope, RopeExt, RopeLines};
#[cfg(any(feature = "inspector", debug_assertions))]
pub use inspector::*;
pub use kavis_ui_macros::simge_adli;
pub use root::KokGorunum;
pub use styled::*;
pub use theme::*;
pub use time::{calendar, date_input, date_picker};
pub use title_bar::*;
pub use virtual_list::{SanalListe, SanalListeKaydirmaTutamaci, h_virtual_list, v_virtual_list};
pub use window_border::{PencereKenarligi, window_border, window_paddings};
pub use window_ext::PencereUzantisi;

/// Turkce adlandirilmis GPUI yuzeyi.
pub mod turkce {
    pub use crate::gpui_turkce::*;
    pub use crate::menu::{
        UygulamaMenuCubugu, UygulamaMenuOgesi, UygulamaMenusu, uygulama_menu_cubugu_gerekli_mi,
        uygulama_menu_cubugu_olustur, uygulama_menu_cubugu_olustur_baglam,
        uygulama_menu_cubugu_olustur_zorla, uygulama_menu_cubugu_olustur_zorla_baglam,
        uygulama_menulerini_kaydet,
    };
}

/// GPUI'nin uygulama tarafinda sik gereken API yuzeyini kavis-ui uzerinden
/// disa aktarir.
///
/// `Size` gibi kavis-ui'nin kendi Turkce bilesen adlariyla cakisan GPUI
/// tipleri root'a ayni adla cikarilmaz; onlar icin [`Boyut`] veya
/// [`ham_gpui`] kullanilabilir.
pub use gpui::{
    Action, AnyElement, AnyView, AnyWindowHandle, App, AppContext, Application, Asset, AssetSource,
    Axis, Background, Bounds, BoxShadow, ClickEvent, Context, CursorStyle, Decorations,
    DefiniteLength, Div, DummyKeyboardMapper, Edges, Element, ElementId, Entity, EventEmitter,
    FocusHandle, Focusable, Global, HighlightStyle, Hsla, InteractiveElement, IntoElement,
    KeyBinding, KeyBindingContextPredicate, Length, Menu, MenuItem, MouseButton, OwnedMenu,
    OwnedMenuItem, ParentElement, Path, PathBuilder, Pixels, Platform, Point, Render, RenderOnce,
    ResizeEdge, Result, Rgba, SharedString, StatefulInteractiveElement, Styled, StyledText,
    Subscription, Task, TextAlign, TextStyle, Tiling, TitlebarOptions, VisualContext, WeakEntity,
    Window, WindowAppearance, WindowBackgroundAppearance, WindowBounds, WindowControlArea,
    WindowDecorations, WindowHandle, WindowId, WindowOptions, black, blue, deferred, div, fill,
    green, hsla, img, point, px, red, relative, rems, rgb, rgba, size, svg, transparent_black,
    transparent_white, uniform_list,
};

#[cfg(feature = "test-support")]
pub use gpui::{TestAppContext, TestDispatcher, VisualTestContext};

/// Zorunlu durumlarda ham GPUI ad alanına kavis-ui içinden erişim sağlar.
///
/// Yeni uygulama kodunda önce Türkçe alias ve sarmalayıcılar tercih edilmeli;
/// bu modül, henüz Türkçe karşılığı eklenmemiş düşük seviye API'ler için
/// kaçış kapısıdır.
pub mod ham_gpui {
    pub use gpui::*;
}

/// `gpui_platform` uygulama kurulum yuzeyini kavis-ui uzerinden aktarir.
pub mod platform {
    pub use gpui_platform::*;
}

/// `gpui::prelude` icerigini ham haliyle disa aktarir.
///
/// Türkçe isimli prelude için bkz. [`onsoz`].
pub mod gpui_onsoz {
    pub use gpui::prelude::*;
}

/// Sik kullanilan Turkce kavis-ui adlarini tek import ile getirir.
///
/// Ham GPUI prelude için bkz. [`gpui_onsoz`].
pub mod onsoz {
    pub use crate::gpui_turkce::onsoz::*;
}

/// `gpui::actions!` yerine kavis-ui üzerinden aksiyon tipi üretir.
///
/// GPUI'nin orijinal makrosu genişlediğinde tüketici crate içinde `gpui`
/// adında doğrudan bir bağımlılık arar. Bu makro aynı işi `kavis_ui::Action`
/// üzerinden yapar; böylece uygulama crate'leri yalnızca kavis-ui'yi bilir.
#[macro_export]
macro_rules! aksiyonlar {
    ($namespace:path, [ $( $(#[$attr:meta])* $name:ident),* $(,)? ]) => {
        #[allow(unused_imports)]
        use $crate::ham_gpui as gpui;
        $(
            #[derive(
                ::std::clone::Clone,
                ::std::cmp::PartialEq,
                ::std::default::Default,
                ::std::fmt::Debug,
                $crate::Action
            )]
            #[action(namespace = $namespace)]
            $(#[$attr])*
            pub struct $name;
        )*
    };
    ([ $( $(#[$attr:meta])* $name:ident),* $(,)? ]) => {
        #[allow(unused_imports)]
        use $crate::ham_gpui as gpui;
        $(
            #[derive(
                ::std::clone::Clone,
                ::std::cmp::PartialEq,
                ::std::default::Default,
                ::std::fmt::Debug,
                $crate::Action
            )]
            $(#[$attr])*
            pub struct $name;
        )*
    };
}

rust_i18n::i18n!("locales", fallback = "en");

/// Bileşenleri başlatır.
///
/// Bileşenleri uygulamanızın giriş noktasında başlatmanız gerekir.
pub fn init(cx: &mut gpui::App) {
    theme::init(cx);
    global_state::init(cx);
    #[cfg(any(feature = "inspector", debug_assertions))]
    inspector::init(cx);
    root::init(cx);
    focus_trap::init(cx);
    color_picker::init(cx);
    date_picker::init(cx);
    dock::init(cx);
    sheet::init(cx);
    select::init(cx);
    input::init(cx);
    list::init(cx);
    dialog::init(cx);
    popover::init(cx);
    menu::init(cx);
    table::init(cx);
    text::init(cx);
    tree::init(cx);
    tooltip::init(cx);
}

#[inline]
pub fn locale() -> impl Deref<Target = str> {
    rust_i18n::locale()
}

#[inline]
pub fn set_locale(locale: &str) {
    i18n::set_locale_override(locale);
    rust_i18n::set_locale(locale)
}

#[inline]
pub(crate) fn olcum_etkin() -> bool {
    std::env::var("ZED_MEASUREMENTS").is_ok() || std::env::var("GPUI_MEASUREMENTS").is_ok()
}

/// Bir fonksiyonun çalışma süresini ölçer ve `if_` true ise günlüğe yazar.
///
/// `GPUI_MEASUREMENTS=1` ortam değişkeni gerekir
#[inline]
#[track_caller]
pub fn kosullu_olc(name: impl Into<gpui::SharedString>, if_: bool, f: impl FnOnce()) {
    if if_ && olcum_etkin() {
        let measure = Olcum::new(name);
        f();
        measure.end();
    } else {
        f();
    }
}

/// Çalışma süresini ölçer.
#[inline]
#[track_caller]
pub fn measure(name: impl Into<gpui::SharedString>, f: impl FnOnce()) {
    kosullu_olc(name, true, f);
}

pub struct Olcum {
    name: gpui::SharedString,
    start: std::time::Instant,
}

impl Olcum {
    #[track_caller]
    pub fn new(name: impl Into<gpui::SharedString>) -> Self {
        Self {
            name: name.into(),
            start: std::time::Instant::now(),
        }
    }

    #[track_caller]
    pub fn end(self) {
        let duration = self.start.elapsed();
        tracing::trace!("{} şu sürede tamamlandı: {:?}", self.name, duration);
    }
}
