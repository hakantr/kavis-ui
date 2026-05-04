use crate::ham_gpui::{App, Global, SharedString};
use crate::{Tema, TemaKumesi, TemaModu, TemaRengi, TemaYapilandirmasi, highlighter::VurguTemasi};
#[allow(unused)]
use anyhow::Result;
use std::{
    collections::HashMap,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, LazyLock},
};

const DEFAULT_THEME: &str = include_str!("./default-theme.json");
pub(crate) static DEFAULT_THEME_COLORS: LazyLock<
    HashMap<TemaModu, (Arc<TemaRengi>, Arc<VurguTemasi>)>,
> = LazyLock::new(|| {
    let mut colors = HashMap::new();

    let themes: Vec<TemaYapilandirmasi> = serde_json::from_str::<TemaKumesi>(DEFAULT_THEME)
        .expect("themes/default.json ayrıştırılamadı")
        .themes;

    for theme in themes {
        let mut theme_color = TemaRengi::default();
        theme_color.apply_config(&theme, &TemaRengi::default());

        let highlight_theme = VurguTemasi {
            name: theme.name.to_string(),
            appearance: theme.mode,
            style: theme.highlight.unwrap_or_default(),
        };

        colors.insert(
            theme.mode,
            (Arc::new(theme_color), Arc::new(highlight_theme)),
        );
    }

    colors
});

pub(super) fn init(cx: &mut App) {
    cx.set_global(TemaKaydi::default());
    TemaKaydi::global_mut(cx).init_default_themes();

    // Observe changes to the theme registry to apply changes to the active theme
    cx.observe_global::<TemaKaydi>(|cx| {
        let mode = Tema::global(cx).mode;
        let light_theme = Tema::global(cx).light_theme.name.clone();
        let dark_theme = Tema::global(cx).dark_theme.name.clone();

        if let Some(theme) = TemaKaydi::global(cx).themes().get(&light_theme).cloned() {
            Tema::global_mut(cx).light_theme = theme;
        }
        if let Some(theme) = TemaKaydi::global(cx).themes().get(&dark_theme).cloned() {
            Tema::global_mut(cx).dark_theme = theme;
        }

        let theme_name = if mode.is_dark() {
            dark_theme
        } else {
            light_theme
        };

        tracing::info!("Etkin tema yeniden yükleniyor: {:?}...", theme_name);
        Tema::change(mode, None, cx);
        cx.refresh_windows();
    })
    .detach();
}

#[derive(Default, Debug)]
pub struct TemaKaydi {
    themes_dir: PathBuf,
    default_themes: HashMap<TemaModu, Rc<TemaYapilandirmasi>>,
    themes: HashMap<SharedString, Rc<TemaYapilandirmasi>>,
    has_custom_themes: bool,
}

impl Global for TemaKaydi {}

