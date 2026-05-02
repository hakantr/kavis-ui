use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement, Render,
    Styled, Window,
};

use kavis_ui::{
    EtkinTema as _, Selectable as _, SimgeAdi, Sizable, Size,
    button::{Dugme, DugmeGrubu, DugmeVaryantlari},
    checkbox::OnayKutusu,
    h_flex,
    tab::{Sekme, SekmeCubugu},
    v_flex,
};

use crate::section;

pub struct TabsStory {
    focus_handle: FocusHandle,
    active_tab_ix: usize,
    size: Size,
    menu: bool,
}

impl super::Story for TabsStory {
    fn title() -> &'static str {
        "Tabs"
    }

    fn description() -> &'static str {
        "A set of layered sections of content—known as tab panels—that are displayed one at a time."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl TabsStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            active_tab_ix: 0,
            size: Size::default(),
            menu: false,
        }
    }

    fn set_active_tab(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.active_tab_ix = ix;
        cx.notify();
    }

    fn set_size(&mut self, size: Size, _: &mut Window, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }
}

impl Focusable for TabsStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TabsStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_3()
            .child(
                h_flex()
                    .gap_3()
                    .child(
                        DugmeGrubu::new("toggle-size")
                            .outline()
                            .compact()
                            .child(
                                Dugme::new("xsmall")
                                    .label("Çok Küçük")
                                    .selected(self.size == Size::XSmall),
                            )
                            .child(
                                Dugme::new("small")
                                    .label("Küçük")
                                    .selected(self.size == Size::Small),
                            )
                            .child(
                                Dugme::new("medium")
                                    .label("Orta")
                                    .selected(self.size == Size::Medium),
                            )
                            .child(
                                Dugme::new("large")
                                    .label("Büyük")
                                    .selected(self.size == Size::Large),
                            )
                            .on_click(cx.listener(|this, selecteds: &Vec<usize>, window, cx| {
                                let size = match selecteds[0] {
                                    0 => Size::XSmall,
                                    1 => Size::Small,
                                    2 => Size::Medium,
                                    3 => Size::Large,
                                    _ => unreachable!(),
                                };
                                this.set_size(size, window, cx);
                            })),
                    )
                    .child(
                        OnayKutusu::new("show-menu")
                            .label("Daha fazla menü")
                            .checked(self.menu)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.menu = !this.menu;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                section("Tabs").max_w_md().child(
                    SekmeCubugu::new("tabs")
                        .w_full()
                        .with_size(self.size)
                        .menu(self.menu)
                        .selected_index(self.active_tab_ix)
                        .on_click(cx.listener(|this, ix: &usize, window, cx| {
                            this.set_active_tab(*ix, window, cx);
                        }))
                        .border_t_1()
                        .border_color(cx.theme().border)
                        .prefix(
                            h_flex()
                                .mx_1()
                                .child(
                                    Dugme::new("back")
                                        .ghost()
                                        .xsmall()
                                        .icon(SimgeAdi::ArrowLeft),
                                )
                                .child(
                                    Dugme::new("forward")
                                        .ghost()
                                        .xsmall()
                                        .icon(SimgeAdi::ArrowRight),
                                ),
                        )
                        .child(Sekme::new().label("Hesap"))
                        .child(Sekme::new().label("Profil").disabled(true))
                        .child(Sekme::new().label("Belgeler"))
                        .child(Sekme::new().label("Posta"))
                        .child(Sekme::new().label("Görünüm"))
                        .child(Sekme::new().label("Ayarlar"))
                        .child(Sekme::new().label("Hakkında"))
                        .child(Sekme::new().label("Lisans"))
                        .suffix(
                            h_flex()
                                .mx_1()
                                .child(Dugme::new("inbox").ghost().xsmall().icon(SimgeAdi::Inbox))
                                .child(
                                    Dugme::new("more").ghost().xsmall().icon(SimgeAdi::Ellipsis),
                                ),
                        ),
                ),
            )
            .child(
                section("Underline Tabs").max_w_md().child(
                    SekmeCubugu::new("underline")
                        .w_full()
                        .underline()
                        .with_size(self.size)
                        .menu(self.menu)
                        .selected_index(self.active_tab_ix)
                        .on_click(cx.listener(|this, ix: &usize, window, cx| {
                            this.set_active_tab(*ix, window, cx);
                        }))
                        .child("Hesap")
                        .child("Profil")
                        .child("Belgeler")
                        .child("Posta")
                        .child("Görünüm")
                        .child("Ayarlar")
                        .child("Hakkında")
                        .child("Lisans"),
                ),
            )
            .child(
                section("Pill Tabs").max_w_md().child(
                    SekmeCubugu::new("pill")
                        .w_full()
                        .pill()
                        .with_size(self.size)
                        .menu(self.menu)
                        .selected_index(self.active_tab_ix)
                        .on_click(cx.listener(|this, ix: &usize, window, cx| {
                            this.set_active_tab(*ix, window, cx);
                        }))
                        .child(Sekme::new().label("Hesap"))
                        .child(Sekme::new().label("Profil").disabled(true))
                        .child(Sekme::new().label("Belgeler ve Dosyalar"))
                        .child(Sekme::new().label("Posta"))
                        .child(Sekme::new().label("Görünüm"))
                        .child(Sekme::new().label("Ayarlar"))
                        .child(Sekme::new().label("Hakkında"))
                        .child(Sekme::new().label("Lisans")),
                ),
            )
            .child(
                section("Outline Tabs").max_w_md().child(
                    SekmeCubugu::new("outline")
                        .w_full()
                        .outline()
                        .with_size(self.size)
                        .menu(self.menu)
                        .selected_index(self.active_tab_ix)
                        .on_click(cx.listener(|this, ix: &usize, window, cx| {
                            this.set_active_tab(*ix, window, cx);
                        }))
                        .child(Sekme::new().label("Hesap"))
                        .child(Sekme::new().label("Profil").disabled(true))
                        .child(Sekme::new().label("Belgeler ve Dosyalar"))
                        .child(Sekme::new().label("Posta"))
                        .child(Sekme::new().label("Görünüm"))
                        .child(Sekme::new().label("Ayarlar"))
                        .child(Sekme::new().label("Hakkında"))
                        .child(Sekme::new().label("Lisans")),
                ),
            )
            .child(
                section("Segmented Tabs").max_w_md().child(
                    SekmeCubugu::new("segmented")
                        .w_full()
                        .segmented()
                        .with_size(self.size)
                        .menu(self.menu)
                        .selected_index(self.active_tab_ix)
                        .on_click(cx.listener(|this, ix: &usize, window, cx| {
                            this.set_active_tab(*ix, window, cx);
                        }))
                        .child(SimgeAdi::Bot)
                        .child(SimgeAdi::Calendar)
                        .child(SimgeAdi::Map)
                        .children(vec!["Görünüm", "Ayarlar", "Hakkında", "Lisans"]),
                ),
            )
            .child(
                section("Segmented Tabs (With filling space)")
                    .max_w_md()
                    .child(
                        SekmeCubugu::new("flex tabs")
                            .w_full()
                            .segmented()
                            .with_size(self.size)
                            .selected_index(self.active_tab_ix)
                            .on_click(cx.listener(|this, ix: &usize, window, cx| {
                                this.set_active_tab(*ix, window, cx);
                            }))
                            .child(Sekme::new().flex_1().label("Hakkında"))
                            .child(Sekme::new().flex_1().label("Profil")),
                    ),
            )
    }
}
