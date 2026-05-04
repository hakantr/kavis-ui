use gpui::{
    App, AppContext, Axis, Context, Element, Entity, FocusHandle, Focusable, Global, IntoElement,
    ParentElement as _, Render, SharedString, Styled, Window, px,
};

use kavis_ui::{
    BilesenBoyutu, Boyutlandirilabilir, EtkinTema, Simge, SimgeAdi, Tema, TemaModu,
    button::Dugme,
    group_box::GrupKutusuVaryanti,
    h_flex,
    label::Etiket,
    setting::{
        AyarAlani, AyarAlaniOgesi, AyarGrubu, AyarOgesi, AyarSayfasi, Ayarlar, NumberFieldOptions,
        RenderOptions,
    },
    text::markdown,
    v_flex,
};

struct AppSettings {
    agent_sidebar_side: SharedString,
    auto_switch_theme: bool,
    bookmarks_enabled: bool,
    branch_diff_context: bool,
    cli_path: SharedString,
    edit_prediction_provider: SharedString,
    external_agent_history: bool,
    font_family: SharedString,
    font_size: f64,
    git_select_all: bool,
    line_height: f64,
    notifications_enabled: bool,
    auto_update: bool,
    parallel_agents: bool,
    resettable: bool,
    restore_agent_threads: bool,
    split_diff_default: bool,
    thinking_effort: SharedString,
    tool_permission_mode: SharedString,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            agent_sidebar_side: "Sol".into(),
            auto_switch_theme: false,
            bookmarks_enabled: true,
            branch_diff_context: true,
            cli_path: "/usr/local/bin/bash".into(),
            edit_prediction_provider: "Zeta 2".into(),
            external_agent_history: true,
            font_family: "Arial".into(),
            font_size: 14.0,
            git_select_all: true,
            line_height: 12.0,
            notifications_enabled: true,
            auto_update: true,
            parallel_agents: true,
            resettable: true,
            restore_agent_threads: true,
            split_diff_default: true,
            thinking_effort: "Yüksek".into(),
            tool_permission_mode: "Yazmalarda sor".into(),
        }
    }
}

impl Global for AppSettings {}

impl AppSettings {
    fn global(cx: &App) -> &AppSettings {
        cx.global::<AppSettings>()
    }

    pub fn global_mut(cx: &mut App) -> &mut AppSettings {
        cx.global_mut::<AppSettings>()
    }
}

pub struct SettingsStory {
    focus_handle: FocusHandle,
    group_variant: GrupKutusuVaryanti,
    size: BilesenBoyutu,
}

struct OpenURLSettingField {
    label: SharedString,
    url: SharedString,
}

impl OpenURLSettingField {
    fn new(label: impl Into<SharedString>, url: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            url: url.into(),
        }
    }
}

impl AyarAlaniOgesi for OpenURLSettingField {
    type Element = Dugme;
    fn render_field(&self, options: &RenderOptions, _: &mut Window, _: &mut App) -> Self::Element {
        let url = self.url.clone();
        Dugme::new("open-url")
            .outline()
            .label(self.label.clone())
            .with_size(options.size)
            .on_click(move |_, _window, cx| {
                cx.open_url(url.as_str());
            })
    }
}

impl super::Story for SettingsStory {
    fn title() -> &'static str {
        "Ayarlar"
    }

    fn description() -> &'static str {
        "Uygulama için ayar grupları ve ayar öğeleri koleksiyonu."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn paddings() -> gpui::Pixels {
        px(0.)
    }
}

