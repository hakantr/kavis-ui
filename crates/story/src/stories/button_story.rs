use kavis_ui::ham_gpui::{
    Action, App, AppContext as _, Axis, ClickEvent, Context, Entity, Focusable, InteractiveElement,
    IntoElement, ParentElement as _, Render, Styled as _, Window, prelude::FluentBuilder, px,
};

use kavis_ui::{
    Boyutlandirilabilir as _, DevreDisiBirakilabilir as _, EtkinTema, Secilebilir as _, Simge,
    SimgeAdi, Tema,
    button::{Dugme, DugmeGrubu, DugmeOzelVaryanti, DugmeVaryantlari as _},
    checkbox::OnayKutusu,
    h_flex,
    progress::DaireselIlerleme,
    v_flex,
};
use serde::Deserialize;

use crate::section;

#[derive(Clone, Action, PartialEq, Eq, Deserialize)]
#[action(namespace = button_story, no_json)]
enum ButtonAction {
    Disabled,
    Loading,
    Selected,
    Compact,
}

pub struct ButtonStory {
    focus_handle: kavis_ui::ham_gpui::FocusHandle,
    disabled: bool,
    loading: bool,
    selected: bool,
    compact: bool,
    toggle_multiple: bool,
}

impl ButtonStory {
    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            disabled: false,
            loading: false,
            selected: false,
            compact: false,
            toggle_multiple: false,
        })
    }

    fn on_click(ev: &ClickEvent, _: &mut Window, _: &mut App) {
        println!("Düğmeye tıklandı: {:?}", ev);
    }

    fn on_hover(hovered: &bool, _: &mut Window, _: &mut App) {
        println!("Düğmenin üzerine gelindi: {:?}", hovered);
    }
}

