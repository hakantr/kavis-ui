use gpui::{
    App, AppContext as _, Context, Entity, FocusHandle, Focusable, IntoElement, ParentElement as _,
    Render, Styled as _, Window,
};

use kavis_ui::{
    Boyutlandirilabilir, SimgeAdi, StilUzantisi,
    button::{Gecis, GecisGrubu, GecisVaryantlari},
    v_flex,
};

use crate::section;

pub struct ToggleStory {
    focus_handle: FocusHandle,
    single_toggle: usize,
    checked: Vec<bool>,
}

impl ToggleStory {
    pub fn view(_: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            single_toggle: 0,
            checked: vec![false; 20],
        })
    }
}

impl super::Story for ToggleStory {
    fn title() -> &'static str {
        "ToggleButton"
    }

    fn description() -> &'static str {
        ""
    }

    fn closable() -> bool {
        false
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl Focusable for ToggleStory {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ToggleStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_3()
            .child(
                section("Gecis")
                    .child(
                        Gecis::new("item1")
                            .label("Tekli Geçiş Öğesi 1")
                            .large()
                            .checked(self.single_toggle == 1)
                            .on_click(cx.listener(|view, checked, _, cx| {
                                if *checked {
                                    view.single_toggle = 1;
                                }
                                cx.notify();
                            })),
                    )
                    .child(
                        Gecis::new("item2")
                            .label("Tekli Geçiş Öğesi 2")
                            .large()
                            .checked(self.single_toggle == 2)
                            .on_click(cx.listener(|view, checked, _, cx| {
                                if *checked {
                                    view.single_toggle = 2;
                                }
                                cx.notify();
                            })),
                    )
                    .child(
                        Gecis::new("item3")
                            .icon(SimgeAdi::Eye)
                            .large()
                            .checked(self.single_toggle == 3)
                            .on_click(cx.listener(|view, checked, _, cx| {
                                if *checked {
                                    view.single_toggle = 3;
                                }
                                cx.notify();
                            })),
                    ),
            )
            .child(
                section("Gecis Group with Ghost Style")
                    .v_flex()
                    .gap_4()
                    .child(
                        GecisGrubu::new("toggle-button-group1")
                            .child(Gecis::new(0).icon(SimgeAdi::Bell).checked(self.checked[0]))
                            .child(Gecis::new(1).icon(SimgeAdi::Bot).checked(self.checked[1]))
                            .child(Gecis::new(2).icon(SimgeAdi::Inbox).checked(self.checked[2]))
                            .child(Gecis::new(3).icon(SimgeAdi::Check).checked(self.checked[3]))
                            .child(Gecis::new(4).label("Diğer").checked(self.checked[4]))
                            .on_click(cx.listener(|view, checkeds: &Vec<bool>, _, cx| {
                                view.checked[0] = checkeds[0];
                                view.checked[1] = checkeds[1];
                                view.checked[2] = checkeds[2];
                                view.checked[3] = checkeds[3];
                                view.checked[4] = checkeds[4];
                                cx.notify();
                            })),
                    )
                    .child(
                        GecisGrubu::new("toggle-button-group1-sm")
                            .small()
                            .child(Gecis::new(0).icon(SimgeAdi::Bell).checked(self.checked[0]))
                            .child(Gecis::new(1).icon(SimgeAdi::Bot).checked(self.checked[1]))
                            .child(Gecis::new(2).icon(SimgeAdi::Inbox).checked(self.checked[2]))
                            .child(Gecis::new(3).icon(SimgeAdi::Check).checked(self.checked[3]))
                            .child(Gecis::new(4).label("Diğer").checked(self.checked[4]))
                            .on_click(cx.listener(|view, checkeds: &Vec<bool>, _, cx| {
                                view.checked[0] = checkeds[0];
                                view.checked[1] = checkeds[1];
                                view.checked[2] = checkeds[2];
                                view.checked[3] = checkeds[3];
                                view.checked[4] = checkeds[4];
                                cx.notify();
                            })),
                    )
                    .child(
                        GecisGrubu::new("toggle-button-group1-xs")
                            .xsmall()
                            .child(Gecis::new(0).icon(SimgeAdi::Bell).checked(self.checked[0]))
                            .child(Gecis::new(1).icon(SimgeAdi::Bot).checked(self.checked[1]))
                            .child(Gecis::new(2).icon(SimgeAdi::Inbox).checked(self.checked[2]))
                            .child(Gecis::new(3).icon(SimgeAdi::Check).checked(self.checked[3]))
                            .child(Gecis::new(4).label("Diğer").checked(self.checked[4]))
                            .on_click(cx.listener(|view, checkeds: &Vec<bool>, _, cx| {
                                view.checked[0] = checkeds[0];
                                view.checked[1] = checkeds[1];
                                view.checked[2] = checkeds[2];
                                view.checked[3] = checkeds[3];
                                view.checked[4] = checkeds[4];
                                cx.notify();
                            })),
                    ),
            )
            .child(
                section("Gecis Group with Outline Style")
                    .v_flex()
                    .gap_4()
                    .child(
                        GecisGrubu::new("toggle-button-group2")
                            .outline()
                            .child(Gecis::new(0).icon(SimgeAdi::Bell).checked(self.checked[0]))
                            .child(Gecis::new(1).icon(SimgeAdi::Bot).checked(self.checked[1]))
                            .child(Gecis::new(2).icon(SimgeAdi::Inbox).checked(self.checked[2]))
                            .child(Gecis::new(3).icon(SimgeAdi::Check).checked(self.checked[3]))
                            .child(Gecis::new(4).label("Diğer").checked(self.checked[4]))
                            .on_click(cx.listener(|view, checkeds: &Vec<bool>, _, cx| {
                                view.checked[0] = checkeds[0];
                                view.checked[1] = checkeds[1];
                                view.checked[2] = checkeds[2];
                                view.checked[3] = checkeds[3];
                                view.checked[4] = checkeds[4];
                                cx.notify();
                            })),
                    )
                    .child(
                        GecisGrubu::new("toggle-button-group2-sm")
                            .outline()
                            .small()
                            .child(Gecis::new(0).icon(SimgeAdi::Bell).checked(self.checked[0]))
                            .child(Gecis::new(1).icon(SimgeAdi::Bot).checked(self.checked[1]))
                            .child(Gecis::new(2).icon(SimgeAdi::Inbox).checked(self.checked[2]))
                            .child(Gecis::new(3).icon(SimgeAdi::Check).checked(self.checked[3]))
                            .child(Gecis::new(4).label("Diğer").checked(self.checked[4]))
                            .on_click(cx.listener(|view, checkeds: &Vec<bool>, _, cx| {
                                view.checked[0] = checkeds[0];
                                view.checked[1] = checkeds[1];
                                view.checked[2] = checkeds[2];
                                view.checked[3] = checkeds[3];
                                view.checked[4] = checkeds[4];
                                cx.notify();
                            })),
                    )
                    .child(
                        GecisGrubu::new("toggle-button-group2-xs")
                            .outline()
                            .xsmall()
                            .child(Gecis::new(0).icon(SimgeAdi::Bell).checked(self.checked[0]))
                            .child(Gecis::new(1).icon(SimgeAdi::Bot).checked(self.checked[1]))
                            .child(Gecis::new(2).icon(SimgeAdi::Inbox).checked(self.checked[2]))
                            .child(Gecis::new(3).icon(SimgeAdi::Check).checked(self.checked[3]))
                            .child(Gecis::new(4).label("Diğer").checked(self.checked[4]))
                            .on_click(cx.listener(|view, checkeds: &Vec<bool>, _, cx| {
                                view.checked[0] = checkeds[0];
                                view.checked[1] = checkeds[1];
                                view.checked[2] = checkeds[2];
                                view.checked[3] = checkeds[3];
                                view.checked[4] = checkeds[4];
                                cx.notify();
                            })),
                    ),
            )
            .child(
                section("Gecis Group with Segmented Style")
                    .v_flex()
                    .gap_4()
                    .child(
                        GecisGrubu::new("toggle-button-group-segmented-outline")
                            .segmented()
                            .outline()
                            .child(Gecis::new(0).label("Kalın").checked(self.checked[5]))
                            .child(Gecis::new(1).label("İtalik").checked(self.checked[6]))
                            .child(Gecis::new(2).label("Kod").checked(self.checked[7]))
                            .on_click(cx.listener(|view, checkeds: &Vec<bool>, _, cx| {
                                for (offset, checked) in checkeds.iter().enumerate() {
                                    view.checked[5 + offset] = *checked;
                                }
                                cx.notify();
                            })),
                    )
                    .child(
                        GecisGrubu::new("toggle-button-group-segmented-gap")
                            .segmented()
                            .outline()
                            .small()
                            .child(Gecis::new(0).label("Yıldız").checked(self.checked[8]))
                            .child(Gecis::new(1).label("İzle").checked(self.checked[9]))
                            .child(Gecis::new(2).label("Sabitle").checked(self.checked[10]))
                            .on_click(cx.listener(|view, checkeds: &Vec<bool>, _, cx| {
                                for (offset, checked) in checkeds.iter().enumerate() {
                                    view.checked[8 + offset] = *checked;
                                }
                                cx.notify();
                            })),
                    ),
            )
    }
}
