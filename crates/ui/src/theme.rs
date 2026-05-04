use crate::{
    highlighter::VurguTemasi, list::ListeAyarlari, notification::BildirimAyarlari,
    scroll::KaydirmaCubuguGosterimi, sheet::SayfaKatmaniAyarlari,
};
use crate::ham_gpui::{App, Global, Hsla, Pixels, SharedString, Window, WindowAppearance, px};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
};

mod color;
mod registry;
mod schema;
mod theme_color;

pub use color::*;
pub use registry::*;
pub use schema::*;
pub use theme_color::*;

pub fn init(cx: &mut App) {
    registry::init(cx);

    // Ensure theme is loaded directly on startup for WASM compatibility
    Tema::change(TemaModu::Light, None, cx);
    Tema::sync_scrollbar_appearance(cx);
}

pub trait EtkinTema {
    fn theme(&self) -> &Tema;
}

impl EtkinTema for App {
    #[inline(always)]
    fn theme(&self) -> &Tema {
        Tema::global(self)
    }
}

/// global tema yapılandırma.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Tema {
    pub colors: TemaRengi,
    pub highlight_theme: Arc<VurguTemasi>,
    pub light_theme: Rc<TemaYapilandirmasi>,
    pub dark_theme: Rc<TemaYapilandirmasi>,

    pub mode: TemaModu,
    /// Uygulama font ailesini ayarlar. Varsayılan `.SystemUIFont` değeridir.
    pub font_family: SharedString,
    /// Uygulama için taban font boyutunu ayarlar. Varsayılan 16pxdir.
    pub font_size: Pixels,
    /// monospace font family için uygulama.
    ///
    /// Defaults için:
    ///
    /// - macOS: `Menlo`
    /// - Windows: `Consolas`
    /// - Linux: `DejaVu Sans Mono`
    pub mono_font_family: SharedString,
    /// Uygulama için monospace yazı tipi boyutu. Varsayılan 13px.
    pub mono_font_size: Pixels,
    /// Genel öğeler için yarıçap.
    pub radius: Pixels,
    /// IletisimKutusu ve Bildirim gibi büyük öğeler için kenarlık yarıçapı.
    pub radius_lg: Pixels,
    pub shadow: bool,
    pub transparent: Hsla,
    /// Kaydırma çubuğu gösterim modunu ayarlar. Varsayılan: Scrolling.
    pub scrollbar_show: KaydirmaCubuguGosterimi,
    /// Bildirim ayarı.
    #[serde(skip)]
    pub notification: BildirimAyarlari,
    /// Döşeme ızgarası boyutu. Varsayılan 4px.
    pub tile_grid_size: Pixels,
    /// Döşeme paneli gölgesi.
    pub tile_shadow: bool,
    /// Döşeme paneli kenarlık yarıçapı. Varsayılan 0px.
    pub tile_radius: Pixels,
    /// Liste ayarları.
    pub list: ListeAyarlari,
    /// Sayfa katmanı ayarları.
    pub sheet: SayfaKatmaniAyarlari,
}

impl Default for Tema {
    fn default() -> Self {
        Self::from(&TemaRengi::default())
    }
}

impl Deref for Tema {
    type Target = TemaRengi;

    fn deref(&self) -> &Self::Target {
        &self.colors
    }
}

impl DerefMut for Tema {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.colors
    }
}

impl Global for Tema {}

impl Tema {
    /// global tema referans döndürür.
    #[inline(always)]
    pub fn global(cx: &App) -> &Tema {
        cx.global::<Tema>()
    }

    /// global tema mutable referans döndürür.
    #[inline(always)]
    pub fn global_mut(cx: &mut App) -> &mut Tema {
        cx.global_mut::<Tema>()
    }

    /// Tema koyuysa true döndürür.
    #[inline(always)]
    pub fn is_dark(&self) -> bool {
        self.mode.is_dark()
    }

    /// geçerli tema ad döndürür.
    pub fn theme_name(&self) -> &SharedString {
        if self.is_dark() {
            &self.dark_theme.name
        } else {
            &self.light_theme.name
        }
    }

