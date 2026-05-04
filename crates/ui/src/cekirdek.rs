//! Kavis UI'nin GPUI uzerindeki cekirdek ara katmani.
//!
//! Bu modul uygulama tarafinin GPUI'yi dogrudan bagimlilik olarak bilmeden
//! uygulama dongusu, pencere acma, temel context ve platform tipleriyle
//! calisabilmesini saglar.

use std::rc::Rc;

use crate::ham_gpui::{
    App, AppContext as _, Application, AssetSource, Bounds, Context, Entity, Pixels, Platform,
    Render, SharedString, Size, TitlebarOptions, Window, WindowBackgroundAppearance, WindowBounds,
    WindowHandle, WindowOptions, px, size,
};

/// GPUI uygulama baglami.
pub type Uygulama = App;

/// GPUI uygulama kurulum nesnesi.
pub type UygulamaKurulumu = Application;

/// Bir view/model entity'sinin baglami.
pub type GorunumBaglami<'a, T> = Context<'a, T>;

/// GPUI pencere baglami.
pub type Pencere = Window;

/// Entity tutamacinin Turkce adi.
pub type Varlik<T> = Entity<T>;

/// Piksel olcusu.
pub type Piksel = Pixels;

/// Iki boyutlu olcu.
pub type Boyut<T = Pixels> = Size<T>;

/// Dikdortgen sinir.
pub type Sinir<T = Pixels> = Bounds<T>;

/// Paylasimli metin tipi.
pub type PaylasimliMetin = SharedString;

/// Pencere tutamaci.
pub type PencereTutamaci<T> = WindowHandle<T>;

/// Kavis UI sonuc tipi.
pub type KavisSonucu<T> = crate::ham_gpui::Result<T>;

/// Varlik kaynaklarini yukleyen soyutlama.
pub trait VarlikKaynagi: AssetSource {}

impl<T: AssetSource> VarlikKaynagi for T {}

/// Uygulama baslatma ve dongusunu yoneten sarmalayici.
pub struct KavisMotoru;

impl KavisMotoru {
    /// Varsayilan platformla varlik kaynagi olmadan uygulamayi baslatir.
    pub fn baslat(baslangic_fonksiyonu: impl FnOnce(&mut Uygulama) + 'static) {
        Self::platform_ile_baslat(gpui_platform::current_platform(false), baslangic_fonksiyonu);
    }

    /// Varsayilan platformla ve verilen varlik kaynagiyla uygulamayi baslatir.
    pub fn varliklarla_baslat(
        varlik_kaynagi: impl VarlikKaynagi,
        baslangic_fonksiyonu: impl FnOnce(&mut Uygulama) + 'static,
    ) {
        Self::platform_ile_kur(gpui_platform::current_platform(false))
            .with_assets(varlik_kaynagi)
            .run(move |cx| {
                crate::init(cx);
                baslangic_fonksiyonu(cx);
            });
    }

    /// Disaridan verilen platformla uygulamayi baslatir.
    pub fn platform_ile_baslat(
        platform: Rc<dyn Platform>,
        baslangic_fonksiyonu: impl FnOnce(&mut Uygulama) + 'static,
    ) {
        Self::platform_ile_kur(platform).run(move |cx| {
            crate::init(cx);
            baslangic_fonksiyonu(cx);
        });
    }

    /// Duzey dusuk kullanimlar icin platformdan uygulama kurulum nesnesi uretir.
    pub fn platform_ile_kur(platform: Rc<dyn Platform>) -> UygulamaKurulumu {
        Application::with_platform(platform)
    }
}

/// Basit ve Turkce pencere ayarlari.
#[derive(Clone, Debug)]
pub struct PencereAyarlari {
    pub baslik: PaylasimliMetin,
    pub genislik: f32,
    pub yukseklik: f32,
    pub minimum_boyut: Option<Boyut>,
    pub odakla: bool,
    pub goster: bool,
    pub saydam_baslik_cubugu: bool,
    pub arka_plan: WindowBackgroundAppearance,
}

impl Default for PencereAyarlari {
    fn default() -> Self {
        Self {
            baslik: "Kavis UI Uygulamasi".into(),
            genislik: 1024.0,
            yukseklik: 768.0,
            minimum_boyut: None,
            odakla: true,
            goster: true,
            saydam_baslik_cubugu: false,
            arka_plan: WindowBackgroundAppearance::default(),
        }
    }
}

impl PencereAyarlari {
    pub fn baslik(mut self, baslik: impl Into<PaylasimliMetin>) -> Self {
        self.baslik = baslik.into();
        self
    }

    pub fn boyut(mut self, genislik: f32, yukseklik: f32) -> Self {
        self.genislik = genislik;
        self.yukseklik = yukseklik;
        self
    }

    pub fn minimum_boyut(mut self, genislik: f32, yukseklik: f32) -> Self {
        self.minimum_boyut = Some(size(px(genislik), px(yukseklik)));
        self
    }

    pub fn saydam_baslik_cubugu(mut self, saydam: bool) -> Self {
        self.saydam_baslik_cubugu = saydam;
        self
    }

    pub fn arka_plan(mut self, arka_plan: WindowBackgroundAppearance) -> Self {
        self.arka_plan = arka_plan;
        self
    }

    /// Bu ayarlari GPUI'nin pencere seceneklerine cevirir.
    pub fn pencere_secenekleri(&self, cx: &Uygulama) -> WindowOptions {
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(self.genislik), px(self.yukseklik)),
                cx,
            ))),
            titlebar: Some(TitlebarOptions {
                title: Some(self.baslik.clone()),
                appears_transparent: self.saydam_baslik_cubugu,
                traffic_light_position: None,
            }),
            window_min_size: self.minimum_boyut,
            focus: self.odakla,
            show: self.goster,
            window_background: self.arka_plan,
            ..WindowOptions::default()
        }
    }
}

/// Hazir bir root entity donduren pencere olusturucuyla yeni pencere acar.
pub fn yeni_pencere_ac<V: Render + 'static>(
    cx: &mut Uygulama,
    ayarlar: PencereAyarlari,
    gorunum_olustur: impl FnOnce(&mut Pencere, &mut Uygulama) -> Varlik<V>,
) -> KavisSonucu<PencereTutamaci<V>> {
    let secenekler = ayarlar.pencere_secenekleri(cx);
    cx.open_window(secenekler, gorunum_olustur)
}

/// Sadece `Context<V>` ile uretilen bir view'i root olarak acan kolay yol.
pub fn yeni_gorunum_penceresi_ac<V: Render + 'static>(
    cx: &mut Uygulama,
    ayarlar: PencereAyarlari,
    gorunum_olustur: impl FnOnce(&mut GorunumBaglami<'_, V>) -> V + 'static,
) -> KavisSonucu<PencereTutamaci<V>> {
    yeni_pencere_ac(cx, ayarlar, move |_window, cx| cx.new(gorunum_olustur))
}
