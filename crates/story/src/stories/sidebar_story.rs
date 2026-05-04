use std::collections::HashMap;

use gpui::{
    Action, App, AppContext, ClickEvent, Context, Entity, Focusable, IntoElement, ParentElement,
    Render, SharedString, Styled, Window, div, prelude::FluentBuilder, px, relative,
};

use kavis_ui::{
    Boyutlandirilabilir, EtkinTema, Side, Simge, SimgeAdi,
    badge::Rozet,
    breadcrumb::{GezintiYolu, GezintiYoluOgesi},
    divider::Ayirici,
    h_flex,
    menu::AcilirMenuTetikleyici,
    sidebar::{
        YanCubuk, YanCubukAltligi, YanCubukBasligi, YanCubukGecisDugmesi, YanCubukGrubu,
        YanCubukMenuOgesi, YanCubukMenusu,
    },
    switch::Anahtar,
    v_flex,
};
use serde::Deserialize;

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = sidebar_story, no_json)]
pub struct SelectCompany(SharedString);

pub struct SidebarStory {
    active_items: HashMap<Item, bool>,
    last_active_item: Item,
    active_subitem: Option<SubItem>,
    collapsed: bool,
    side: Side,
    click_to_toggle_submenu: bool,
    show_dynamic_children: bool,
    focus_handle: gpui::FocusHandle,
    checked: bool,
}

impl SidebarStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut active_items = HashMap::new();
        active_items.insert(Item::Playground, true);

        Self {
            active_items,
            last_active_item: Item::Playground,
            active_subitem: None,
            collapsed: false,
            side: Side::Left,
            focus_handle: cx.focus_handle(),
            checked: false,
            click_to_toggle_submenu: false,
            show_dynamic_children: false,
        }
    }

    fn render_content(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().gap_3().child(
            h_flex()
                .gap_3()
                .child(
                    Anahtar::new("side")
                        .label("Kenar çubuğunu sağa al")
                        .checked(self.side.is_right())
                        .on_click(cx.listener(|this, checked: &bool, _, cx| {
                            this.side = if *checked { Side::Right } else { Side::Left };
                            cx.notify();
                        })),
                )
                .child(
                    Anahtar::new("click-to-toggle")
                        .checked(self.click_to_toggle_submenu)
                        .label("Oturum gruplarını tıklamayla aç/kapat")
                        .on_click(cx.listener(|this, checked: &bool, _, cx| {
                            this.click_to_toggle_submenu = *checked;
                            cx.notify();
                        })),
                )
                .child(
                    Anahtar::new("dynamic-children")
                        .checked(self.show_dynamic_children)
                        .label("Bağlı çalışma ağacı alt öğelerini göster")
                        .on_click(cx.listener(|this, checked: &bool, _, cx| {
                            this.show_dynamic_children = *checked;
                            cx.notify();
                        })),
                ),
        )
    }

    fn switch_checked_handler(
        &mut self,
        checked: &bool,
        _: &mut Window,
        _: &mut Context<SidebarStory>,
    ) {
        self.checked = *checked;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Item {
    Playground,
    Models,
    Documentation,
    Ayarlar,
    DesignEngineering,
    SalesAndMarketing,
    Travel,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SubItem {
    History,
    Starred,
    General,
    Team,
    Billing,
    Limits,
    Ayarlar,
    Genesis,
    Explorer,
    Quantum,
    Introduction,
    GetStarted,
    Tutorial,
    Changelog,
}

impl Item {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Playground => "Oturumlar",
            Self::Models => "Ajan Paneli",
            Self::Documentation => "Proje Paneli",
            Self::Ayarlar => "Ayarlar",
            Self::DesignEngineering => "Dal Farkı",
            Self::SalesAndMarketing => "Git Paneli",
            Self::Travel => "Uzak Geliştirme",
        }
    }

    pub fn is_disabled(&self) -> bool {
        match self {
            Self::Travel => true,
            _ => false,
        }
    }

    pub fn icon(&self) -> SimgeAdi {
        match self {
            Self::Playground => SimgeAdi::SquareTerminal,
            Self::Models => SimgeAdi::Bot,
            Self::Documentation => SimgeAdi::FolderOpen,
            Self::Ayarlar => SimgeAdi::Settings2,
            Self::DesignEngineering => SimgeAdi::Replace,
            Self::SalesAndMarketing => SimgeAdi::Github,
            Self::Travel => SimgeAdi::Globe,
        }
    }

    pub fn handler(
        &self,
    ) -> impl Fn(&mut SidebarStory, &ClickEvent, &mut Window, &mut Context<SidebarStory>) + 'static
    {
        let item = *self;
        move |this, _, _, cx| {
            if this.active_items.contains_key(&item) {
                this.active_items.remove(&item);
            } else {
                this.active_items.insert(item, true);
            }

            this.last_active_item = item;
            this.active_subitem = None;
            cx.notify();
        }
    }

    pub fn items(&self) -> Vec<SubItem> {
        match self {
            Self::Playground => vec![SubItem::History, SubItem::Starred, SubItem::Ayarlar],
            Self::Models => vec![SubItem::Genesis, SubItem::Explorer, SubItem::Quantum],
            Self::Documentation => vec![
                SubItem::Introduction,
                SubItem::GetStarted,
                SubItem::Tutorial,
                SubItem::Changelog,
            ],
            Self::Ayarlar => vec![
                SubItem::General,
                SubItem::Team,
                SubItem::Billing,
                SubItem::Limits,
            ],
            _ => Vec::new(),
        }
    }
}

