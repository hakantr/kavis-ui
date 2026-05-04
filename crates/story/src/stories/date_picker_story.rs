use chrono::{Datelike, Days, Duration, Utc};
use gpui::{
    App, AppContext, Context, Entity, Focusable, IntoElement, ParentElement as _, Render,
    Styled as _, Subscription, Window, div, px,
};
use kavis_ui::{
    Boyutlandirilabilir as _, EtkinTema as _, calendar,
    date_input::{TarihGirdisi, TarihGirdisiDurumu, TarihGirdisiOlayi},
    date_picker::{TarihAraligiOnAyari, TarihSecici, TarihSeciciDurumu, TarihSeciciOlayi},
    v_flex,
};

use crate::section;

pub struct DatePickerStory {
    date_picker: Entity<TarihSeciciDurumu>,
    date_picker_small: Entity<TarihSeciciDurumu>,
    date_picker_large: Entity<TarihSeciciDurumu>,
    data_picker_custom: Entity<TarihSeciciDurumu>,
    date_picker_value: Option<String>,
    date_range_picker: Entity<TarihSeciciDurumu>,
    default_range_mode_picker: Entity<TarihSeciciDurumu>,
    birthday_picker: Entity<TarihSeciciDurumu>,
    without_appearance_picker: Entity<TarihSeciciDurumu>,
    date_input: Entity<TarihGirdisiDurumu>,
    date_input_with_short_month_name: Entity<TarihGirdisiDurumu>,
    date_input_with_month_name: Entity<TarihGirdisiDurumu>,
    date_input_value: Option<String>,
    _subscriptions: Vec<Subscription>,
}

impl super::Story for DatePickerStory {
    fn title() -> &'static str {
        "TarihSecici"
    }

    fn description() -> &'static str {
        "A date picker to select a date or date range."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl DatePickerStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let now = chrono::Local::now().naive_local().date();
        let date_picker = cx.new(|cx| {
            let mut picker = TarihSeciciDurumu::new(window, cx).disabled_matcher(vec![0, 6]);
            picker.set_date(now, window, cx);
            picker
        });
        let date_picker_large = cx.new(|cx| {
            let mut picker = TarihSeciciDurumu::new(window, cx)
                .date_format("%Y-%m-%d")
                .disabled_matcher(calendar::Esleyici::range(
                    Some(now),
                    now.checked_add_days(Days::new(7)),
                ));
            picker.set_date(
                now.checked_sub_days(Days::new(1)).unwrap_or_default(),
                window,
                cx,
            );
            picker
        });
        let date_picker_small = cx.new(|cx| {
            let mut picker = TarihSeciciDurumu::new(window, cx).disabled_matcher(
                calendar::Esleyici::interval(Some(now), now.checked_add_days(Days::new(5))),
            );
            picker.set_date(now, window, cx);
            picker
        });
        let data_picker_custom = cx.new(|cx| {
            let mut picker = TarihSeciciDurumu::new(window, cx)
                .disabled_matcher(calendar::Esleyici::custom(|date| date.day0() < 5));
            picker.set_date(now, window, cx);
            picker
        });
        let date_range_picker = cx.new(|cx| {
            let mut picker = TarihSeciciDurumu::new(window, cx);
            picker.set_date(
                (now, now.checked_add_days(Days::new(4)).unwrap()),
                window,
                cx,
            );
            picker
        });

        let default_range_mode_picker = cx.new(|cx| TarihSeciciDurumu::range(window, cx));

        let birthday_picker = cx.new(|cx| {
            let mut picker = TarihSeciciDurumu::new(window, cx);
            picker.set_year_range((1927, now.year() + 1), cx);
            picker
        });

        let without_appearance_picker = cx.new(|cx| TarihSeciciDurumu::new(window, cx));
        let date_input = cx.new(|cx| TarihGirdisiDurumu::new(window, cx).date_format("gg/aa/yyyy"));
        let date_input_with_short_month_name =
            cx.new(|cx| TarihGirdisiDurumu::new(window, cx).date_format("gg-AAA-yyyy"));
        let date_input_with_month_name =
            cx.new(|cx| TarihGirdisiDurumu::new(window, cx).date_format("gg-aaaa-yyyy"));

        let _subscriptions = vec![
            cx.subscribe(&date_picker, |this, _, ev, _| match ev {
                TarihSeciciOlayi::Change(date) => {
                    this.date_picker_value = date.format("%Y-%m-%d").map(|s| s.to_string());
                }
            }),
            cx.subscribe(&date_range_picker, |this, _, ev, _| match ev {
                TarihSeciciOlayi::Change(date) => {
                    this.date_picker_value = date.format("%Y-%m-%d").map(|s| s.to_string());
                }
            }),
            cx.subscribe(&default_range_mode_picker, |this, _, ev, _| match ev {
                TarihSeciciOlayi::Change(date) => {
                    this.date_picker_value = date.format("%Y-%m-%d").map(|s| s.to_string());
                }
            }),
            cx.subscribe(&date_input, |this, _, ev, _| match ev {
                TarihGirdisiOlayi::Change(date) => {
                    this.date_input_value = (*date).map(|date| date.to_string());
                }
            }),
            cx.subscribe(
                &date_input_with_short_month_name,
                |this, _, ev, _| match ev {
                    TarihGirdisiOlayi::Change(date) => {
                        this.date_input_value = (*date).map(|date| date.to_string());
                    }
                },
            ),
            cx.subscribe(&date_input_with_month_name, |this, _, ev, _| match ev {
                TarihGirdisiOlayi::Change(date) => {
                    this.date_input_value = (*date).map(|date| date.to_string());
                }
            }),
        ];

        Self {
            date_picker,
            date_picker_large,
            date_picker_small,
            data_picker_custom,
            date_range_picker,
            default_range_mode_picker,
            birthday_picker,
            without_appearance_picker,
            date_input,
            date_input_with_short_month_name,
            date_input_with_month_name,
            date_input_value: None,
            date_picker_value: None,
            _subscriptions,
        }
    }
}

