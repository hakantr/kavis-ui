use itertools::Itertools as _;
use kavis_ui::ham_gpui::*;
use kavis_ui::{
    Boyutlandirilabilir as _, EtkinTema as _, IndexPath, SimgeAdi,
    button::{Dugme, DugmeVaryantlari as _},
    checkbox::OnayKutusu,
    divider::Ayirici,
    h_flex,
    input::{Girdi, GirdiDurumu},
    select::{AranabilirListe, Secim, SecimDurumu, SecimGrubu, SecimOgesi, SecimOlayi},
    v_flex,
};
use serde::{Deserialize, Serialize};

use crate::section;

pub fn init(_: &mut App) {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Country {
    name: SharedString,
    code: SharedString,
}

impl Country {
    pub fn letter_prefix(&self) -> char {
        self.name.chars().next().unwrap_or(' ')
    }
}

impl SecimOgesi for Country {
    type Value = SharedString;

    fn title(&self) -> SharedString {
        self.name.clone()
    }

    fn display_title(&self) -> Option<kavis_ui::ham_gpui::AnyElement> {
        Some(format!("{} ({})", self.name, self.code).into_any_element())
    }

    fn value(&self) -> &Self::Value {
        &self.code
    }
}

pub struct SelectStory {
    disabled: bool,
    country_select: Entity<SecimDurumu<AranabilirListe<SecimGrubu<Country>>>>,
    fruit_select: Entity<SecimDurumu<AranabilirListe<&'static str>>>,
    simple_select1: Entity<SecimDurumu<Vec<&'static str>>>,
    simple_select2: Entity<SecimDurumu<AranabilirListe<&'static str>>>,
    simple_select3: Entity<SecimDurumu<Vec<SharedString>>>,
    menu_max_h_select: Entity<SecimDurumu<Vec<&'static str>>>,
    disabled_select: Entity<SecimDurumu<Vec<SharedString>>>,
    appearance_select: Entity<SecimDurumu<Vec<SharedString>>>,
    input_state: Entity<GirdiDurumu>,
}

impl super::Story for SelectStory {
    fn title() -> &'static str {
        "Secim"
    }

    fn description() -> &'static str {
        "Displays a list of options for the user to pick from—triggered by a button."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl Focusable for SelectStory {
    fn focus_handle(&self, cx: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.fruit_select.focus_handle(cx)
    }
}

impl SelectStory {
    fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let countries =
            serde_json::from_str::<Vec<Country>>(include_str!("../fixtures/countries.json"))
                .unwrap();
        let mut grouped_countries: AranabilirListe<SecimGrubu<Country>> =
            AranabilirListe::new(vec![]);
        for (prefix, items) in countries.iter().chunk_by(|c| c.letter_prefix()).into_iter() {
            let items = items.cloned().collect::<Vec<Country>>();
            grouped_countries.push(SecimGrubu::new(prefix.to_string()).items(items));
        }

        let country_select = cx.new(|cx| {
            SecimDurumu::new(
                grouped_countries,
                Some(IndexPath::default().row(8).section(2)),
                window,
                cx,
            )
            .searchable(true)
        });
        let appearance_select = cx.new(|cx| {
            SecimDurumu::new(
                vec![
                    "CN".into(),
                    "US".into(),
                    "HK".into(),
                    "JP".into(),
                    "KR".into(),
                ],
                Some(IndexPath::default()),
                window,
                cx,
            )
        });
        let input_state =
            cx.new(|cx| GirdiDurumu::new(window, cx).placeholder("Telefon numaranız"));

        let fruits = AranabilirListe::new(vec![
            "Elma",
            "Portakal",
            "Muz",
            "Üzüm",
            "Ananas",
            "Karpuz ve çok çok çok çok çok çok çok uzun bir başlık",
            "Avokado",
        ]);
        let fruit_select = cx.new(|cx| SecimDurumu::new(fruits, None, window, cx).searchable(true));

        cx.new(|cx| {
            cx.subscribe_in(&country_select, window, Self::on_select_event)
                .detach();

            Self {
                disabled: false,
                country_select,
                fruit_select,
                simple_select1: cx.new(|cx| {
                    SecimDurumu::new(
                        vec![
                            "GPUI", "Iced", "egui", "Makepad", "Slint", "QT", "ImGui", "Cocoa",
                            "WinUI",
                        ],
                        Some(IndexPath::default()),
                        window,
                        cx,
                    )
                }),
                simple_select2: cx.new(|cx| {
                    let mut select =
                        SecimDurumu::new(AranabilirListe::new(vec![]), None, window, cx)
                            .searchable(true);

                    select.set_items(
                        AranabilirListe::new(vec!["Rust", "Go", "C++", "JavaScript"]),
                        window,
                        cx,
                    );

                    select
                }),
                simple_select3: cx
                    .new(|cx| SecimDurumu::new(Vec::<SharedString>::new(), None, window, cx)),
                menu_max_h_select: cx.new(|cx| {
                    SecimDurumu::new(
                        vec![
                            "GPUI", "Iced", "egui", "Makepad", "Slint", "QT", "ImGui", "Cocoa",
                            "WinUI",
                        ],
                        Some(IndexPath::default()),
                        window,
                        cx,
                    )
                }),
                disabled_select: cx
                    .new(|cx| SecimDurumu::new(Vec::<SharedString>::new(), None, window, cx)),
                appearance_select,
                input_state,
            }
        })
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        Self::new(window, cx)
    }

    fn on_select_event(
        &mut self,
        _: &Entity<SecimDurumu<AranabilirListe<SecimGrubu<Country>>>>,
        event: &SecimOlayi<AranabilirListe<SecimGrubu<Country>>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        match event {
            SecimOlayi::Confirm(value) => println!("Seçilen ülke: {:?}", value),
        }
    }

    fn toggle_disabled(&mut self, disabled: bool, _: &mut Window, cx: &mut Context<Self>) {
        self.disabled = disabled;
        cx.notify();
    }
}

