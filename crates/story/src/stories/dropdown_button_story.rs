use gpui::{
    Action, Anchor, App, AppContext as _, Context, Entity, Focusable, IntoElement,
    ParentElement as _, Render, Styled as _, Window, prelude::FluentBuilder as _,
};
use serde::Deserialize;

use crate::section;
use kavis_ui::{
    Boyutlandirilabilir as _, DevreDisiBirakilabilir, EtkinTema, Secilebilir as _, Tema,
    button::{AcilirDugme, Dugme, DugmeVaryantlari as _},
    checkbox::OnayKutusu,
    h_flex, v_flex,
};

#[derive(Clone, Action, PartialEq, Eq, Deserialize)]
#[action(namespace = dropdown_button_story, no_json)]
enum ButtonAction {
    Disabled,
    Loading,
    Selected,
    Compact,
}

pub struct DropdownButtonStory {
    focus_handle: gpui::FocusHandle,
    disabled: bool,
    loading: bool,
    selected: bool,
    compact: bool,
}

impl DropdownButtonStory {
    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            disabled: false,
            loading: false,
            selected: false,
            compact: false,
        })
    }
}

impl super::Story for DropdownButtonStory {
    fn title() -> &'static str {
        "AcilirDugme"
    }

    fn description() -> &'static str {
        "A button with an attached dropdown menu for additional options."
    }

    fn closable() -> bool {
        false
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl Focusable for DropdownButtonStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DropdownButtonStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let disabled = self.disabled;
        let loading = self.loading;
        let selected = self.selected;
        let compact = self.compact;

        v_flex()
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
                section("Dropdown Dugme").child(
                    AcilirDugme::new("btn0")
                        .primary()
                        .button(Dugme::new("btn").label("Birincil Açılır Düğme"))
                        .when(self.compact, |this| this.compact())
                        .loading(self.loading)
                        .disabled(self.disabled)
                        .selected(selected)
                        .dropdown_menu_with_anchor(Anchor::BottomRight, move |this, _, _| {
                            this.menu_with_check(
                                "Devre Dışı",
                                disabled,
                                Box::new(ButtonAction::Disabled),
                            )
                            .menu_with_check("Yükleniyor", loading, Box::new(ButtonAction::Loading))
                            .menu_with_check("Seçili", selected, Box::new(ButtonAction::Selected))
                            .menu_with_check(
                                "Kompakt",
                                compact,
                                Box::new(ButtonAction::Compact),
                            )
                        }),
                ),
            )
            .child(
                section("Small Size").child(
                    AcilirDugme::new("btn-sm")
                        .small()
                        .button(Dugme::new("btn").label("Küçük Açılır Düğme"))
                        .when(self.compact, |this| this.compact())
                        .loading(self.loading)
                        .disabled(self.disabled)
                        .selected(selected)
                        .dropdown_menu(move |this, _, _| {
                            this.menu_with_check(
                                "Devre Dışı",
                                disabled,
                                Box::new(ButtonAction::Disabled),
                            )
                            .menu_with_check("Yükleniyor", loading, Box::new(ButtonAction::Loading))
                            .menu_with_check("Seçili", selected, Box::new(ButtonAction::Selected))
                            .menu_with_check(
                                "Kompakt",
                                compact,
                                Box::new(ButtonAction::Compact),
                            )
                        }),
                ),
            )
            .child(
                section("Outline").child(
                    AcilirDugme::new("btn-outline")
                        .outline()
                        .danger()
                        .button(Dugme::new("btn").label("Çerçeveli Açılır Düğme"))
                        .when(self.compact, |this| this.compact())
                        .loading(self.loading)
                        .disabled(self.disabled)
                        .selected(selected)
                        .dropdown_menu(move |this, _, _| {
                            this.menu_with_check(
                                "Devre Dışı",
                                disabled,
                                Box::new(ButtonAction::Disabled),
                            )
                            .menu_with_check("Yükleniyor", loading, Box::new(ButtonAction::Loading))
                            .menu_with_check("Seçili", selected, Box::new(ButtonAction::Selected))
                            .menu_with_check(
                                "Kompakt",
                                compact,
                                Box::new(ButtonAction::Compact),
                            )
                        }),
                ),
            )
            .child(
                section("Ghost").child(
                    AcilirDugme::new("btn-ghost")
                        .ghost()
                        .button(Dugme::new("btn").label("Hayalet Açılır Düğme"))
                        .when(self.compact, |this| this.compact())
                        .loading(self.loading)
                        .disabled(self.disabled)
                        .selected(selected)
                        .dropdown_menu(move |this, _, _| {
                            this.menu_with_check(
                                "Devre Dışı",
                                disabled,
                                Box::new(ButtonAction::Disabled),
                            )
                            .menu_with_check("Yükleniyor", loading, Box::new(ButtonAction::Loading))
                            .menu_with_check("Seçili", selected, Box::new(ButtonAction::Selected))
                            .menu_with_check(
                                "Kompakt",
                                compact,
                                Box::new(ButtonAction::Compact),
                            )
                        }),
                ),
            )
    }
}
