use gpui::{
    Action, App, AppContext, Entity, Menu, MenuItem, OwnedMenu, OwnedMenuItem, SharedString,
};

use crate::{
    global_state::KureselDurum,
    gpui_turkce::{IsletimSistemiAksiyonu, SistemMenusuTuru},
    menu::AppMenuBar,
};

/// Platform farklarini gizleyen uygulama menusu.
///
/// Bu modelden uretilen menuler tek cagriyla hem GPUI native menulerine
/// yazilir hem de Linux/Windows titlebar menu cubugunun kullanacagi
/// sahipli kopya olarak saklanir.
#[derive(Clone)]
pub struct UygulamaMenusu {
    baslik: SharedString,
    ogeler: Vec<UygulamaMenuOgesi>,
    devre_disi: bool,
}

impl UygulamaMenusu {
    pub fn yeni(baslik: impl Into<SharedString>) -> Self {
        Self {
            baslik: baslik.into(),
            ogeler: Vec::new(),
            devre_disi: false,
        }
    }

    pub fn baslik(&self) -> &SharedString {
        &self.baslik
    }

    pub fn ogeler(&self) -> &[UygulamaMenuOgesi] {
        &self.ogeler
    }

    pub fn oge(mut self, oge: UygulamaMenuOgesi) -> Self {
        self.ogeler.push(oge);
        self
    }

    pub fn ogelerle(mut self, ogeler: impl IntoIterator<Item = UygulamaMenuOgesi>) -> Self {
        self.ogeler.extend(ogeler);
        self
    }

    pub fn aksiyon(mut self, baslik: impl Into<SharedString>, aksiyon: impl Action) -> Self {
        self.ogeler
            .push(UygulamaMenuOgesi::aksiyon(baslik, aksiyon));
        self
    }

    pub fn kutulu_aksiyon(
        mut self,
        baslik: impl Into<SharedString>,
        aksiyon: Box<dyn Action>,
    ) -> Self {
        self.ogeler
            .push(UygulamaMenuOgesi::kutulu_aksiyon(baslik, aksiyon));
        self
    }

    pub fn ayirici(mut self) -> Self {
        self.ogeler.push(UygulamaMenuOgesi::ayirici());
        self
    }

    pub fn alt_menu(mut self, menu: UygulamaMenusu) -> Self {
        self.ogeler.push(UygulamaMenuOgesi::alt_menu(menu));
        self
    }

    pub fn devre_disi(mut self, devre_disi: bool) -> Self {
        self.devre_disi = devre_disi;
        self
    }

    pub fn gpui_menusu(&self) -> Menu {
        Menu {
            name: self.baslik.clone(),
            items: self
                .ogeler
                .iter()
                .map(UygulamaMenuOgesi::gpui_menu_ogesi)
                .collect(),
            disabled: self.devre_disi,
        }
    }

    pub fn sahipli_menu(&self) -> OwnedMenu {
        OwnedMenu {
            name: self.baslik.clone(),
            items: self
                .ogeler
                .iter()
                .map(UygulamaMenuOgesi::sahipli_menu_ogesi)
                .collect(),
            disabled: self.devre_disi,
        }
    }
}

/// [`UygulamaMenusu`] icindeki tek menu ogesi.
pub enum UygulamaMenuOgesi {
    Ayirici,
    AltMenu(UygulamaMenusu),
    SistemMenusu {
        baslik: SharedString,
        tur: SistemMenusuTuru,
    },
    Aksiyon {
        baslik: SharedString,
        aksiyon: Box<dyn Action>,
        isletim_sistemi_aksiyonu: Option<IsletimSistemiAksiyonu>,
        secili: bool,
        devre_disi: bool,
    },
}

impl Clone for UygulamaMenuOgesi {
    fn clone(&self) -> Self {
        match self {
            Self::Ayirici => Self::Ayirici,
            Self::AltMenu(menu) => Self::AltMenu(menu.clone()),
            Self::SistemMenusu { baslik, tur } => Self::SistemMenusu {
                baslik: baslik.clone(),
                tur: *tur,
            },
            Self::Aksiyon {
                baslik,
                aksiyon,
                isletim_sistemi_aksiyonu,
                secili,
                devre_disi,
            } => Self::Aksiyon {
                baslik: baslik.clone(),
                aksiyon: aksiyon.boxed_clone(),
                isletim_sistemi_aksiyonu: *isletim_sistemi_aksiyonu,
                secili: *secili,
                devre_disi: *devre_disi,
            },
        }
    }
}

