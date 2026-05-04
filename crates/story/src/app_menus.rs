use gpui::{App, Entity, Menu, MenuItem, SharedString};
use kavis_ui::{EtkinTema as _, KureselDurum, Tema, TemaKaydi, TemaModu, menu::AppMenuBar};

use crate::{
    About, Open, Quit, SelectLocale, ToggleSearch,
    themes::{SwitchTheme, SwitchThemeMode},
};

pub fn init(title: impl Into<SharedString>, cx: &mut App) -> Entity<AppMenuBar> {
    let app_menu_bar = AppMenuBar::new(cx);
    let title: SharedString = title.into();
    update_app_menu(title.clone(), app_menu_bar.clone(), cx);

    cx.on_action({
        let title = title.clone();
        let app_menu_bar = app_menu_bar.clone();
        move |s: &SelectLocale, cx: &mut App| {
            rust_i18n::set_locale(&s.0.as_str());
            update_app_menu(title.clone(), app_menu_bar.clone(), cx);
        }
    });

    // Observe theme changes to update the menu to refresh the checked state
    cx.observe_global::<Tema>({
        let title = title.clone();
        let app_menu_bar = app_menu_bar.clone();
        move |cx| {
            update_app_menu(title.clone(), app_menu_bar.clone(), cx);
        }
    })
    .detach();

    app_menu_bar
}

fn update_app_menu(title: impl Into<SharedString>, app_menu_bar: Entity<AppMenuBar>, cx: &mut App) {
    let title: SharedString = title.into();

    cx.set_menus(build_menus(title.clone(), cx));
    let menus = build_menus(title, cx)
        .into_iter()
        .map(|menu| menu.owned())
        .collect();
    KureselDurum::global_mut(cx).set_app_menus(menus);

    app_menu_bar.update(cx, |menu_bar, cx| {
        menu_bar.reload(cx);
    })
}

fn build_menus(title: impl Into<SharedString>, cx: &App) -> Vec<Menu> {
    vec![
        Menu {
            name: title.into(),
            items: vec![
                MenuItem::action("Hakkında", About),
                MenuItem::Separator,
                MenuItem::action("Aç...", Open),
                MenuItem::Separator,
                MenuItem::Submenu(Menu {
                    name: "Görünüm".into(),
                    items: vec![
                        MenuItem::action("Açık", SwitchThemeMode(TemaModu::Light))
                            .checked(!cx.theme().mode.is_dark()),
                        MenuItem::action("Koyu", SwitchThemeMode(TemaModu::Dark))
                            .checked(cx.theme().mode.is_dark()),
                    ],
                    disabled: false,
                }),
                theme_menu(cx),
                language_menu(cx),
                MenuItem::Separator,
                MenuItem::action("Çık", Quit),
            ],
            disabled: false,
        },
        Menu {
            name: "Düzen".into(),
            items: vec![
                MenuItem::action("Geri Al", kavis_ui::input::Undo),
                MenuItem::action("Yinele", kavis_ui::input::Redo),
                MenuItem::separator(),
                MenuItem::action("Kes", kavis_ui::input::Cut),
                MenuItem::action("Kopyala", kavis_ui::input::Copy),
                MenuItem::action("Yapıştır", kavis_ui::input::Paste),
                MenuItem::separator(),
                MenuItem::action("Sil", kavis_ui::input::Delete),
                MenuItem::action(
                    "Önceki Sözcüğü Sil",
                    kavis_ui::input::DeleteToPreviousWordStart,
                ),
                MenuItem::action("Sonraki Sözcüğü Sil", kavis_ui::input::DeleteToNextWordEnd),
                MenuItem::separator(),
                MenuItem::action("Bul", kavis_ui::input::Search),
                MenuItem::separator(),
                MenuItem::action("Tümünü Seç", kavis_ui::input::SelectAll),
            ],
            disabled: false,
        },
        Menu {
            name: "Pencere".into(),
            items: vec![MenuItem::action("Aramayı Aç/Kapat", ToggleSearch)],
            disabled: false,
        },
        Menu {
            name: "Yardım".into(),
            items: vec![
                MenuItem::action("Dokümantasyon", Open).disabled(true),
                MenuItem::separator(),
                MenuItem::action("Web Sitesini Aç", Open),
            ],
            disabled: false,
        },
    ]
}

fn language_menu(_: &App) -> MenuItem {
    let locale = rust_i18n::locale().to_string();
    MenuItem::Submenu(Menu {
        name: "Dil".into(),
        items: vec![MenuItem::action("English", SelectLocale("en".into())).checked(locale == "en")],
        disabled: false,
    })
}

fn theme_menu(cx: &App) -> MenuItem {
    let themes = TemaKaydi::global(cx).sorted_themes();
    let current_name = cx.theme().theme_name();
    MenuItem::Submenu(Menu {
        name: "Tema".into(),
        items: themes
            .iter()
            .map(|theme| {
                let checked = current_name == &theme.name;
                MenuItem::action(theme.name.clone(), SwitchTheme(theme.name.clone()))
                    .checked(checked)
            })
            .collect(),
        disabled: false,
    })
}