impl SubItem {
    pub fn label(&self) -> &'static str {
        match self {
            Self::History => "Çalışıyor",
            Self::Starred => "Arşivlenenler",
            Self::Ayarlar => "Kurallar",
            Self::Genesis => "Codex",
            Self::Explorer => "Claude Ajanı",
            Self::Quantum => "OpenCode",
            Self::Introduction => "Dosyalar",
            Self::GetStarted => "Ana Hat",
            Self::Tutorial => "Yer İmleri",
            Self::Changelog => "Tanılamalar",
            Self::Team => "Ajan",
            Self::Billing => "Sağlayıcılar",
            Self::Limits => "İzinler",
            Self::General => "Genel",
        }
    }

    pub fn is_disabled(&self) -> bool {
        match self {
            Self::Quantum => true,
            _ => false,
        }
    }

    pub fn handler(
        &self,
        item: &Item,
    ) -> impl Fn(&mut SidebarStory, &ClickEvent, &mut Window, &mut Context<SidebarStory>) + 'static
    {
        let item = *item;
        let subitem = *self;
        move |this, _, _, cx| {
            println!(
                "Öğeye tıklandı: {}, alt öğe: {}",
                item.label(),
                subitem.label()
            );
            this.active_items.insert(item, true);
            this.last_active_item = item;
            this.active_subitem = Some(subitem);
            cx.notify();
        }
    }
}