impl TemaKaydi {
    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }

    /// Watch themes dizin.
    ///
    /// And reload themes için tetikleyici `on_load` geri çağrı.
    #[cfg(not(target_family = "wasm"))]
    pub fn watch_dir<F>(themes_dir: PathBuf, cx: &mut App, on_load: F) -> Result<()>
    where
        F: Fn(&mut App) + 'static,
    {
        Self::global_mut(cx).themes_dir = themes_dir.clone();

        // Load theme in the background.
        cx.spawn(async move |cx| {
            _ = cx.update(|cx| {
                if let Err(err) = Self::_watch_themes_dir(themes_dir, cx) {
                    tracing::error!("Temalar dizini izlenemedi: {}", err);
                }

                Self::reload_themes(cx);
                on_load(cx);
            });
        })
        .detach();

        Ok(())
    }

    /// bir referans için eşleme themes (dahil varsayılan themes). döndürür.
    pub fn themes(&self) -> &HashMap<SharedString, Rc<TemaYapilandirmasi>> {
        &self.themes
    }

    /// bir sıralı liste themes. döndürür.
    pub fn sorted_themes(&self) -> Vec<&Rc<TemaYapilandirmasi>> {
        let mut themes = self.themes.values().collect::<Vec<_>>();
        // sort by is_default true first, then light first dark later, then by name case-insensitive
        themes.sort_by(|a, b| {
            b.is_default
                .cmp(&a.is_default)
                .then(a.mode.cmp(&b.mode))
                .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        themes
    }

    /// bir referans için eşleme varsayılan themes. döndürür.
    pub fn default_themes(&self) -> &HashMap<TemaModu, Rc<TemaYapilandirmasi>> {
        &self.default_themes
    }

    pub fn default_light_theme(&self) -> &Rc<TemaYapilandirmasi> {
        &self.default_themes[&TemaModu::Light]
    }

    pub fn default_dark_theme(&self) -> &Rc<TemaYapilandirmasi> {
        &self.default_themes[&TemaModu::Dark]
    }

    pub fn load_themes_from_str(&mut self, content: &str) -> anyhow::Result<()> {
        let theme_set = serde_json::from_str::<TemaKumesi>(content)?;
        for theme in theme_set.themes {
            if !self.themes.contains_key(&theme.name) {
                let theme_name = theme.name.clone();
                self.themes.insert(theme_name, Rc::new(theme));
                self.has_custom_themes = true;
            }
        }
        Ok(())
    }

    fn init_default_themes(&mut self) {
        let default_themes: Vec<TemaYapilandirmasi> =
            serde_json::from_str::<TemaKumesi>(DEFAULT_THEME)
                .expect("varsayılan tema ayrıştırılamadı.")
                .themes;
        for theme in default_themes.into_iter() {
            if theme.mode.is_dark() {
                self.default_themes.insert(TemaModu::Dark, Rc::new(theme));
            } else {
                self.default_themes.insert(TemaModu::Light, Rc::new(theme));
            }
        }
        self.themes_dir = PathBuf::from("./themes");
        self.themes = self
            .default_themes
            .values()
            .map(|theme| {
                let name = theme.name.clone();
                (name, Rc::clone(theme))
            })
            .collect();
    }

    #[cfg(not(target_family = "wasm"))]
    fn _watch_themes_dir(themes_dir: PathBuf, cx: &mut App) -> anyhow::Result<()> {
        if !themes_dir.exists() {
            std::fs::create_dir_all(&themes_dir)?;
        }

        let (tx, rx) = smol::channel::bounded(100);
        let mut watcher =
            notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
                if let Ok(event) = &res {
                    match event.kind {
                        notify::EventKind::Create(_)
                        | notify::EventKind::Modify(_)
                        | notify::EventKind::Remove(_) => {
                            if let Err(err) = tx.send_blocking(res) {
                                tracing::error!("Tema olayı gönderilemedi: {:?}", err);
                            }
                        }
                        _ => {}
                    }
                }
            })?;

        cx.spawn(async move |cx| {
            use notify::Watcher as _;

            if let Err(err) = watcher.watch(&themes_dir, notify::RecursiveMode::Recursive) {
                tracing::error!("Temalar dizini izlenemedi: {:?}", err);
            }

            while (rx.recv().await).is_ok() {
                tracing::info!("Temalar yeniden yükleniyor...");
                _ = cx.update(Self::reload_themes);
            }
        })
        .detach();

        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    fn reload_themes(cx: &mut App) {
        let registry = Self::global_mut(cx);
        match registry.reload() {
            Ok(_) => {
                tracing::info!("Temalar başarıyla yeniden yüklendi.");
            }
            Err(e) => tracing::error!("Temalar yeniden yüklenemedi: {:?}", e),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    /// Temaları `themes_dir` dizininden yeniden yükler.
    fn reload(&mut self) -> Result<()> {
        let mut themes = vec![];

        if self.themes_dir.exists() {
            for entry in std::fs::read_dir(&self.themes_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    let file_content = std::fs::read_to_string(path.clone())?;

                    match serde_json::from_str::<TemaKumesi>(&file_content) {
                        Ok(theme_set) => {
                            themes.extend(theme_set.themes);
                        }
                        Err(e) => {
                            tracing::error!(
                                "geçersiz tema dosyası yoksayıldı: {}, {}",
                                path.display(),
                                e
                            );
                        }
                    }
                }
            }
        }

        self.themes.clear();
        for theme in self.default_themes.values() {
            self.themes
                .insert(theme.name.clone(), Rc::new((**theme).clone()));
        }

        for theme in themes.iter() {
            if self.themes.contains_key(&theme.name) {
                continue;
            }

            if theme.is_default {
                self.default_themes
                    .insert(theme.mode, Rc::new(theme.clone()));
            }

            self.has_custom_themes = true;
            self.themes
                .insert(theme.name.clone(), Rc::new(theme.clone()));
        }

        Ok(())
    }
}