impl Focusable for DatePickerStory {
    fn focus_handle(&self, cx: &gpui::App) -> gpui::FocusHandle {
        self.date_picker.focus_handle(cx)
    }
}

impl Render for DatePickerStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let presets = vec![
            TarihAraligiOnAyari::single(
                "Yesterday",
                (Utc::now() - Duration::days(1)).naive_local().date(),
            ),
            TarihAraligiOnAyari::single(
                "Last Week",
                (Utc::now() - Duration::weeks(1)).naive_local().date(),
            ),
            TarihAraligiOnAyari::single(
                "Last Month",
                (Utc::now() - Duration::days(30)).naive_local().date(),
            ),
        ];
        let range_presets = vec![
            TarihAraligiOnAyari::range(
                "Last 7 Days",
                (Utc::now() - Duration::days(7)).naive_local().date(),
                Utc::now().naive_local().date(),
            ),
            TarihAraligiOnAyari::range(
                "Last 14 Days",
                (Utc::now() - Duration::days(14)).naive_local().date(),
                Utc::now().naive_local().date(),
            ),
            TarihAraligiOnAyari::range(
                "Last 30 Days",
                (Utc::now() - Duration::days(30)).naive_local().date(),
                Utc::now().naive_local().date(),
            ),
            TarihAraligiOnAyari::range(
                "Last 90 Days",
                (Utc::now() - Duration::days(90)).naive_local().date(),
                Utc::now().naive_local().date(),
            ),
        ];

        v_flex()
            .gap_3()
            .child(
                section("Normal").max_w_128().child(
                    TarihSecici::new(&self.date_picker)
                        .cleanable(true)
                        .presets(presets),
                ),
            )
            .child(
                section("Small with 180px width").max_w_128().child(
                    TarihSecici::new(&self.date_picker_small)
                        .small()
                        .w(px(180.)),
                ),
            )
            .child(
                section("Large").max_w_128().child(
                    TarihSecici::new(&self.date_picker_large)
                        .large()
                        .w(px(300.)),
                ),
            )
            .child(
                section("Custom (First 5 days of each month disabled)")
                    .max_w_128()
                    .child(TarihSecici::new(&self.data_picker_custom)),
            )
            .child(
                section("Date Range").max_w_128().child(
                    TarihSecici::new(&self.date_range_picker)
                        .number_of_months(2)
                        .cleanable(true)
                        .presets(range_presets.clone()),
                ),
            )
            .child(
                section("Default Range Mode").max_w_128().child(
                    TarihSecici::new(&self.default_range_mode_picker)
                        .placeholder("Aralık modu seçici")
                        .cleanable(true)
                        .presets(range_presets.clone()),
                ),
            )
            .child(
                section("Date Picker Value").max_w_128().child(
                    format!("Tarih seçici değeri: {:?}", self.date_picker_value).into_element(),
                ),
            )
            .child(
                section("Date Input")
                    .max_w_128()
                    .child(TarihGirdisi::new(&self.date_input).cleanable(true))
                    .child(
                        TarihGirdisi::new(&self.date_input_with_short_month_name).cleanable(true),
                    )
                    .child(TarihGirdisi::new(&self.date_input_with_month_name).cleanable(true))
                    .child(format!("Tarih girişi değeri: {:?}", self.date_input_value)),
            )
            .child(
                section("Custom Year Range (birthday, 1900 to current)")
                    .max_w_128()
                    .child(
                        TarihSecici::new(&self.birthday_picker)
                            .number_of_months(1)
                            .cleanable(true)
                            .placeholder("Doğum gününü seç"),
                    ),
            )
            .child(
                section("Without Appearance").max_w_128().child(
                    div().w_full().bg(cx.theme().secondary).child(
                        TarihSecici::new(&self.without_appearance_picker)
                            .appearance(false)
                            .placeholder("Görünüm olmadan"),
                    ),
                ),
            )
    }
}
