use kavis_ui::ham_gpui::{
    App, AppContext, Context, Entity, Focusable, Hsla, IntoElement, ParentElement, Render,
    SharedString, Styled, Subscription, Window, hsla, px,
};
use kavis_ui::{
    EtkinTema, PencereUzantisi, Renklendir as _, StilUzantisi,
    checkbox::OnayKutusu,
    clipboard::Pano,
    h_flex,
    slider::{Kaydirici, KaydiriciDurumu, KaydiriciOlayi, KaydiriciOlcegi},
    v_flex,
};

use crate::section;

pub struct SliderStory {
    focus_handle: kavis_ui::ham_gpui::FocusHandle,
    slider1: Entity<KaydiriciDurumu>,
    slider1_value: f32,
    slider2: Entity<KaydiriciDurumu>,
    slider2_value: f32,
    slider3: Entity<KaydiriciDurumu>,
    slider_hsl: [Entity<KaydiriciDurumu>; 4],
    slider_hsl_value: Hsla,
    slider4: Entity<KaydiriciDurumu>,
    slider_logarithmic: Entity<KaydiriciDurumu>,
    disabled: bool,
    _subscritions: Vec<Subscription>,
}

impl super::Story for SliderStory {
    fn title() -> &'static str {
        "Kaydirici"
    }

    fn description() -> &'static str {
        "Displays a slider control for selecting a value within a range."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl SliderStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let slider1 = cx.new(|_| {
            KaydiriciDurumu::new()
                .min(-255.)
                .max(255.)
                .default_value(75.)
                .step(15.)
        });

        let slider2 = cx.new(|_| {
            KaydiriciDurumu::new()
                .min(0.)
                .max(5.)
                .step(1.0)
                .default_value(2.)
        });
        let slider_hsl = [
            cx.new(|_| {
                KaydiriciDurumu::new()
                    .min(0.)
                    .max(1.)
                    .step(0.01)
                    .default_value(0.38)
            }),
            cx.new(|_| {
                KaydiriciDurumu::new()
                    .min(0.)
                    .max(1.)
                    .step(0.01)
                    .default_value(0.5)
            }),
            cx.new(|_| {
                KaydiriciDurumu::new()
                    .min(0.)
                    .max(1.)
                    .step(0.01)
                    .default_value(0.5)
            }),
            cx.new(|_| {
                KaydiriciDurumu::new()
                    .min(0.)
                    .max(1.)
                    .step(0.01)
                    .default_value(0.5)
            }),
        ];

        let slider3 = cx.new(|_| {
            KaydiriciDurumu::new()
                .min(0.)
                .max(100.)
                .default_value(12.0..45.0)
                .step(1.)
        });

        let slider4 = cx.new(|_| {
            KaydiriciDurumu::new()
                .min(0.)
                .max(360.)
                .default_value(100.0..300.0)
                .step(1.)
        });

        let slider_logarithmic = cx.new(|_| {
            KaydiriciDurumu::new()
                .min(0.25)
                .max(4.0)
                .default_value(1.0)
                .step(0.05)
                .scale(KaydiriciOlcegi::Logarithmic)
        });

        let mut _subscritions = vec![
            cx.subscribe(
                &slider1,
                |this, _, event: &KaydiriciOlayi, cx| match event {
                    KaydiriciOlayi::Change(value) => {
                        this.slider1_value = value.start();
                        cx.notify();
                    }
                },
            ),
            cx.subscribe(
                &slider2,
                |this, _, event: &KaydiriciOlayi, cx| match event {
                    KaydiriciOlayi::Change(value) => {
                        this.slider2_value = value.start();
                        cx.notify();
                    }
                },
            ),
        ];

        _subscritions.extend(
            slider_hsl
                .iter()
                .map(|slider| {
                    cx.subscribe(slider, |this, _, event: &KaydiriciOlayi, cx| match event {
                        KaydiriciOlayi::Change(_) => {
                            this.slider_hsl_value = hsla(
                                this.slider_hsl[0].read(cx).value().start(),
                                this.slider_hsl[1].read(cx).value().start(),
                                this.slider_hsl[2].read(cx).value().start(),
                                this.slider_hsl[3].read(cx).value().start(),
                            );
                            cx.notify();
                        }
                    })
                })
                .collect::<Vec<_>>(),
        );

        slider_hsl[0].update(cx, |slider, cx| {
            cx.emit(KaydiriciOlayi::Change(slider.value()));
        });

        Self {
            focus_handle: cx.focus_handle(),
            slider1_value: 0.,
            slider2_value: 0.,
            slider1,
            slider2,
            slider3,
            slider4,
            slider_hsl,
            slider_hsl_value: kavis_ui::ham_gpui::red(),
            slider_logarithmic,
            disabled: false,
            _subscritions,
        }
    }
}