impl super::Story for SidebarStory {
    fn title() -> &'static str {
        "YanCubuk"
    }

    fn description() -> &'static str {
        "Birleştirilebilir, temalanabilir ve özelleştirilebilir kenar çubuğu bileşeni."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl Focusable for SidebarStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SidebarStory {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let groups: [Vec<Item>; 2] = [
            vec![
                Item::Playground,
                Item::Models,
                Item::Documentation,
                Item::Ayarlar,
            ],
            vec![
                Item::DesignEngineering,
                Item::SalesAndMarketing,
                Item::Travel,
            ],
        ];

        h_flex()
            .rounded(cx.theme().radius)
            .border_1()
            .border_color(cx.theme().border)
            .h_full()
            .when(self.side.is_right(), |this| this.flex_row_reverse())
            .child(
                YanCubuk::new("sidebar-story")
                    .side(self.side)
                    .collapsed(self.collapsed)
                    .w(px(220.))
                    .gap_0()
                    .header(
                        YanCubukBasligi::new()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded(cx.theme().radius)
                                    .bg(cx.theme().success)
                                    .text_color(cx.theme().success_foreground)
                                    .size_8()
                                    .flex_shrink_0()
                                    .when(!self.collapsed, |this| {
                                        this.child(Simge::new(SimgeAdi::GalleryVerticalEnd))
                                    })
                                    .when(self.collapsed, |this| {
                                        this.size_4()
                                            .bg(cx.theme().transparent)
                                            .text_color(cx.theme().foreground)
                                            .child(Simge::new(SimgeAdi::GalleryVerticalEnd))
                                    }),
                            )
                            .when(!self.collapsed, |this| {
                                this.child(
                                    v_flex()
                                        .gap_0()
                                        .text_sm()
                                        .flex_1()
                                        .line_height(relative(1.25))
                                        .overflow_hidden()
                                        .text_ellipsis()
                                        .child("Zed")
                                        .child(div().child("1.0 Çalışma Alanı").text_xs()),
                                )
                            })
                            .when(!self.collapsed, |this| {
                                this.child(
                                    Simge::new(SimgeAdi::ChevronsUpDown)
                                        .size_4()
                                        .flex_shrink_0(),
                                )
                            })
                            .acilir_menu(|menu, _, _| {
                                menu.menu(
                                    "Zed 1.0",
                                    Box::new(SelectCompany(SharedString::from("zed-1-0"))),
                                )
                                .menu(
                                    "Paralel Ajanlar",
                                    Box::new(SelectCompany(SharedString::from("agents"))),
                                )
                                .menu(
                                    "Bölünmüş Diff",
                                    Box::new(SelectCompany(SharedString::from("split-diff"))),
                                )
                            }),
                    )
                    .child(YanCubukGrubu::new("Ajanlı Çalışma").child(
                        YanCubukMenusu::new().children(groups[0].iter().enumerate().map(
                            |(ix, item)| {
                                let is_active =
                                    self.last_active_item == *item && self.active_subitem == None;
                                YanCubukMenuOgesi::new(item.label())
                                    .icon(item.icon())
                                    .active(is_active)
                                    .default_open(ix == 0)
                                    .click_to_toggle(self.click_to_toggle_submenu)
                                    .when(ix == 0, |this| {
                                        this.baglam_menusu({
                                            move |this, _, _| {
                                                this.link(
                                                    "Paralel Ajanlar",
                                                    "https://zed.dev/blog/parallel-agents",
                                                )
                                            }
                                        })
                                    })
                                    .children(item.items().into_iter().enumerate().map(
                                        |(ix, sub_item)| {
                                            YanCubukMenuOgesi::new(sub_item.label())
                                                .active(self.active_subitem == Some(sub_item))
                                                .disable(sub_item.is_disabled())
                                                .when(ix == 0, |this| {
                                                    this.suffix({
                                                        let checked = self.checked;
                                                        let view = cx.entity();
                                                        move |window, _| {
                                                            Anahtar::new("switch")
                                                                .xsmall()
                                                                .checked(checked)
                                                                .on_click(window.listener_for(
                                                                    &view,
                                                                    Self::switch_checked_handler,
                                                                ))
                                                        }
                                                    })
                                                    .baglam_menusu({
                                                        move |this, _, _| {
                                                            this.label("Bu bir etikettir")
                                                        }
                                                    })
                                                })
                                                .on_click(cx.listener(sub_item.handler(&item)))
                                        },
                                    ))
                                    .on_click(cx.listener(item.handler()))
                            },
                        )),
                    ))
                    .child(YanCubukGrubu::new("Çalışma Alanı").child(
                        YanCubukMenusu::new().children(groups[1].iter().enumerate().map(
                            |(ix, item)| {
                                let is_active =
                                    self.last_active_item == *item && self.active_subitem == None;
                                YanCubukMenuOgesi::new(item.label())
                                    .icon(item.icon())
                                    .active(is_active)
                                    .disable(item.is_disabled())
                                    .click_to_toggle(self.click_to_toggle_submenu)
                                    .when(ix == 0 && self.show_dynamic_children, |this| {
                                        this.default_open(true).children(vec![
                                            YanCubukMenuOgesi::new("Bağlı çalışma ağacı")
                                                .on_click(cx.listener(|_, _, _, _| {})),
                                            YanCubukMenuOgesi::new("Bölünmüş diff")
                                                .on_click(cx.listener(|_, _, _, _| {})),
                                        ])
                                    })
                                    .when(ix == 0, |this| {
                                        this.suffix(|_, _| {
                                            Rozet::new().dot().count(1).child(
                                                div().p_0p5().child(Simge::new(SimgeAdi::Bell)),
                                            )
                                        })
                                    })
                                    .when(ix == 1, |this| {
                                        this.suffix(|_, _| Simge::new(SimgeAdi::Settings2))
                                    })
                                    .on_click(cx.listener(item.handler()))
                            },
                        )),
                    ))
                    .footer(
                        YanCubukAltligi::new()
                            .justify_between()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(SimgeAdi::CircleUser)
                                    .when(!self.collapsed, |this| this.child("Paralel Ajanlar")),
                            )
                            .when(!self.collapsed, |this| {
                                this.child(Simge::new(SimgeAdi::ChevronsUpDown).size_4())
                            }),
                    ),
            )
            .child(
                v_flex()
                    .size_full()
                    .gap_4()
                    .p_4()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_3()
                            .when(self.side.is_right(), |this| {
                                this.flex_row_reverse().justify_between()
                            })
                            .child(
                                YanCubukGecisDugmesi::new()
                                    .side(self.side)
                                    .collapsed(self.collapsed)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.collapsed = !this.collapsed;
                                        cx.notify();
                                    })),
                            )
                            .child(Ayirici::vertical().h_4())
                            .child(
                                GezintiYolu::new()
                                    .child("Çalışma Alanı")
                                    .child(GezintiYoluOgesi::new("Zed").on_click(cx.listener(
                                        |this, _, _, cx| {
                                            this.last_active_item = Item::Playground;
                                            cx.notify();
                                        },
                                    )))
                                    .child(
                                        GezintiYoluOgesi::new(self.last_active_item.label())
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.active_subitem = None;
                                                cx.notify();
                                            })),
                                    )
                                    .when_some(self.active_subitem, |this, subitem| {
                                        this.child(GezintiYoluOgesi::new(subitem.label()))
                                    }),
                            ),
                    )
                    .child(self.render_content(window, cx)),
            )
    }
}
