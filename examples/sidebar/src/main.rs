use kavis_ui::ham_gpui::{prelude::FluentBuilder as _, *};
use kavis_ui::{
    Boyutlandirilabilir, EtkinTema, KokGorunum, Secilebilir, Simge, SimgeAdi, StilUzantisi,
    button::Dugme,
    h_flex,
    sidebar::{
        YanCubuk, YanCubukAltligi, YanCubukBasligi, YanCubukDaralma, YanCubukGecisDugmesi,
        YanCubukGrubu, YanCubukMenuOgesi, YanCubukMenusu,
    },
    v_flex,
};

pub struct Ornek {
    collapsible: YanCubukDaralma,
    collapsed: bool,
}

impl Ornek {
    fn new() -> Self {
        Self {
            collapsible: YanCubukDaralma::Icon,
            collapsed: false,
        }
    }

    fn menu() -> YanCubukMenusu {
        YanCubukMenusu::new().children([
            YanCubukMenuOgesi::new("Gösterge Paneli")
                .icon(SimgeAdi::LayoutDashboard)
                .active(true),
            YanCubukMenuOgesi::new("Gelen Kutusu").icon(SimgeAdi::Inbox),
            YanCubukMenuOgesi::new("Takvim").icon(SimgeAdi::Calendar),
            YanCubukMenuOgesi::new("Projeler")
                .icon(SimgeAdi::Folder)
                .default_open(true)
                .click_to_toggle(true)
                .children([
                    YanCubukMenuOgesi::new("Tasarım"),
                    YanCubukMenuOgesi::new("Mühendislik"),
                    YanCubukMenuOgesi::new("Pazarlama"),
                ]),
            YanCubukMenuOgesi::new("Ayarlar").icon(SimgeAdi::Settings),
        ])
    }

    fn aciklama(&self) -> &'static str {
        match self.collapsible {
            YanCubukDaralma::Icon => {
                "Yan çubuk simge genişliğine daralır; shadcn collapsible=\"icon\" davranışına karşılık gelir."
            }
            YanCubukDaralma::Offcanvas => {
                "Yan çubuk daralınca yerleşim genişliğini serbest bırakır ve gizlenen kontrolleri klavye geziniminden çıkarır; shadcn collapsible=\"offcanvas\" davranışına karşılık gelir."
            }
            YanCubukDaralma::None => {
                "Yan çubuk collapsed durumunu yok sayar ve açık kalır; shadcn collapsible=\"none\" davranışına karşılık gelir."
            }
        }
    }

    fn mod_dugmesi(
        &self,
        id: &'static str,
        label: &'static str,
        mode: YanCubukDaralma,
        cx: &mut Context<Self>,
    ) -> Dugme {
        Dugme::new(id)
            .label(label)
            .small()
            .selected(self.collapsible == mode)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.collapsible = mode;
                cx.notify();
            }))
    }
}

impl Render for Ornek {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let icon_collapsed = self.collapsed && self.collapsible == YanCubukDaralma::Icon;
        let show_toggle = self.collapsible != YanCubukDaralma::None;

        h_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(
                YanCubuk::new("sidebar-example")
                    .collapsible(self.collapsible)
                    .collapsed(self.collapsed)
                    .w(px(240.))
                    .header(
                        YanCubukBasligi::new()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .size_8()
                                    .flex_shrink_0()
                                    .rounded(cx.theme().radius)
                                    .bg(cx.theme().sidebar_primary)
                                    .text_color(cx.theme().sidebar_primary_foreground)
                                    .when(icon_collapsed, |this| {
                                        this.size_4()
                                            .bg(cx.theme().transparent)
                                            .text_color(cx.theme().foreground)
                                    })
                                    .child(Simge::new(SimgeAdi::GalleryVerticalEnd)),
                            )
                            .when(!icon_collapsed, |this| {
                                this.child(
                                    v_flex()
                                        .flex_1()
                                        .overflow_hidden()
                                        .child("Kavis Inc")
                                        .child(div().text_xs().child("Çalışma Alanı")),
                                )
                            }),
                    )
                    .child(YanCubukGrubu::new("Uygulama").child(Self::menu()))
                    .footer(
                        YanCubukAltligi::new().child(
                            h_flex()
                                .gap_2()
                                .child(SimgeAdi::CircleUser)
                                .when(!icon_collapsed, |this| this.child("Hakan Biris")),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .h_full()
                    .flex_1()
                    .min_w_0()
                    .gap_4()
                    .p_4()
                    .child(
                        h_flex()
                            .items_center()
                            .gap_3()
                            .when(show_toggle, |this| {
                                this.child(
                                    YanCubukGecisDugmesi::new()
                                        .collapsed(icon_collapsed)
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.collapsed = !this.collapsed;
                                            cx.notify();
                                        })),
                                )
                            })
                            .child(div().font_bold().child("Yan çubuk daralma modları")),
                    )
                    .child(
                        h_flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_sm().child("Mod:"))
                            .child(self.mod_dugmesi("mode-icon", "Icon", YanCubukDaralma::Icon, cx))
                            .child(self.mod_dugmesi(
                                "mode-offcanvas",
                                "Offcanvas",
                                YanCubukDaralma::Offcanvas,
                                cx,
                            ))
                            .child(self.mod_dugmesi(
                                "mode-none",
                                "None",
                                YanCubukDaralma::None,
                                cx,
                            )),
                    )
                    .child(
                        div()
                            .flex_1()
                            .rounded(cx.theme().radius)
                            .border_1()
                            .border_color(cx.theme().border)
                            .p_5()
                            .child(self.aciklama()),
                    ),
            )
    }
}

fn main() {
    let app = kavis_ui::platform::application().with_assets(kavis_ui_assets::Varliklar);

    app.run(move |cx| {
        kavis_ui::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(900.), px(620.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|_| Ornek::new());
                cx.new(|cx| KokGorunum::new(view, window, cx))
            })
            .expect("Pencere açılamadı");
        })
        .detach();
    });
}
