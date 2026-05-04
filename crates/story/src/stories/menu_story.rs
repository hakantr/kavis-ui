use kavis_ui::ham_gpui::{
    Anchor, App, AppContext, Context, Entity, InteractiveElement, IntoElement, KeyBinding,
    ParentElement as _, Render, SharedString, Styled as _, Window, div, px,
};
use kavis_ui::{
    EtkinTema as _, Side, SimgeAdi, StilUzantisi,
    button::Dugme,
    h_flex,
    menu::{AcilirMenuOgesi, AcilirMenuTetikleyici as _, BaglamMenusuUzantisi},
    v_flex,
};
use serde::Deserialize;

use crate::section;

#[derive(kavis_ui::Aksiyon, Clone, PartialEq, Deserialize)]
#[aksiyon(namespace = menu_story, no_json)]
struct Info(usize);

kavis_ui::aksiyonlar!(menu_story, [Copy, Paste, Cut, SearchAll, ToggleCheck]);

const CONTEXT: &str = "menu_story";
pub fn init(cx: &mut App) {
    cx.bind_keys([
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-c", Copy, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", Paste, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-x", Cut, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-f", SearchAll, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-shift-f", SearchAll, Some(CONTEXT)),
        KeyBinding::new("ctrl-shift-alt-t", ToggleCheck, Some(CONTEXT)),
    ])
}

pub struct MenuStory {
    check_side: Option<Side>,
    message: String,
}

impl super::Story for MenuStory {
    fn title() -> &'static str {
        "Menu"
    }

    fn description() -> &'static str {
        "Popup menu and context menu"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl MenuStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            check_side: None,
            message: "".to_string(),
        }
    }

    fn on_copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        self.message = "Kopyala seçeneğine tıkladınız".to_string();
        cx.notify()
    }

    fn on_cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        self.message = "Kes seçeneğine tıkladınız".to_string();
        cx.notify()
    }

    fn on_paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        self.message = "Yapıştır seçeneğine tıkladınız".to_string();
        cx.notify()
    }

    fn on_search_all(&mut self, _: &SearchAll, _: &mut Window, cx: &mut Context<Self>) {
        self.message = "Tümünde ara seçeneğine tıkladınız".to_string();
        cx.notify()
    }

    fn on_action_info(&mut self, info: &Info, _: &mut Window, cx: &mut Context<Self>) {
        self.message = format!("Bilgi seçeneğine tıkladınız: {}", info.0);
        cx.notify()
    }

    fn on_action_toggle_check(&mut self, _: &ToggleCheck, _: &mut Window, cx: &mut Context<Self>) {
        self.check_side = if self.check_side == Some(Side::Left) {
            Some(Side::Right)
        } else if self.check_side == Some(Side::Right) {
            None
        } else {
            Some(Side::Left)
        };

        self.message = format!("Onay işareti tarafı kullanıldı: {:?}", self.check_side);
        cx.notify()
    }
}

