use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, Focusable, InteractiveElement, KeyBinding, ParentElement,
    Render, StatefulInteractiveElement as _, Styled, Window, actions, div,
};

use kavis_ui::{
    SimgeAdi,
    button::{Dugme, DugmeVaryanti, DugmeVaryantlari, Gecis},
    checkbox::OnayKutusu,
    clipboard::Pano,
    dock::PanelDenetimi,
    h_flex,
    radio::Radyo,
    switch::Anahtar,
    tooltip::AracIpucu,
    v_flex,
};

use crate::{Story, section};

actions!(tooltip_story, [Info]);

pub fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new(
        "ctrl-shift-delete",
        Info,
        Some("AracIpucu"),
    )]);
}

pub struct TooltipStory {
    focus_handle: kavis_ui::ham_gpui::FocusHandle,
}

impl TooltipStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Story for TooltipStory {
    fn title() -> &'static str {
        "AracIpucu"
    }

    fn description() -> &'static str {
        "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }

    fn zoomable() -> Option<PanelDenetimi> {
        None
    }
}

impl Focusable for TooltipStory {
    fn focus_handle(&self, _: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TooltipStory {
    fn render(
        &mut self,
        _: &mut kavis_ui::ham_gpui::Window,
        _cx: &mut kavis_ui::ham_gpui::Context<Self>,
    ) -> impl kavis_ui::ham_gpui::IntoElement {
        v_flex()
            .w_full()
            .gap_3()
            .child(
                section("AracIpucu for Dugme")
                    .child(
                        Dugme::new("btn0")
                            .label("Ara")
                            .with_variant(DugmeVaryanti::Primary)
                            .tooltip("Bu bir arama düğmesidir."),
                    )
                    .child(Dugme::new("btn1").label("Bilgi").tooltip_with_action(
                        "Bu, klavye kısayolunu göstermek için eylem içeren bir ipucudur.",
                        &Info,
                        Some("AracIpucu"),
                    ))
                    .child(
                        Dugme::new("btn3")
                            .label("Üzerime gel")
                            .tooltip("Bu ipucu 3"),
                    ),
            )
            .child(
                section("OnayKutusu AracIpucu").child(
                    OnayKutusu::new("check")
                        .label("Beni hatırla")
                        .checked(true)
                        .tooltip("Bu bir ipucudur"),
                ),
            )
            .child(
                section("Radyo AracIpucu").child(
                    Radyo::new("radio")
                        .label("İpuçlu radyo")
                        .checked(true)
                        .tooltip("Bu bir radyo düğmesidir"),
                ),
            )
            .child(
                section("Anahtar AracIpucu").child(
                    Anahtar::new("switch")
                        .checked(true)
                        .tooltip("Bu bir anahtardır"),
                ),
            )
            .child(
                section("Gecis AracIpucu").child(
                    h_flex()
                        .gap_2()
                        .child(
                            Gecis::new("toggle1")
                                .label("Kalın")
                                .tooltip("Kalını aç/kapat"),
                        )
                        .child(
                            Gecis::new("toggle2")
                                .icon(SimgeAdi::Heart)
                                .tooltip("Favoriyi aç/kapat"),
                        ),
                ),
            )
            .child(
                section("Pano AracIpucu").child(
                    Pano::new("clip1")
                        .value("Hello, World!")
                        .tooltip("Panoya kopyala"),
                ),
            )
            .child(
                section("Default AracIpucu").child(
                    div()
                        .child("Üzerime gel")
                        .id("tooltip-2")
                        .tooltip(|window, cx| {
                            AracIpucu::new("Bu, GPUI varsayılan ipucu stilidir.")
                                .action(&Info, Some("AracIpucu"))
                                .build(window, cx)
                        }),
                ),
            )
    }
}
