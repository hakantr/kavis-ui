use kavis_ui::ham_gpui::*;
use kavis_ui::ham_gpui::{
    App, AppContext, Axis, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Window,
};
use kavis_ui::{
    BilesenBoyutu, Boyutlandirilabilir as _,
    button::Dugme,
    checkbox::OnayKutusu,
    description_list::{AciklamaListesi, AciklamaOgesi},
    dock::PanelDenetimi,
    text::MetinGorunumu,
    v_flex,
};
use kavis_ui::{EksenUzantisi, h_flex, menu::AcilirMenuTetikleyici as _};
use serde::Deserialize;

#[derive(kavis_ui::Aksiyon, Clone, PartialEq, Eq, Deserialize)]
#[aksiyon(namespace = description_list_story, no_json)]
struct ChangeSize(BilesenBoyutu);

pub struct DescriptionListStory {
    focus_handle: FocusHandle,
    layout: Axis,
    bordered: bool,
    size: BilesenBoyutu,
    items: Vec<(&'static str, &'static str, usize)>,
}

impl DescriptionListStory {
    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let items = vec![
            ("Ad", "Kavis UI", 1),
            (
                "Açıklama",
                "[GPUI](https://gpui.rs) kullanarak masaüstü uygulamaları geliştirmek için UI bileşenleri.\
                \n\n \
                **Dugme**, **Girdi**, **Tablo**, **Liste**, **Secim**, **TarihSecici** gibi birçok kullanışlı UI bileşeni içerir. \
                \n\n \
                Kavis UI ile yerel masaüstü uygulamanızı kolayca oluşturabilirsiniz.
                ",
                3,
            ),
            ("Sürüm", "0.1.0", 1),
            ("Lisans", "Apache-2.0", 1),
            ("Yazar", "Longbridge", 1),
            ("--", "--", 1),
            (
                "Depo",
                "https://github.com/hakantr/kavis-ui",
                2,
            ),
            (
                "Kategori",
                "UI, Desktop, Framework",
                1,
            ),
            (
                "Platform için uzun bir etiket",
                "macOS, Windows, Linux",
                1,
            ),
        ];

        Self {
            items,
            bordered: true,
            size: BilesenBoyutu::default(),
            layout: Axis::Horizontal,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn set_layout(&mut self, layout: Axis, cx: &mut Context<Self>) {
        self.layout = layout;
        cx.notify();
    }

    fn set_bordered(&mut self, bordered: bool, cx: &mut Context<Self>) {
        self.bordered = bordered;
        cx.notify();
    }

    fn on_change_size(&mut self, a: &ChangeSize, _: &mut Window, cx: &mut Context<Self>) {
        self.size = a.0;
        cx.notify();
    }
}

impl super::Story for DescriptionListStory {
    fn title() -> &'static str {
        "AciklamaListesi"
    }

    fn description() -> &'static str {
        "Use to display details with a tidy layout."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn zoomable() -> Option<PanelDenetimi> {
        None
    }
}

impl Focusable for DescriptionListStory {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DescriptionListStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("example")
            .on_action(cx.listener(Self::on_change_size))
            .p_4()
            .size_full()
            .gap_2()
            .child(
                h_flex()
                    .gap_3()
                    .child(
                        OnayKutusu::new("layout")
                            .checked(self.layout.dikey_mi())
                            .label("Dikey Yerleşim")
                            .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                let new_layout = if *checked {
                                    Axis::Vertical
                                } else {
                                    Axis::Horizontal
                                };
                                this.set_layout(new_layout, cx);
                            })),
                    )
                    .child(
                        OnayKutusu::new("bordered")
                            .checked(self.bordered)
                            .label("Kenarlıklı")
                            .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                this.set_bordered(*checked, cx);
                            })),
                    )
                    .child(
                        Dugme::new("size")
                            .small()
                            .outline()
                            .label(format!("boyut: {:?}", self.size))
                            .acilir_menu({
                                let size = self.size;
                                move |menu, _, _| {
                                    menu.menu_with_check(
                                        "Büyük",
                                        size == BilesenBoyutu::Buyuk,
                                        Box::new(ChangeSize(BilesenBoyutu::Buyuk)),
                                    )
                                    .menu_with_check(
                                        "Orta",
                                        size == BilesenBoyutu::Orta,
                                        Box::new(ChangeSize(BilesenBoyutu::Orta)),
                                    )
                                    .menu_with_check(
                                        "Küçük",
                                        size == BilesenBoyutu::Kucuk,
                                        Box::new(ChangeSize(BilesenBoyutu::Kucuk)),
                                    )
                                }
                            }),
                    ),
            )
            .child(
                AciklamaListesi::new()
                    .columns(3)
                    .layout(self.layout)
                    .bordered(self.bordered)
                    .with_size(self.size)
                    .children(self.items.clone().into_iter().enumerate().map(
                        |(ix, (label, value, span))| {
                            if label == "--" {
                                return AciklamaOgesi::Ayirici;
                            }

                            AciklamaOgesi::new(label)
                                .value(MetinGorunumu::markdown(ix, value).into_any_element())
                                .span(span)
                        },
                    )),
            )
    }
}