impl super::Story for ButtonStory {
    fn title() -> &'static str {
        "Dugme"
    }

    fn description() -> &'static str {
        "Displays a button or a component that looks like a button."
    }

    fn closable() -> bool {
        false
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl Focusable for ButtonStory {
    fn focus_handle(&self, _: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ButtonStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let disabled = self.disabled;
        let loading = self.loading;
        let selected = self.selected;
        let compact = self.compact;
        let toggle_multiple = self.toggle_multiple;

        let custom_variant = DugmeOzelVaryanti::new(cx)
            .color(cx.theme().magenta)
            .foreground(cx.theme().magenta)
            .hover(cx.theme().magenta.opacity(0.1))
            .active(cx.theme().magenta)
            .shadow(true);

        v_flex()
            .on_action(
                cx.listener(|this, action: &ButtonAction, _, _| match action {
                    ButtonAction::Disabled => this.disabled = !this.disabled,
                    ButtonAction::Loading => this.loading = !this.loading,
                    ButtonAction::Selected => this.selected = !this.selected,
                    ButtonAction::Compact => this.compact = !this.compact,
                }),
            )
            .gap_6()
            .child(
                h_flex()
                    .gap_3()
                    .child(
                        OnayKutusu::new("disabled-button")
                            .label("Devre Dışı")
                            .checked(self.disabled)
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.disabled = !view.disabled;
                                cx.notify();
                            })),
                    )
                    .child(
                        OnayKutusu::new("loading-button")
                            .label("Yükleniyor")
                            .checked(self.loading)
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.loading = !view.loading;
                                cx.notify();
                            })),
                    )
                    .child(
                        OnayKutusu::new("selected-button")
                            .label("Seçili")
                            .checked(self.selected)
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.selected = !view.selected;
                                cx.notify();
                            })),
                    )
                    .child(
                        OnayKutusu::new("compact-button")
                            .label("Kompakt")
                            .checked(self.compact)
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.compact = !view.compact;
                                cx.notify();
                            })),
                    )
                    .child(
                        OnayKutusu::new("shadow-button")
                            .label("Gölge")
                            .checked(cx.theme().shadow)
                            .on_click(cx.listener(|_, _, window, cx| {
                                let mut theme = cx.theme().clone();
                                theme.shadow = !theme.shadow;
                                cx.set_global::<Tema>(theme);
                                window.refresh();
                            })),
                    ),
            )
            .child(
                section("Normal Düğme")
                    .max_w_lg()
                    .child(
                        Dugme::new("button-0")
                            .label("Varsayılan")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_hover(Self::on_hover),
                    )
                    .child(
                        Dugme::new("button-1")
                            .primary()
                            .label("Birincil")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_hover(Self::on_hover),
                    )
                    .child(
                        Dugme::new("button-2")
                            .secondary()
                            .label("İkincil")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_hover(Self::on_hover),
                    )
                    .child(
                        Dugme::new("button-4")
                            .danger()
                            .label("Tehlike")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_hover(Self::on_hover),
                    )
                    .child(
                        Dugme::new("button-4-warning")
                            .warning()
                            .label("Uyarı")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_hover(Self::on_hover),
                    )
                    .child(
                        Dugme::new("button-4-success")
                            .success()
                            .label("Başarılı")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_hover(Self::on_hover),
                    )
                    .child(
                        Dugme::new("button-5-info")
                            .info()
                            .label("Bilgi")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_hover(Self::on_hover),
                    )
                    .child(
                        Dugme::new("button-5-ghost")
                            .ghost()
                            .label("Hayalet")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_hover(Self::on_hover),
                    )
                    .child(
                        Dugme::new("button-5-link")
                            .link()
                            .label("Bağlantı")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_hover(Self::on_hover),
                    )
                    .child(
                        Dugme::new("button-5-text")
                            .text()
                            .label("Metin")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click)
                            .on_hover(Self::on_hover),
                    ),
            )
            .child(
                section("Dugme with Simge")
                    .child(
                        Dugme::new("button-icon-1")
                            .outline()
                            .label("Onayla")
                            .icon(SimgeAdi::Check)
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-icon-2")
                            .outline()
                            .label("Vazgeç")
                            .icon(SimgeAdi::Close)
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-icon-3")
                            .outline()
                            .label("Büyüt")
                            .icon(Simge::new(SimgeAdi::Maximize))
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-icon-4")
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_2()
                                    .child("Özel Alt Öğe")
                                    .child(SimgeAdi::ChevronDown)
                                    .child(SimgeAdi::Eye),
                            )
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-icon-5-ghost")
                            .ghost()
                            .icon(SimgeAdi::Check)
                            .label("Onayla")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-icon-6-link")
                            .link()
                            .icon(SimgeAdi::Check)
                            .label("Bağlantı")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-icon-6-text")
                            .text()
                            .icon(SimgeAdi::Check)
                            .label("Metin Düğmesi")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    ),
            )
            .child(
                section("With Ilerleme").child(
                    h_flex()
                        .gap_4()
                        .child(
                            Dugme::new("progress-button-1")
                                .primary()
                                .large()
                                .icon(
                                    DaireselIlerleme::new("circle-progress-1")
                                        .color(cx.theme().primary_foreground)
                                        .value(25.),
                                )
                                .label("Kuruluyor..."),
                        )
                        .child(
                            Dugme::new("progress-button-2")
                                .icon(DaireselIlerleme::new("circle-progress-2").value(35.))
                                .label("Kuruluyor..."),
                        )
                        .child(
                            Dugme::new("progress-button-3")
                                .small()
                                .icon(DaireselIlerleme::new("circle-progress-3").value(68.))
                                .label("Kuruluyor..."),
                        )
                        .child(
                            Dugme::new("progress-button-4")
                                .xsmall()
                                .icon(DaireselIlerleme::new("circle-progress-4").value(85.))
                                .label("Kuruluyor..."),
                        ),
                ),
            )
            .child(
                section("Çerçeveli Düğme")
                    .max_w_lg()
                    .child(
                        Dugme::new("button-outline-1")
                            .primary()
                            .outline()
                            .label("Birincil Düğme")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-2")
                            .outline()
                            .label("Normal Düğme")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-4-danger")
                            .danger()
                            .outline()
                            .label("Tehlike Düğmesi")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-4-warning")
                            .warning()
                            .outline()
                            .label("Uyarı Düğmesi")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-4-success")
                            .success()
                            .outline()
                            .label("Başarı Düğmesi")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-5-info")
                            .info()
                            .outline()
                            .label("Bilgi Düğmesi")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-5-ghost")
                            .ghost()
                            .outline()
                            .label("Hayalet Düğme")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-5-link")
                            .link()
                            .outline()
                            .label("Bağlantı Düğmesi")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-5-text")
                            .text()
                            .outline()
                            .label("Metin Düğmesi")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    ),
            )
            .child(
                section("With Dropdown Caret")
                    .max_w_lg()
                    .child(
                        Dugme::new("button-outline-1")
                            .primary()
                            .dropdown_caret(true)
                            .label("Birincil Düğme")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-2")
                            .label("Varsayılan Düğme")
                            .dropdown_caret(true)
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-2")
                            .secondary()
                            .label("İkincil Düğme")
                            .dropdown_caret(true)
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-5-ghost")
                            .ghost()
                            .dropdown_caret(true)
                            .label("Hayalet Düğme")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-5-link")
                            .link()
                            .dropdown_caret(true)
                            .label("Bağlantı Düğmesi")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-5-text")
                            .outline()
                            .small()
                            .dropdown_caret(true)
                            .label("Küçük Düğme")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    ),
            )
            .child(
                section("Small Size")
                    .child(
                        Dugme::new("button-6")
                            .label("Birincil Düğme")
                            .icon(SimgeAdi::Check)
                            .primary()
                            .small()
                            .loading(true)
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-7")
                            .label("İkincil Düğme")
                            .small()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-8")
                            .label("Tehlike Düğmesi")
                            .danger()
                            .small()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-8-outline")
                            .label("Çerçeveli Düğme")
                            .outline()
                            .small()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-8-ghost")
                            .label("Hayalet Düğme")
                            .ghost()
                            .small()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-8-link")
                            .label("Bağlantı Düğmesi")
                            .link()
                            .small()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    ),
            )
            .child(
                section("XSmall Size")
                    .child(
                        Dugme::new("button-xs-1")
                            .label("Birincil Düğme")
                            .primary()
                            .icon(SimgeAdi::Check)
                            .xsmall()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-xs-2")
                            .label("İkincil Düğme")
                            .xsmall()
                            .loading(true)
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-xs-3")
                            .label("Tehlike Düğmesi")
                            .danger()
                            .xsmall()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-xs-3-ghost")
                            .label("Hayalet Düğme")
                            .ghost()
                            .xsmall()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-xs-3-outline")
                            .label("Çerçeveli Düğme")
                            .outline()
                            .xsmall()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-xs-3-link")
                            .label("Bağlantı Düğmesi")
                            .link()
                            .xsmall()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    ),
            )
            .child(
                section("Dugme Group").child(
                    DugmeGrubu::new("button-group")
                        .outline()
                        .disabled(disabled)
                        .child(
                            Dugme::new("button-one")
                                .label("Bir")
                                .disabled(disabled)
                                .selected(selected)
                                .when(compact, |this| this.compact())
                                .on_click(Self::on_click),
                        )
                        .child(
                            Dugme::new("button-two")
                                .label("İki")
                                .disabled(disabled)
                                .selected(selected)
                                .when(compact, |this| this.compact())
                                .on_click(Self::on_click),
                        )
                        .child(
                            Dugme::new("button-three")
                                .label("Üç")
                                .disabled(disabled)
                                .selected(selected)
                                .when(compact, |this| this.compact())
                                .on_click(Self::on_click),
                        ),
                ),
            )
            .child(
                section("Dugme Group (Vertical)").child(
                    DugmeGrubu::new("button-group-vertical")
                        .outline()
                        .layout(Axis::Vertical)
                        .disabled(disabled)
                        .child(
                            Dugme::new("button-one")
                                .label("Bir")
                                .disabled(disabled)
                                .selected(selected)
                                .when(compact, |this| this.compact())
                                .on_click(Self::on_click),
                        )
                        .child(
                            Dugme::new("button-two")
                                .label("İki")
                                .disabled(disabled)
                                .selected(selected)
                                .when(compact, |this| this.compact())
                                .on_click(Self::on_click),
                        )
                        .child(
                            Dugme::new("button-three")
                                .label("Üç")
                                .disabled(disabled)
                                .selected(selected)
                                .when(compact, |this| this.compact())
                                .on_click(Self::on_click),
                        ),
                ),
            )
            .child(
                section("Gecis Dugme Group")
                    .sub_title(
                        OnayKutusu::new("multiple-button")
                            .text_sm()
                            .label("Çoklu")
                            .checked(toggle_multiple)
                            .on_click(cx.listener(|view, _, _, cx| {
                                view.toggle_multiple = !view.toggle_multiple;
                                cx.notify();
                            })),
                    )
                    .child(
                        DugmeGrubu::new("toggle-button-group")
                            .outline()
                            .compact()
                            .multiple(toggle_multiple)
                            .child(
                                Dugme::new("disabled-toggle-button")
                                    .label("Devre Dışı")
                                    .selected(disabled),
                            )
                            .child(
                                Dugme::new("loading-toggle-button")
                                    .label("Yükleniyor")
                                    .selected(loading),
                            )
                            .child(
                                Dugme::new("selected-toggle-button")
                                    .label("Seçili")
                                    .selected(selected),
                            )
                            .child(
                                Dugme::new("compact-toggle-button")
                                    .label("Kompakt")
                                    .selected(compact),
                            )
                            .on_click(cx.listener(|view, selected: &Vec<usize>, _, cx| {
                                view.disabled = selected.contains(&0);
                                view.loading = selected.contains(&1);
                                view.selected = selected.contains(&2);
                                view.compact = selected.contains(&3);
                                cx.notify();
                            })),
                    ),
            )
            .child(
                section("Simge Düğmesi")
                    .child(
                        Dugme::new("icon-button-primary")
                            .icon(SimgeAdi::Search)
                            .loading_icon(SimgeAdi::LoaderCircle)
                            .primary()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Dugme::new("icon-button-secondary")
                            .icon(SimgeAdi::Info)
                            .loading(true)
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Dugme::new("icon-button-danger")
                            .icon(SimgeAdi::Close)
                            .danger()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Dugme::new("icon-button-small-primary")
                            .icon(SimgeAdi::Search)
                            .small()
                            .primary()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Dugme::new("icon-button-outline")
                            .icon(SimgeAdi::Search)
                            .outline()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Dugme::new("icon-button-ghost")
                            .icon(SimgeAdi::ArrowLeft)
                            .loading_icon(SimgeAdi::LoaderCircle)
                            .ghost()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    ),
            )
            .child(
                section("Simge Düğmesi")
                    .child(
                        Dugme::new("icon-button-4")
                            .icon(SimgeAdi::Info)
                            .small()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Dugme::new("icon-button-5")
                            .icon(SimgeAdi::Close)
                            .small()
                            .danger()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Dugme::new("icon-button-6")
                            .icon(SimgeAdi::Search)
                            .small()
                            .primary()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Dugme::new("icon-button-7")
                            .icon(SimgeAdi::Info)
                            .xsmall()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Dugme::new("icon-button-8")
                            .icon(SimgeAdi::Close)
                            .xsmall()
                            .danger()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    )
                    .child(
                        Dugme::new("icon-button-9")
                            .icon(SimgeAdi::Heart)
                            .size(px(24.))
                            .ghost()
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact()),
                    ),
            )
            .child(
                section("Özel Düğme")
                    .child(
                        Dugme::new("button-6-custom")
                            .custom(custom_variant)
                            .label("Özel Düğme")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-6-custom")
                            .outline()
                            .custom(custom_variant)
                            .label("Çerçeveli Düğme")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    )
                    .child(
                        Dugme::new("button-outline-6-custom-1")
                            .outline()
                            .icon(SimgeAdi::Bell)
                            .custom(custom_variant)
                            .label("Simge Düğmesi")
                            .disabled(disabled)
                            .selected(selected)
                            .loading(loading)
                            .when(compact, |this| this.compact())
                            .on_click(Self::on_click),
                    ),
            )
    }
}