impl Render for SelectStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_4()
            .child(
                OnayKutusu::new("disable-selects")
                    .label("Devre Dışı")
                    .checked(self.disabled)
                    .on_click(cx.listener(|this, checked, window, cx| {
                        this.toggle_disabled(*checked, window, cx);
                    })),
            )
            .child(
                section("Secim").max_w_128().child(
                    Secim::new(&self.country_select)
                        .search_placeholder("Ülkeyi ad veya kod ile ara")
                        .cleanable(true)
                        .disabled(self.disabled),
                ),
            )
            .child(
                section("Searchable").max_w_128().child(
                    Secim::new(&self.fruit_select)
                        .disabled(self.disabled)
                        .icon(SimgeAdi::Search)
                        .w(px(320.))
                        .menu_width(px(400.)),
                ),
            )
            .child(
                section("Devre Dışı")
                    .max_w_128()
                    .child(Secim::new(&self.disabled_select).disabled(true)),
            )
            .child(
                section("With preview label").max_w_128().child(
                    Secim::new(&self.simple_select1)
                        .disabled(self.disabled)
                        .small()
                        .placeholder("UI")
                        .title_prefix("UI: "),
                ),
            )
            .child(
                section("Custom Menu Max Height").max_w_128().child(
                    Secim::new(&self.menu_max_h_select)
                        .disabled(self.disabled)
                        .small()
                        .placeholder("UI")
                        .title_prefix("UI: ")
                        .menu_max_h(rems(6.)),
                ),
            )
            .child(
                section("Searchable Secim").max_w_128().child(
                    Secim::new(&self.simple_select2)
                        .disabled(self.disabled)
                        .small()
                        .placeholder("Dil")
                        .title_prefix("Language: "),
                ),
            )
            .child(
                section("Empty Items").max_w_128().child(
                    Secim::new(&self.simple_select3)
                        .disabled(self.disabled)
                        .small()
                        .empty(
                            h_flex()
                                .h_24()
                                .justify_center()
                                .text_color(cx.theme().muted_foreground)
                                .child("Veri Yok"),
                        ),
                ),
            )
            .child(
                section("Appearance false with Girdi").max_w_128().child(
                    h_flex()
                        .border_1()
                        .border_color(cx.theme().input)
                        .rounded(cx.theme().radius_lg)
                        .text_color(cx.theme().secondary_foreground)
                        .w_full()
                        .gap_1()
                        .child(
                            div().w(px(140.)).child(
                                Secim::new(&self.appearance_select)
                                    .appearance(false)
                                    .py_2()
                                    .pl_3(),
                            ),
                        )
                        .child(Ayirici::vertical())
                        .child(
                            div().flex_1().child(
                                Girdi::new(&self.input_state)
                                    .appearance(false)
                                    .pr_3()
                                    .py_2(),
                            ),
                        )
                        .child(
                            div()
                                .p_2()
                                .child(Dugme::new("send").small().ghost().label("Gönder")),
                        ),
                ),
            )
            .child(
                section("Selected Values").max_w_lg().child(
                    v_flex()
                        .gap_3()
                        .child(format!(
                            "Ülke: {:?}",
                            self.country_select.read(cx).selected_value()
                        ))
                        .child(format!(
                            "Meyve: {:?}",
                            self.fruit_select.read(cx).selected_value()
                        ))
                        .child(format!(
                            "UI: {:?}",
                            self.simple_select1.read(cx).selected_value()
                        ))
                        .child(format!(
                            "Dil: {:?}",
                            self.simple_select2.read(cx).selected_value()
                        ))
                        .child("Bu başka bir metindir."),
                ),
            )
    }
}
