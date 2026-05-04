use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, ParentElement, Render, Styled,
    Subscription, Window,
};
use kavis_ui::{
    BilesenBoyutu, Boyutlandirilabilir, Secilebilir as _, SimgeAdi, StilUzantisi,
    button::{Dugme, DugmeGrubu},
    checkbox::OnayKutusu,
    h_flex,
    stepper::{Adimlayici, AdimlayiciOgesi},
    v_flex,
};

use crate::section;

pub struct StepperStory {
    focus_handle: kavis_ui::ham_gpui::FocusHandle,
    size: BilesenBoyutu,
    stepper0_step: usize,
    stepper1_step: usize,
    stepper2_step: usize,
    stepper3_step: usize,
    disabled: bool,
    _subscritions: Vec<Subscription>,
}

impl super::Story for StepperStory {
    fn title() -> &'static str {
        "Adimlayici"
    }

    fn description() -> &'static str {
        "A step-by-step process for users to navigate through a series of steps."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl StepperStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            size: BilesenBoyutu::default(),
            stepper0_step: 1,
            stepper1_step: 0,
            stepper2_step: 2,
            stepper3_step: 0,
            disabled: false,
            _subscritions: vec![],
        }
    }
}

impl Focusable for StepperStory {
    fn focus_handle(&self, _: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for StepperStory {
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
                                    .selected(self.size == BilesenBoyutu::CokKucuk),
                            )
                            .child(
                                Dugme::new("small")
                                    .label("Küçük")
                                    .selected(self.size == BilesenBoyutu::Kucuk),
                            )
                            .child(
                                Dugme::new("medium")
                                    .label("Orta")
                                    .selected(self.size == BilesenBoyutu::Orta),
                            )
                            .child(
                                Dugme::new("large")
                                    .label("Büyük")
                                    .selected(self.size == BilesenBoyutu::Buyuk),
                            )
                            .on_click(cx.listener(|this, selecteds: &Vec<usize>, _, cx| {
                                let size = match selecteds[0] {
                                    0 => BilesenBoyutu::CokKucuk,
                                    1 => BilesenBoyutu::Kucuk,
                                    2 => BilesenBoyutu::Orta,
                                    3 => BilesenBoyutu::Buyuk,
                                    _ => unreachable!(),
                                };
                                this.size = size;
                                cx.notify();
                            })),
                    )
                    .child(
                        OnayKutusu::new("disabled")
                            .checked(self.disabled)
                            .label("Devre Dışı")
                            .on_click(cx.listener(|this, check: &bool, _, cx| {
                                this.disabled = *check;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                section("Horizontal Adimlayici").max_w_md().v_flex().child(
                    Adimlayici::new("stepper0")
                        .w_full()
                        .with_size(self.size)
                        .disabled(self.disabled)
                        .selected_index(self.stepper0_step)
                        .items([
                            AdimlayiciOgesi::new().child("Adım 1"),
                            AdimlayiciOgesi::new().child("Adım 2"),
                            AdimlayiciOgesi::new().child("Adım 3"),
                        ])
                        .on_click(cx.listener(|this, step, _, cx| {
                            this.stepper0_step = *step;
                            cx.notify();
                        })),
                ),
            )
            .child(
                section("Simge Adimlayici").max_w_md().v_flex().child(
                    Adimlayici::new("stepper1")
                        .w_full()
                        .with_size(self.size)
                        .disabled(self.disabled)
                        .selected_index(self.stepper1_step)
                        .items([
                            AdimlayiciOgesi::new()
                                .icon(SimgeAdi::Calendar)
                                .child("Sipariş Ayrıntıları"),
                            AdimlayiciOgesi::new().icon(SimgeAdi::Inbox).child("Kargo"),
                            AdimlayiciOgesi::new()
                                .icon(SimgeAdi::Frame)
                                .child("Önizleme"),
                            AdimlayiciOgesi::new().icon(SimgeAdi::Info).child("Bitir"),
                        ])
                        .on_click(cx.listener(|this, step, _, cx| {
                            this.stepper1_step = *step;
                            cx.notify();
                        })),
                ),
            )
            .child(
                section("Vertical Adimlayici").max_w_md().v_flex().child(
                    Adimlayici::new("stepper3")
                        .vertical()
                        .with_size(self.size)
                        .disabled(self.disabled)
                        .selected_index(self.stepper2_step)
                        .items_center()
                        .items([
                            AdimlayiciOgesi::new()
                                .pb_8()
                                .icon(SimgeAdi::Building2)
                                .child(v_flex().child("Adım 1").child("Adım 1 için açıklama.")),
                            AdimlayiciOgesi::new()
                                .pb_8()
                                .icon(SimgeAdi::Asterisk)
                                .child(v_flex().child("Adım 2").child("Adım 2 için açıklama.")),
                            AdimlayiciOgesi::new()
                                .pb_8()
                                .icon(SimgeAdi::Folder)
                                .child(v_flex().child("Adım 3").child("Adım 3 için açıklama.")),
                            AdimlayiciOgesi::new()
                                .icon(SimgeAdi::CircleCheck)
                                .child(v_flex().child("Adım 4").child("Adım 4 için açıklama.")),
                        ])
                        .on_click(cx.listener(|this, step, _, cx| {
                            this.stepper2_step = *step;
                            cx.notify();
                        })),
                ),
            )
            .child(
                section("Text Center").max_w_md().v_flex().child(
                    Adimlayici::new("stepper4")
                        .with_size(self.size)
                        .disabled(self.disabled)
                        .selected_index(self.stepper3_step)
                        .text_center(true)
                        .items([
                            AdimlayiciOgesi::new().child(
                                v_flex()
                                    .items_center()
                                    .child("Adım 1")
                                    .child("Adım 1 açıklaması."),
                            ),
                            AdimlayiciOgesi::new().child(
                                v_flex()
                                    .items_center()
                                    .child("Adım 2")
                                    .child("Adım 2 açıklaması."),
                            ),
                            AdimlayiciOgesi::new().child(
                                v_flex()
                                    .items_center()
                                    .child("Adım 3")
                                    .child("Adım 3 açıklaması."),
                            ),
                        ])
                        .on_click(cx.listener(|this, step, _, cx| {
                            this.stepper3_step = *step;
                            cx.notify();
                        })),
                ),
            )
    }
}