impl UygulamaMenuOgesi {
    pub fn ayirici() -> Self {
        Self::Ayirici
    }

    pub fn alt_menu(menu: UygulamaMenusu) -> Self {
        Self::AltMenu(menu)
    }

    pub fn sistem_menusu(baslik: impl Into<SharedString>, tur: SistemMenusuTuru) -> Self {
        Self::SistemMenusu {
            baslik: baslik.into(),
            tur,
        }
    }

    pub fn aksiyon(baslik: impl Into<SharedString>, aksiyon: impl Action) -> Self {
        Self::Aksiyon {
            baslik: baslik.into(),
            aksiyon: Box::new(aksiyon),
            isletim_sistemi_aksiyonu: None,
            secili: false,
            devre_disi: false,
        }
    }

    pub fn kutulu_aksiyon(baslik: impl Into<SharedString>, aksiyon: Box<dyn Action>) -> Self {
        Self::Aksiyon {
            baslik: baslik.into(),
            aksiyon,
            isletim_sistemi_aksiyonu: None,
            secili: false,
            devre_disi: false,
        }
    }

    pub fn isletim_sistemi_aksiyonu(mut self, aksiyon: IsletimSistemiAksiyonu) -> Self {
        if let Self::Aksiyon {
            isletim_sistemi_aksiyonu,
            ..
        } = &mut self
        {
            *isletim_sistemi_aksiyonu = Some(aksiyon);
        }
        self
    }

    pub fn secili(mut self, secili: bool) -> Self {
        if let Self::Aksiyon { secili: eski, .. } = &mut self {
            *eski = secili;
        }
        self
    }

    pub fn devre_disi(mut self, devre_disi: bool) -> Self {
        match &mut self {
            Self::Aksiyon {
                devre_disi: eski, ..
            } => *eski = devre_disi,
            Self::AltMenu(menu) => menu.devre_disi = devre_disi,
            _ => {}
        }
        self
    }

    pub fn gpui_menu_ogesi(&self) -> MenuItem {
        match self {
            Self::Ayirici => MenuItem::Separator,
            Self::AltMenu(menu) => MenuItem::Submenu(menu.gpui_menusu()),
            Self::SistemMenusu { baslik, tur } => {
                MenuItem::os_submenu(baslik.clone(), (*tur).into())
            }
            Self::Aksiyon {
                baslik,
                aksiyon,
                isletim_sistemi_aksiyonu,
                secili,
                devre_disi,
            } => MenuItem::Action {
                name: baslik.clone(),
                action: aksiyon.boxed_clone(),
                os_action: isletim_sistemi_aksiyonu.map(Into::into),
                checked: *secili,
                disabled: *devre_disi,
            },
        }
    }

    pub fn sahipli_menu_ogesi(&self) -> OwnedMenuItem {
        match self {
            Self::Ayirici => OwnedMenuItem::Separator,
            Self::AltMenu(menu) => OwnedMenuItem::Submenu(menu.sahipli_menu()),
            Self::SistemMenusu { baslik, tur } => OwnedMenuItem::SystemMenu(gpui::OwnedOsMenu {
                name: baslik.clone(),
                menu_type: (*tur).into(),
            }),
            Self::Aksiyon {
                baslik,
                aksiyon,
                isletim_sistemi_aksiyonu,
                secili,
                devre_disi,
            } => OwnedMenuItem::Action {
                name: baslik.to_string(),
                action: aksiyon.boxed_clone(),
                os_action: isletim_sistemi_aksiyonu.map(Into::into),
                checked: *secili,
                disabled: *devre_disi,
            },
        }
    }
}