    /// Sync tema ile sistem görünüm
    pub fn sync_system_appearance(window: Option<&mut Window>, cx: &mut App) {
        // Better use window.appearance() for avoid error on Linux.
        // https://github.com/hakantr/kavis-ui/issues/104
        let appearance = window
            .as_ref()
            .map(|window| window.appearance())
            .unwrap_or_else(|| cx.window_appearance());

        Self::change(appearance, window, cx);
    }

    /// KaydirmaCubugu gösterim davranışını sistemle eşitler.
    pub fn sync_scrollbar_appearance(cx: &mut App) {
        Tema::global_mut(cx).scrollbar_show = if cx.should_auto_hide_scrollbars() {
            KaydirmaCubuguGosterimi::Scrolling
        } else {
            KaydirmaCubuguGosterimi::Hover
        };
    }

    /// Change tema mod.
    pub fn change(mode: impl Into<TemaModu>, window: Option<&mut Window>, cx: &mut App) {
        let mode = mode.into();
        if !cx.has_global::<Tema>() {
            let mut theme = Tema::default();
            theme.light_theme = TemaKaydi::global(cx).default_light_theme().clone();
            theme.dark_theme = TemaKaydi::global(cx).default_dark_theme().clone();
            cx.set_global(theme);
        }

        let theme = cx.global_mut::<Tema>();
        theme.mode = mode;
        if mode.is_dark() {
            theme.apply_config(&theme.dark_theme.clone());
        } else {
            theme.apply_config(&theme.light_theme.clone());
        }

        if let Some(window) = window {
            window.refresh();
        }
    }

    /// girdi arka plan renk döndürür.
    ///
    /// For koyu, kullanım bir transparent renk mixed ile girdi kenarlık: `cx.theme().girdi`,
    /// Aksi halde `cx.theme().arka plan` rengini kullanır.
    #[inline]
    pub fn input_background(&self) -> Hsla {
        if self.is_dark() {
            self.input.mix_oklab(self.transparent, 0.3)
        } else {
            self.background
        }
    }

    /// düzenleyici arka plan renk, ise değil ayarlar, kullanım girdi arka plan renk döndürür.
    #[inline]
    pub(crate) fn editor_background(&self) -> Hsla {
        self.highlight_theme
            .style
            .editor_background
            .unwrap_or_else(|| self.input_background())
    }
}

impl From<&TemaRengi> for Tema {
    fn from(colors: &TemaRengi) -> Self {
        Tema {
            mode: TemaModu::default(),
            transparent: Hsla::transparent_black(),
            font_family: ".SystemUIFont".into(),
            font_size: px(16.),
            mono_font_family: if cfg!(target_os = "macos") {
                // https://en.wikipedia.org/wiki/Menlo_(typeface)
                "Menlo".into()
            } else if cfg!(target_os = "windows") {
                "Consolas".into()
            } else {
                "DejaVu Sans Mono".into()
            },
            mono_font_size: px(13.),
            radius: px(6.),
            radius_lg: px(8.),
            shadow: true,
            scrollbar_show: KaydirmaCubuguGosterimi::default(),
            notification: BildirimAyarlari::default(),
            tile_grid_size: px(8.),
            tile_shadow: true,
            tile_radius: px(0.),
            list: ListeAyarlari::default(),
            colors: *colors,
            light_theme: Rc::new(TemaYapilandirmasi::default()),
            dark_theme: Rc::new(TemaYapilandirmasi::default()),
            highlight_theme: VurguTemasi::default_light(),
            sheet: SayfaKatmaniAyarlari::default(),
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum TemaModu {
    #[default]
    Light,
    Dark,
}

impl TemaModu {
    #[inline(always)]
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Dark)
    }

    /// lower_case tema ad: `light`, `dark`. döndürür.
    pub fn name(&self) -> &'static str {
        match self {
            TemaModu::Light => "light",
            TemaModu::Dark => "dark",
        }
    }
}

impl From<WindowAppearance> for TemaModu {
    fn from(appearance: WindowAppearance) -> Self {
        match appearance {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::Dark,
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::Light,
        }
    }
}