impl Render for MenuStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let check_side = self.check_side;
        let view = cx.entity();

        v_flex()
            .key_context(CONTEXT)
            .on_action(cx.listener(Self::on_copy))
            .on_action(cx.listener(Self::on_cut))
            .on_action(cx.listener(Self::on_paste))
            .on_action(cx.listener(Self::on_search_all))
            .on_action(cx.listener(Self::on_action_info))
            .on_action(cx.listener(Self::on_action_toggle_check))
            .size_full()
            .min_h(px(400.))
            .gap_6()
            .child(
                section("Popup Menu")
                    .child(
                        Dugme::new("popup-menu-1")
                            .outline()
                            .label("Düzenle")
                            .acilir_menu(move |this, window, cx| {
                                this.min_w(250.)
                                    .link("Hakkında", "https://github.com/hakantr/kavis-ui")
                                    .check_side(check_side.unwrap_or(Side::Left))
                                    .separator()
                                    .item(AcilirMenuOgesi::new("Tıklamayı Yakala").on_click(
                                        window.listener_for(&view, |this, _, _, cx| {
                                            this.message =
                                                "Tıklama yakalama öğesine tıkladınız".to_string();
                                            cx.notify();
                                        }),
                                    ))
                                    .separator()
                                    .menu("Kopyala", Box::new(Copy))
                                    .menu("Kes", Box::new(Cut))
                                    .menu("Yapıştır", Box::new(Paste))
                                    .separator()
                                    .menu_with_check(
                                        format!("Onay İşareti Tarafı {:?}", check_side),
                                        check_side.is_some(),
                                        Box::new(ToggleCheck),
                                    )
                                    .separator()
                                    .menu_with_icon("Ara", SimgeAdi::Search, Box::new(SearchAll))
                                    .separator()
                                    .item(
                                        AcilirMenuOgesi::element(|_, cx| {
                                            v_flex().child("Özel Eleman").child(
                                                div()
                                                    .text_xs()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child("Bu alt başlıktır"),
                                            )
                                        })
                                        .on_click(
                                            window.listener_for(&view, |this, _, _, cx| {
                                                this.message =
                                                    "Özel elemana tıkladınız".to_string();
                                                cx.notify();
                                            }),
                                        ),
                                    )
                                    .menu_element_with_check(
                                        check_side.is_some(),
                                        Box::new(ToggleCheck),
                                        |_, cx| {
                                            h_flex().gap_1().child("Özel Eleman").child(
                                                div()
                                                    .text_xs()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child("seçili"),
                                            )
                                        },
                                    )
                                    .menu_element_with_icon(
                                        SimgeAdi::Info,
                                        Box::new(Info(0)),
                                        |_, cx| {
                                            h_flex().gap_1().child("Özel").child(
                                                div()
                                                    .text_sm()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child("eleman"),
                                            )
                                        },
                                    )
                                    .separator()
                                    .menu_with_disabled("Devre Dışı Öğe", Box::new(Info(0)), true)
                                    .separator()
                                    .submenu("Bağlantılar", window, cx, |menu, _, _| {
                                        menu.link_with_icon(
                                            "Kavis UI",
                                            SimgeAdi::Github,
                                            "https://github.com/hakantr/kavis-ui",
                                        )
                                        .separator()
                                        .link("GPUI", "https://gpui.rs")
                                        .link("Zed", "https://zed.dev")
                                    })
                                    .separator()
                                    .submenu("Diğer Bağlantılar", window, cx, |menu, _, _| {
                                        menu.link("Crates", "https://crates.io")
                                            .link("Rust Docs", "https://docs.rs")
                                    })
                            }),
                    )
                    .child(self.message.clone()),
            )
            .child(
                section("Context Menu")
                    .v_flex()
                    .gap_4()
                    .child(
                        v_flex()
                            .w_full()
                            .p_4()
                            .items_center()
                            .justify_center()
                            .min_h_20()
                            .rounded(cx.theme().radius_lg)
                            .border_2()
                            .border_dashed()
                            .border_color(cx.theme().border)
                            .child("Bağlam menüsünü açmak için sağ tıklayın")
                            .baglam_menusu({
                                move |this, window, cx| {
                                    this.check_side(check_side.unwrap_or(Side::Left))
                                        .external_link_icon(false)
                                        .link("Hakkında", "https://github.com/hakantr/kavis-ui")
                                        .separator()
                                        .menu("Kes", Box::new(Cut))
                                        .menu("Kopyala", Box::new(Copy))
                                        .menu("Yapıştır", Box::new(Paste))
                                        .separator()
                                        .label("Bu bir etikettir")
                                        .menu_with_check(
                                            format!("Onay İşareti Tarafı {:?}", check_side),
                                            check_side.is_some(),
                                            Box::new(ToggleCheck),
                                        )
                                        .separator()
                                        .submenu("Ayarlar", window, cx, move |menu, _, _| {
                                            menu.menu("Bilgi 0", Box::new(Info(0)))
                                                .separator()
                                                .menu("Öğe 1", Box::new(Info(1)))
                                                .menu("Öğe 2", Box::new(Info(2)))
                                        })
                                        .separator()
                                        .menu("Tümünde Ara", Box::new(SearchAll))
                                        .separator()
                                }
                            })
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child(
                                        "Bağlam menüsünü açmak için bu alanın \
                                         herhangi bir yerine sağ tıklayabilirsiniz.",
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .id("other")
                            .flex()
                            .w_full()
                            .p_4()
                            .items_center()
                            .justify_center()
                            .min_h_20()
                            .rounded(cx.theme().radius_lg)
                            .border_2()
                            .border_dashed()
                            .border_color(cx.theme().border)
                            .child("Burada bağlam menülü başka bir alan var.")
                            .baglam_menusu({
                                move |this, _, _| {
                                    this.link("Hakkında", "https://github.com/hakantr/kavis-ui")
                                        .separator()
                                        .menu("Öğe 1", Box::new(Info(1)))
                                }
                            }),
                    )
                    .child(
                        div()
                            .id("other1")
                            .flex()
                            .w_full()
                            .p_4()
                            .items_center()
                            .justify_center()
                            .min_h_20()
                            .rounded(cx.theme().radius_lg)
                            .border_2()
                            .border_dashed()
                            .border_color(cx.theme().border)
                            .child("Bağlam menüsü alanı 1")
                            .baglam_menusu({
                                move |this, _, _| {
                                    this.link("Hakkında", "https://github.com/hakantr/kavis-ui")
                                        .separator()
                                        .menu("Öğe 1", Box::new(Info(1)))
                                }
                            }),
                    ),
            )
            .child(
                section("Menu with scrollbar")
                    .child(
                        Dugme::new("dropdown-menu-scrollable-1")
                            .outline()
                            .label("Kaydırılabilir Menü (100 öğe)")
                            .acilir_menu_capa_ile(Anchor::TopRight, move |this, _, _| {
                                let mut this = this
                                    .scrollable(true)
                                    .max_h(px(300.))
                                    .label(format!("Toplam {} öğe", 100));
                                for i in 0..100 {
                                    if i % 5 == 0 {
                                        this = this.separator();
                                    }

                                    this = this.menu(
                                        SharedString::from(format!("Öğe {}", i)),
                                        Box::new(Info(i)),
                                    )
                                }
                                this.min_w(px(100.))
                            }),
                    )
                    .child(
                        Dugme::new("dropdown-menu-scrollable-2")
                            .outline()
                            .label("Kaydırılabilir Menü (5 öğe)")
                            .acilir_menu_capa_ile(Anchor::TopRight, move |this, _, _| {
                                let mut this = this
                                    .scrollable(true)
                                    .max_h(px(300.))
                                    .label(format!("Toplam {} öğe", 100));
                                for i in 0..5 {
                                    this = this.menu(
                                        SharedString::from(format!("Öğe {}", i)),
                                        Box::new(Info(i)),
                                    )
                                }
                                this.min_w(px(100.))
                            }),
                    ),
            )
    }
}