/// Menuleri OS farklarini gizleyerek kaydeder.
///
/// macOS'ta native menu bar, Linux/Windows'ta ise kavis-ui'in
/// [`AppMenuBar`] bileseni ayni modelden beslenir. Cagiran uygulama
/// yalnizca bu fonksiyona Turkce menu modelini verir.
pub fn uygulama_menulerini_kaydet(cx: &mut App, menuler: impl IntoIterator<Item = UygulamaMenusu>) {
    KureselDurum::ensure_global(cx);

    let menuler: Vec<UygulamaMenusu> = menuler.into_iter().collect();
    let gpui_menuleri = menuler
        .iter()
        .map(UygulamaMenusu::gpui_menusu)
        .collect::<Vec<_>>();
    let sahipli_menuler = menuler
        .iter()
        .map(UygulamaMenusu::sahipli_menu)
        .collect::<Vec<_>>();

    cx.set_menus(gpui_menuleri);
    KureselDurum::global_mut(cx).set_app_menus(sahipli_menuler);
}

/// Bu platformda uygulama icinde cizilen menu cubuguna ihtiyac olup olmadigini doner.
pub fn uygulama_menu_cubugu_gerekli_mi() -> bool {
    !cfg!(target_os = "macos")
}

/// Linux/Windows icin uygulama icinde cizilen menu cubugunu olusturur.
///
/// macOS'ta sistem menu cubugu kullanildigi icin `None` doner. Boylece
/// uygulama kodu `cfg(target_os = ...)` yazmak zorunda kalmaz.
pub fn uygulama_menu_cubugu_olustur(cx: &mut App) -> Option<Entity<AppMenuBar>> {
    uygulama_menu_cubugu_gerekli_mi().then(|| AppMenuBar::new(cx))
}

/// View/model baglami icinden uygulama menu cubugunu olusturur.
pub fn uygulama_menu_cubugu_olustur_baglam<Cx: AppContext>(
    cx: &mut Cx,
) -> Option<Entity<AppMenuBar>> {
    uygulama_menu_cubugu_gerekli_mi().then(|| AppMenuBar::new_in(cx))
}

/// Menu cubugunu platforma bakmadan olusturur.
///
/// Testlerde veya ozel titlebar tasarimlarinda macOS'ta da menu cubugu
/// cizmek istenirse bu fonksiyon kullanilabilir.
pub fn uygulama_menu_cubugu_olustur_zorla(cx: &mut App) -> Entity<AppMenuBar> {
    AppMenuBar::new(cx)
}

/// View/model baglami icinden, platforma bakmadan menu cubugu olusturur.
pub fn uygulama_menu_cubugu_olustur_zorla_baglam<Cx: AppContext>(
    cx: &mut Cx,
) -> Entity<AppMenuBar> {
    AppMenuBar::new_in(cx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::Cancel;

    #[test]
    fn turkce_menu_modeli_gpui_ve_sahipli_menu_uretir() {
        let menu = UygulamaMenusu::yeni("Dosya")
            .aksiyon("Vazgec", Cancel)
            .ayirici()
            .alt_menu(
                UygulamaMenusu::yeni("Duzen").oge(
                    UygulamaMenuOgesi::aksiyon("Kes", Cancel)
                        .isletim_sistemi_aksiyonu(IsletimSistemiAksiyonu::Kes),
                ),
            );

        let gpui_menu = menu.gpui_menusu();
        assert_eq!(gpui_menu.name.as_ref(), "Dosya");
        assert_eq!(gpui_menu.items.len(), 3);
        assert!(matches!(gpui_menu.items[1], MenuItem::Separator));

        let sahipli_menu = menu.sahipli_menu();
        assert_eq!(sahipli_menu.name.as_ref(), "Dosya");
        assert_eq!(sahipli_menu.items.len(), 3);
    }

    #[test]
    fn menu_kaydi_kuresel_duruma_da_yazilir() {
        let mut cx = gpui::TestAppContext::single();
        KureselDurum::ensure_global(&mut cx);

        uygulama_menulerini_kaydet(
            &mut cx,
            [UygulamaMenusu::yeni("Dosya").aksiyon("Vazgec", Cancel)],
        );

        let menuler = KureselDurum::global(&cx).app_menus();
        assert_eq!(menuler.len(), 1);
        assert_eq!(menuler[0].name.as_ref(), "Dosya");
    }
}