impl Focusable for SliderStory {
    fn focus_handle(&self, _: &kavis_ui::ham_gpui::App) -> kavis_ui::ham_gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SliderStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rgb = SharedString::from(self.slider_hsl_value.to_hex());

        v_flex()
            .w_full()
            .gap_3()
            .child(
                h_flex().child(
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
                section("Horizontal Kaydirici")
                    .max_w_md()
                    .v_flex()
                    .child(Kaydirici::new(&self.slider1).disabled(self.disabled))
                    .child(format!("Değer: {}", self.slider1_value)),
            )
            .child(
                section("Kaydirici (0 - 5) and with color")
                    .max_w_md()
                    .v_flex()
                    .child(
                        Kaydirici::new(&self.slider2)
                            .disabled(self.disabled)
                            .bg(cx.theme().success)
                            .text_color(cx.theme().success_foreground),
                    )
                    .child(format!("Değer: {}", self.slider2_value)),
            )
            .child(
                section("Range Mode")
                    .max_w_md()
                    .v_flex()
                    .child(Kaydirici::new(&self.slider3).disabled(self.disabled))
                    .child(format!("Değer: {}", self.slider3.read(cx).value())),
            )
            .child(
                section("Vertical with Range")
                    .max_w_md()
                    .v_flex()
                    .child(
                        Kaydirici::new(&self.slider4)
                            .vertical()
                            .h(px(200.))
                            .rounded(px(2.))
                            .disabled(self.disabled),
                    )
                    .child(format!("Değer: {}", self.slider4.read(cx).value())),
            )
            .child(
                section("Color Picker")
                    .sub_title(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                h_flex()
                                    .text_color(self.slider_hsl_value)
                                    .child(rgb.clone()),
                            )
                            .child(Pano::new("copy-hsl").value(rgb).on_copied(|_, window, cx| {
                                window.push_notification("Renk panoya kopyalandı.", cx)
                            })),
                    )
                    .max_w_md()
                    .justify_around()
                    .child(
                        v_flex()
                            .h_32()
                            .gap_3()
                            .items_center()
                            .justify_center()
                            .child(
                                Kaydirici::new(&self.slider_hsl[0])
                                    .vertical()
                                    .disabled(self.disabled),
                            )
                            .child(
                                v_flex()
                                    .items_center()
                                    .child("Ton")
                                    .child(format!("{:.0}", self.slider_hsl_value.h * 360.)),
                            ),
                    )
                    .child(
                        v_flex()
                            .h_32()
                            .gap_3()
                            .items_center()
                            .justify_center()
                            .child(
                                Kaydirici::new(&self.slider_hsl[1])
                                    .vertical()
                                    .disabled(self.disabled),
                            )
                            .child(
                                v_flex()
                                    .items_center()
                                    .child("Doygunluk")
                                    .child(format!("{:.0}", self.slider_hsl_value.s * 100.)),
                            ),
                    )
                    .child(
                        v_flex()
                            .h_32()
                            .gap_3()
                            .items_center()
                            .justify_center()
                            .child(
                                Kaydirici::new(&self.slider_hsl[2])
                                    .vertical()
                                    .disabled(self.disabled),
                            )
                            .child(
                                v_flex()
                                    .items_center()
                                    .child("Açıklık")
                                    .child(format!("{:.0}", self.slider_hsl_value.l * 100.)),
                            ),
                    )
                    .child(
                        v_flex()
                            .h_32()
                            .gap_3()
                            .items_center()
                            .justify_center()
                            .child(
                                Kaydirici::new(&self.slider_hsl[3])
                                    .vertical()
                                    .disabled(self.disabled),
                            )
                            .child(
                                v_flex()
                                    .items_center()
                                    .child("Alfa")
                                    .child(format!("{:.0}", self.slider_hsl_value.a * 100.)),
                            ),
                    ),
            )
            .child(
                section("Logarithmic Kaydirici")
                    .max_w_md()
                    .v_flex()
                    .child(
                        Kaydirici::new(&self.slider_logarithmic)
                            .horizontal()
                            .disabled(self.disabled),
                    )
                    .child(format!(
                        "Oynatma Hızı: {:.2}",
                        self.slider_logarithmic.read(cx).value().start()
                    )),
            )
    }
}