impl SettingsStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.set_global::<AppSettings>(AppSettings::default());

        Self {
            focus_handle: cx.focus_handle(),
            group_variant: GrupKutusuVaryanti::Outline,
            size: BilesenBoyutu::default(),
        }
    }

    fn setting_pages(&self, _: &mut Window, cx: &mut Context<Self>) -> Vec<AyarSayfasi> {
        let view = cx.entity();
        let default_settings = AppSettings::default();
        let resettable = AppSettings::global(cx).resettable;

        vec![
            AyarSayfasi::new("Genel")
                .resettable(resettable)
                .default_open(true)
                .icon(Simge::new(SimgeAdi::Settings2))
                .groups(vec![
                    AyarGrubu::new().title("Görünüm").items(vec![
                        AyarOgesi::new(
                            "Koyu Mod",
                            AyarAlani::switch(
                                |cx: &App| cx.theme().mode.is_dark(),
                                |val: bool, cx: &mut App| {
                                    let mode = if val {
                                        TemaModu::Dark
                                    } else {
                                        TemaModu::Light
                                    };
                                    Tema::global_mut(cx).mode = mode;
                                    Tema::change(mode, None, cx);
                                },
                            )
                            .default_value(false),
                        )
                        .description("Açık ve koyu temalar arasında geçiş yapar."),
                        AyarOgesi::new(
                            "Temayı Otomatik Değiştir",
                            AyarAlani::checkbox(
                                |cx: &App| AppSettings::global(cx).auto_switch_theme,
                                |val: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).auto_switch_theme = val;
                                },
                            )
                            .default_value(default_settings.auto_switch_theme),
                        )
                        .description("Sistem ayarlarına göre temayı otomatik değiştirir."),
                        AyarOgesi::new(
                            "Sıfırlanabilir",
                            AyarAlani::switch(
                                |cx: &App| AppSettings::global(cx).resettable,
                                |checked: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).resettable = checked
                                },
                            ),
                        )
                        .description("Ayarlar için sıfırlama düğmesini etkinleştirir veya devre dışı bırakır."),
                        AyarOgesi::new(
                            "Grup Varyantı",
                            AyarAlani::dropdown(
                                vec![
                                    (GrupKutusuVaryanti::Normal.as_str().into(), "Normal".into()),
                                    (GrupKutusuVaryanti::Outline.as_str().into(), "Çerçeveli".into()),
                                    (GrupKutusuVaryanti::Fill.as_str().into(), "Dolgulu".into()),
                                ],
                                {
                                    let view = view.clone();
                                    move |cx: &App| {
                                        SharedString::from(
                                            view.read(cx).group_variant.as_str().to_string(),
                                        )
                                    }
                                },
                                {
                                    let view = view.clone();
                                    move |val: SharedString, cx: &mut App| {
                                        view.update(cx, |view, cx| {
                                            view.group_variant =
                                                GrupKutusuVaryanti::from_str(val.as_str());
                                            cx.notify();
                                        });
                                    }
                                },
                            )
                            .default_value(GrupKutusuVaryanti::Outline.as_str().to_string()),
                        )
                        .description("Ayar grupları için varyant seçer."),
                        AyarOgesi::new(
                            "Grup Boyutu",
                            AyarAlani::dropdown(
                                vec![
                                    (BilesenBoyutu::Orta.as_str().into(), "Orta".into()),
                                    (BilesenBoyutu::Kucuk.as_str().into(), "Küçük".into()),
                                    (BilesenBoyutu::CokKucuk.as_str().into(), "Çok Küçük".into()),
                                ],
                                {
                                    let view = view.clone();
                                    move |cx: &App| {
                                        SharedString::from(view.read(cx).size.as_str().to_string())
                                    }
                                },
                                {
                                    let view = view.clone();
                                    move |val: SharedString, cx: &mut App| {
                                        view.update(cx, |view, cx| {
                                            view.size = BilesenBoyutu::from_str(val.as_str());
                                            cx.notify();
                                        });
                                    }
                                },
                            )
                            .default_value(BilesenBoyutu::default().as_str().to_string()),
                        )
                        .description("Ayar grubu için boyut seçer."),
                    ]),
                    AyarGrubu::new()
                        .title("Yazı Tipi")
                        .item(
                            AyarOgesi::new(
                                "Yazı Tipi Ailesi",
                                AyarAlani::dropdown(
                                    vec![
                                        ("Arial".into(), "Arial".into()),
                                        ("Helvetica".into(), "Helvetica".into()),
                                        ("Times New Roman".into(), "Times New Roman".into()),
                                        ("Courier New".into(), "Courier New".into()),
                                    ],
                                    |cx: &App| AppSettings::global(cx).font_family.clone(),
                                    |val: SharedString, cx: &mut App| {
                                        AppSettings::global_mut(cx).font_family = val;
                                    },
                                )
                                .default_value(default_settings.font_family),
                            )
                            .description("Story için yazı tipi ailesini seçer."),
                        )
                        .item(
                            AyarOgesi::new(
                                "Yazı Boyutu",
                                AyarAlani::number_input(
                                    NumberFieldOptions {
                                        min: 8.0,
                                        max: 72.0,
                                        ..Default::default()
                                    },
                                    |cx: &App| AppSettings::global(cx).font_size,
                                    |val: f64, cx: &mut App| {
                                        AppSettings::global_mut(cx).font_size = val;
                                    },
                                )
                                .default_value(default_settings.font_size),
                            )
                            .description(
                                "Daha iyi okunabilirlik için yazı boyutunu 8 ile 72 arasında ayarlar.",
                            ),
                        )
                        .item(
                            AyarOgesi::new(
                                "Satır Yüksekliği",
                                AyarAlani::number_input(
                                    NumberFieldOptions {
                                        min: 8.0,
                                        max: 32.0,
                                        ..Default::default()
                                    },
                                    |cx: &App| AppSettings::global(cx).line_height,
                                    |val: f64, cx: &mut App| {
                                        AppSettings::global_mut(cx).line_height = val;
                                    },
                                )
                                .default_value(default_settings.line_height),
                            )
                            .description(
                                "Daha iyi okunabilirlik için satır yüksekliğini 8 ile 32 arasında ayarlar.",
                            ),
                        ),
                    AyarGrubu::new().title("Diğer").items(vec![
                        AyarOgesi::render(|options, _, _| {
                            h_flex()
                                .w_full()
                                .justify_between()
                                .flex_wrap()
                                .gap_3()
                                .child("Bu öğe AyarOgesi::element ile oluşturulan özel bir elemandır.")
                                .child(
                                    Dugme::new("action")
                                        .icon(SimgeAdi::Globe)
                                        .label("Depo...")
                                        .outline()
                                        .with_size(options.size)
                                        .on_click(|_, _, cx| {
                                            cx.open_url(
                                                "https://github.com/hakantr/kavis-ui",
                                            );
                                        }),
                                )
                                .into_any_element()
                        }),
                        AyarOgesi::new(
                            "CLI Yolu",
                            AyarAlani::input(
                                |cx: &App| AppSettings::global(cx).cli_path.clone(),
                                |val: SharedString, cx: &mut App| {
                                    println!("cli-path değeri ayarlandı: {}", val);
                                    AppSettings::global_mut(cx).cli_path = val;
                                },
                            )
                            .default_value(default_settings.cli_path),
                        )
                        .layout(Axis::Vertical)
                        .description(
                            "CLI çalıştırılabilir dosyasının yolu. \n\
                        Bu öğe dikey yerleşim kullanır. Başlık,\
                        açıklama ve alan %100 genişlikte dikey hizalanır.",
                        ),
                    ]),
                ]),
            AyarSayfasi::new("Ajan")
                .resettable(resettable)
                .icon(Simge::new(SimgeAdi::Bot))
                .groups(vec![
                    AyarGrubu::new().title("Oturumlar").items(vec![
                        AyarOgesi::new(
                            "Paralel Ajan Oturumları",
                            AyarAlani::switch(
                                |cx: &App| AppSettings::global(cx).parallel_agents,
                                |val: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).parallel_agents = val;
                                },
                            )
                            .default_value(default_settings.parallel_agents),
                        )
                        .description("Aynı çalışma alanında birden çok ajan oturumu çalıştırır."),
                        AyarOgesi::new(
                            "Ajan Oturumlarını Geri Yükle",
                            AyarAlani::switch(
                                |cx: &App| AppSettings::global(cx).restore_agent_threads,
                                |val: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).restore_agent_threads = val;
                                },
                            )
                            .default_value(default_settings.restore_agent_threads),
                        )
                        .description("Uygulama yeniden başlatıldıktan sonra ajan oturumlarını kullanılabilir tutar."),
                        AyarOgesi::new(
                            "Oturum Kenar Çubuğu Tarafı",
                            AyarAlani::dropdown(
                                vec![
                                    ("Sol".into(), "Sol".into()),
                                    ("Sağ".into(), "Sağ".into()),
                                ],
                                |cx: &App| AppSettings::global(cx).agent_sidebar_side.clone(),
                                |val: SharedString, cx: &mut App| {
                                    AppSettings::global_mut(cx).agent_sidebar_side = val;
                                },
                            )
                            .default_value(default_settings.agent_sidebar_side),
                        )
                        .description(
                            "Oturum kenar çubuğunun pencerenin hangi tarafında görüneceğini seçer.",
                        ),
                    ]),
                    AyarGrubu::new().title("Modeller ve Bağlam").items(vec![
                        AyarOgesi::new(
                            "Düzenleme Tahmini Sağlayıcısı",
                            AyarAlani::dropdown(
                                vec![
                                    ("Zeta 2".into(), "Zeta 2".into()),
                                    ("GitHub Copilot NES".into(), "GitHub Copilot NES".into()),
                                    ("Ollama".into(), "Ollama".into()),
                                    ("Codestral".into(), "Codestral".into()),
                                ],
                                |cx: &App| AppSettings::global(cx).edit_prediction_provider.clone(),
                                |val: SharedString, cx: &mut App| {
                                    AppSettings::global_mut(cx).edit_prediction_provider = val;
                                },
                            )
                            .default_value(default_settings.edit_prediction_provider),
                        )
                        .description("Sonraki düzenleme tahminleri için kullanılan sağlayıcıyı seçer."),
                        AyarOgesi::new(
                            "Düşünme Çabası",
                            AyarAlani::dropdown(
                                vec![
                                    ("Düşük".into(), "Düşük".into()),
                                    ("Orta".into(), "Orta".into()),
                                    ("Yüksek".into(), "Yüksek".into()),
                                ],
                                |cx: &App| AppSettings::global(cx).thinking_effort.clone(),
                                |val: SharedString, cx: &mut App| {
                                    AppSettings::global_mut(cx).thinking_effort = val;
                                },
                            )
                            .default_value(default_settings.thinking_effort),
                        )
                        .description("Desteklenen dil modelleri için düşünme çabasını kontrol eder."),
                        AyarOgesi::new(
                            "Dal Farkı Bağlamı",
                            AyarAlani::switch(
                                |cx: &App| AppSettings::global(cx).branch_diff_context,
                                |val: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).branch_diff_context = val;
                                },
                            )
                            .default_value(default_settings.branch_diff_context),
                        )
                        .description("Dal farklarının ajan istemine eklenmesine izin verir."),
                    ]),
                    AyarGrubu::new().title("Araç İzinleri").items(vec![
                        AyarOgesi::new(
                            "İzin Hazır Ayarı",
                            AyarAlani::dropdown(
                                vec![
                                    ("Yazmalarda sor".into(), "Yazmalarda sor".into()),
                                    ("Salt okumayı otomatik onayla".into(), "Salt okumayı otomatik onayla".into()),
                                    ("Sıkı onay".into(), "Sıkı onay".into()),
                                ],
                                |cx: &App| AppSettings::global(cx).tool_permission_mode.clone(),
                                |val: SharedString, cx: &mut App| {
                                    AppSettings::global_mut(cx).tool_permission_mode = val;
                                },
                            )
                            .default_value(default_settings.tool_permission_mode),
                        )
                        .description("Ajan araç çağrıları için varsayılan davranışı belirler."),
                        AyarOgesi::new(
                            "Harici Ajan Oturum Geçmişi",
                            AyarAlani::checkbox(
                                |cx: &App| AppSettings::global(cx).external_agent_history,
                                |val: bool, cx: &mut App| {
                                    AppSettings::global_mut(cx).external_agent_history = val;
                                },
                            )
                            .default_value(default_settings.external_agent_history),
                        )
                        .description(
                            "ACP uyumlu harici ajanlardan sürdürülebilir oturumları gösterir.",
                        ),
                    ]),
                ]),
            AyarSayfasi::new("Editör ve Git")
                .resettable(resettable)
                .icon(Simge::new(SimgeAdi::Github))
                .groups(vec![AyarGrubu::new().title("İş Akışı").items(vec![
                    AyarOgesi::new(
                        "Yer İmleri",
                        AyarAlani::switch(
                            |cx: &App| AppSettings::global(cx).bookmarks_enabled,
                            |val: bool, cx: &mut App| {
                                AppSettings::global_mut(cx).bookmarks_enabled = val;
                            },
                        )
                        .default_value(default_settings.bookmarks_enabled),
                    )
                    .description("Editör kenarında kalıcı yer imlerini gösterir."),
                    AyarOgesi::new(
                        "Varsayılan Bölünmüş Diff",
                        AyarAlani::switch(
                            |cx: &App| AppSettings::global(cx).split_diff_default,
                            |val: bool, cx: &mut App| {
                                AppSettings::global_mut(cx).split_diff_default = val;
                            },
                        )
                        .default_value(default_settings.split_diff_default),
                    )
                    .description("Dal ve proje farklarını yan yana yerleşimde açar."),
                    AyarOgesi::new(
                        "Git Panelinde Tümünü Seç",
                        AyarAlani::checkbox(
                            |cx: &App| AppSettings::global(cx).git_select_all,
                            |val: bool, cx: &mut App| {
                                AppSettings::global_mut(cx).git_select_all = val;
                            },
                        )
                        .default_value(default_settings.git_select_all),
                    )
                    .description("Git panelinde tümünü seç ve seçimi kaldır kontrollerini gösterir."),
                ])]),
            AyarSayfasi::new("Yazılım Güncellemesi")
                .resettable(resettable)
                .icon(Simge::new(SimgeAdi::Cpu))
                .groups(vec![AyarGrubu::new().title("Güncellemeler").items(vec![
                    AyarOgesi::new(
                        "Bildirimleri Etkinleştir",
                        AyarAlani::switch(
                            |cx: &App| AppSettings::global(cx).notifications_enabled,
                            |val: bool, cx: &mut App| {
                                AppSettings::global_mut(cx).notifications_enabled = val;
                            },
                        )
                        .default_value(default_settings.notifications_enabled),
                    )
                    .description("Güncellemeler ve haberler hakkında bildirim alır."),
                    AyarOgesi::new(
                        "Otomatik Güncelle",
                        AyarAlani::switch(
                            |cx: &App| AppSettings::global(cx).auto_update,
                            |val: bool, cx: &mut App| {
                                AppSettings::global_mut(cx).auto_update = val;
                            },
                        )
                        .default_value(default_settings.auto_update),
                    )
                    .description("Güncellemeleri otomatik indirir ve kurar."),
                ])]),
            AyarSayfasi::new("Hakkında")
                .resettable(resettable)
                .icon(Simge::new(SimgeAdi::Info))
                .group(
                    AyarGrubu::new().item(AyarOgesi::render(|_options, _, cx| {
                        v_flex()
                            .gap_3()
                            .w_full()
                            .items_center()
                            .justify_center()
                            .child(Simge::new(SimgeAdi::GalleryVerticalEnd).size_16())
                            .child("Kavis UI")
                            .child(
                                Etiket::new(
                                    "GPUI ile harika çapraz platform masaüstü uygulamaları \
                                    oluşturmak için Rust GUI bileşenleri.",
                                )
                                .text_sm()
                                .text_color(cx.theme().muted_foreground),
                            )
                            .into_any()
                    })),
                )
                .group(AyarGrubu::new().title("Bağlantılar").items(vec![
                        AyarOgesi::new(
                            "GitHub Deposu",
                            AyarAlani::element(OpenURLSettingField::new(
                                "Depo...",
                                "https://github.com/hakantr/kavis-ui",
                            )),
                        )
                        .description("GitHub deposunu varsayılan tarayıcınızda açar."),
                        AyarOgesi::new(
                            "Documentation",
                            AyarAlani::element(OpenURLSettingField::new(
                                "Rust Dokümantasyonu...",
                                "https://hakantr.github.io/kavis-ui"
                            )),
                        )
                        .description(markdown(
                            "`kavis-ui` crate'i için Rust dokümantasyonu.",
                        )),
                        AyarOgesi::new(
                            "Web Sitesi",
                            AyarAlani::render(|options, _window, _cx| {
                                Dugme::new("open-url")
                                    .outline()
                                    .label("Web Sitesi...")
                                    .with_size(options.size)
                                    .on_click(|_, _window, cx| {
                                        cx.open_url("https://hakantr.github.io/kavis-ui/");
                                    })
                            }),
                        )
                        .description("Kavis UI için resmi web sitesi ve dokümantasyon."),
                    ])),
        ]
    }
}

impl Focusable for SettingsStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SettingsStory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Ayarlar::new("app-settings")
            .with_size(self.size)
            .with_group_variant(self.group_variant)
            .pages(self.setting_pages(window, cx))
    }
}
