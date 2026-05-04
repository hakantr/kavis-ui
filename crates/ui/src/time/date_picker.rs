use std::rc::Rc;

use crate::ham_gpui::{
    App, AppContext, ClickEvent, Context, ElementId, Empty, Entity, EventEmitter, FocusHandle,
    Focusable, InteractiveElement as _, IntoElement, KeyBinding, MouseButton, ParentElement as _,
    Render, RenderOnce, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Subscription, Window, anchored, deferred, div, prelude::FluentBuilder as _, px,
};
use chrono::NaiveDate;
use rust_i18n::t;

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, EtkinTema, Simge, SimgeAdi,
    StilBoyutlandirma as _, StilUzantisi as _,
    actions::{Cancel, Confirm},
    button::{Dugme, DugmeVaryantlari as _},
    h_flex,
    input::{Delete, clear_button, input_style},
    v_flex,
};

use super::calendar::{Date, Esleyici, Takvim, TakvimDurumu, TakvimOlayi};

const CONTEXT: &'static str = "TarihSecici";
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("enter", Confirm { secondary: false }, Some(CONTEXT)),
        KeyBinding::new("escape", Cancel, Some(CONTEXT)),
        KeyBinding::new("delete", Delete, Some(CONTEXT)),
        KeyBinding::new("backspace", Delete, Some(CONTEXT)),
    ])
}

/// Olaylar yayılan ile TarihSecici.
#[derive(Clone)]
pub enum TarihSeciciOlayi {
    Change(Date),
}

/// Preset değer için TarihAraligiOnAyari.
#[derive(Clone)]
pub enum TarihAraligiOnAyarDegeri {
    Single(NaiveDate),
    Range(NaiveDate, NaiveDate),
}

/// Preset için tarih aralık seçim.
#[derive(Clone)]
pub struct TarihAraligiOnAyari {
    label: SharedString,
    value: TarihAraligiOnAyarDegeri,
}

impl TarihAraligiOnAyari {
    /// Yeni bir TarihAraligiOnAyari ile bir tarih oluşturur.
    pub fn single(label: impl Into<SharedString>, date: NaiveDate) -> Self {
        TarihAraligiOnAyari {
            label: label.into(),
            value: TarihAraligiOnAyarDegeri::Single(date),
        }
    }
    /// Yeni bir TarihAraligiOnAyari ile aralık dates oluşturur.
    pub fn range(label: impl Into<SharedString>, start: NaiveDate, end: NaiveDate) -> Self {
        TarihAraligiOnAyari {
            label: label.into(),
            value: TarihAraligiOnAyarDegeri::Range(start, end),
        }
    }
}

/// Kullanım için store durum tarih seçici.
pub struct TarihSeciciDurumu {
    focus_handle: FocusHandle,
    date: Date,
    open: bool,
    calendar: Entity<TakvimDurumu>,
    date_format: Option<SharedString>,
    number_of_months: usize,
    disabled_matcher: Option<Rc<Esleyici>>,
    _subscriptions: Vec<Subscription>,
}

impl Focusable for TarihSeciciDurumu {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl EventEmitter<TarihSeciciOlayi> for TarihSeciciDurumu {}

impl TarihSeciciDurumu {
    /// Bir tarih durum. oluşturur.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::new_with_range(false, window, cx)
    }

    /// Bir tarih durum ile aralık mod. oluşturur.
    pub fn range(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::new_with_range(true, window, cx)
    }

    fn new_with_range(is_range: bool, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let date = if is_range {
            Date::Range(None, None)
        } else {
            Date::Single(None)
        };

        let calendar = cx.new(|cx| {
            let mut this = TakvimDurumu::new(window, cx);
            this.set_date(date, window, cx);
            this
        });

        let _subscriptions = vec![cx.subscribe_in(
            &calendar,
            window,
            |this, _, ev: &TakvimOlayi, window, cx| match ev {
                TakvimOlayi::Selected(date) => {
                    this.update_date(*date, true, window, cx);
                    this.focus_handle.focus(window, cx);
                }
            },
        )];

        Self {
            focus_handle: cx.focus_handle(),
            date,
            calendar,
            open: false,
            date_format: None,
            number_of_months: 1,
            disabled_matcher: None,
            _subscriptions,
        }
    }

    /// tarih biçim tarih seçici göstermek için içinde girdi ayarlar.
    ///
    /// Bu ayarlanmazsa etkin yerel ayarın varsayılan ICU4X tarih biçimi kullanılır.
    pub fn date_format(mut self, format: impl Into<SharedString>) -> Self {
        self.date_format = Some(format.into());
        self
    }

    /// Takvim görünümünde gösterilecek ay sayısını ayarlar. Varsayılan 1dir.
    pub fn number_of_months(mut self, number_of_months: usize) -> Self {
        self.number_of_months = number_of_months;
        self
    }

    /// tarih tarih seçici döndürür.
    pub fn date(&self) -> Date {
        self.date
    }

    /// tarih tarih seçici ayarlar.
    pub fn set_date(&mut self, date: impl Into<Date>, window: &mut Window, cx: &mut Context<Self>) {
        self.update_date(date.into(), false, window, cx);
    }

    /// devre dışı eşleşme için takvim ayarlar.
    pub fn disabled_matcher(mut self, disabled: impl Into<Esleyici>) -> Self {
        self.disabled_matcher = Some(Rc::new(disabled.into()));
        self
    }

    /// yıl aralık için dahili takvim ayarlar.
    ///
    /// Varsayılan, geçerli yıldan 50 yıl önce ve sonrasıdır.
    /// `aralık`, `end` değerinin hariç olduğu yarı açık `(start, end)` aralığını kullanır.
    pub fn set_year_range(&mut self, range: (i32, i32), cx: &mut Context<Self>) {
        self.calendar.update(cx, |state, cx| {
            state.set_year_range(range, cx);
        });
    }

    fn update_date(&mut self, date: Date, emit: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.date = date;
        self.calendar.update(cx, |view, cx| {
            view.set_date(date, window, cx);
        });
        self.open = false;
        if emit {
            cx.emit(TarihSeciciOlayi::Change(date));
        }
        cx.notify();
    }

    /// devre dışı eşleyici tarih seçici ayarlar.
    fn set_canlendar_disabled_matcher(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        let matcher = self.disabled_matcher.clone();
        self.calendar.update(cx, |state, _| {
            state.disabled_matcher = matcher;
        });
    }

    fn on_escape(&mut self, _: &Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            cx.propagate();
        }

        self.focus_back_if_need(window, cx);
        self.open = false;

        cx.notify();
    }

    fn on_enter(&mut self, _: &Confirm, _: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            self.open = true;
            cx.notify();
        }
    }

    fn on_delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        self.clean(&ClickEvent::default(), window, cx);
    }

    // To focus the Picker Input, if current focus in is on the container.
    //
    // This is because mouse down out the Takvim, GPUI will move focus to the container.
    // So we need to move focus back to the Picker Input.
    //
    // But if mouse down target is some other focusable element (e.g.: [`crate::Input`]), we should not move focus.
    fn focus_back_if_need(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.open {
            return;
        }

        if let Some(focused) = window.focused(cx) {
            if focused.contains(&self.focus_handle, window) {
                self.focus_handle.focus(window, cx);
            }
        }
    }

    fn clean(
        &mut self,
        _: &crate::ham_gpui::ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.stop_propagation();
        match self.date {
            Date::Single(_) => {
                self.update_date(Date::Single(None), true, window, cx);
            }
            Date::Range(_, _) => {
                self.update_date(Date::Range(None, None), true, window, cx);
            }
        }
    }

    fn toggle_calendar(
        &mut self,
        _: &crate::ham_gpui::ClickEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open = !self.open;
        cx.notify();
    }

    fn select_preset(
        &mut self,
        preset: &TarihAraligiOnAyari,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match preset.value {
            TarihAraligiOnAyarDegeri::Single(single) => {
                self.update_date(Date::Single(Some(single)), true, window, cx)
            }
            TarihAraligiOnAyarDegeri::Range(start, end) => {
                self.update_date(Date::Range(Some(start), Some(end)), true, window, cx)
            }
        }
    }
}

/// Bir TarihSecici öğe.
#[derive(IntoElement)]
pub struct TarihSecici {
    id: ElementId,
    style: StyleRefinement,
    state: Entity<TarihSeciciDurumu>,
    cleanable: bool,
    placeholder: Option<SharedString>,
    size: BilesenBoyutu,
    number_of_months: usize,
    presets: Option<Vec<TarihAraligiOnAyari>>,
    appearance: bool,
    disabled: bool,
}

impl Boyutlandirilabilir for TarihSecici {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}
impl Focusable for TarihSecici {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.state.focus_handle(cx)
    }
}

impl Styled for TarihSecici {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl DevreDisiBirakilabilir for TarihSecici {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for TarihSeciciDurumu {
    fn render(
        &mut self,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> impl crate::ham_gpui::IntoElement {
        Empty
    }
}

impl TarihSecici {
    /// Yeni bir TarihSecici ile verilen [`TarihSeciciDurumu`] oluşturur.
    pub fn new(state: &Entity<TarihSeciciDurumu>) -> Self {
        Self {
            id: ("date-picker", state.entity_id()).into(),
            state: state.clone(),
            cleanable: false,
            placeholder: None,
            size: BilesenBoyutu::default(),
            style: StyleRefinement::default(),
            number_of_months: 2,
            presets: None,
            appearance: true,
            disabled: false,
        }
    }

    /// yer tutucu tarih seçici, varsayılan: "" ayarlar.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Girdi alanı boş değilken temizleme düğmesinin gösterilip gösterilmeyeceğini ayarlar. Varsayılan false.
    pub fn cleanable(mut self, cleanable: bool) -> Self {
        self.cleanable = cleanable;
        self
    }

    /// preset aralıklar için tarih seçici ayarlar.
    pub fn presets(mut self, presets: Vec<TarihAraligiOnAyari>) -> Self {
        self.presets = Some(presets);
        self
    }

    /// Takvimde gösterilecek ay sayısını ayarlar. Varsayılan 2dir.
    pub fn number_of_months(mut self, number_of_months: usize) -> Self {
        self.number_of_months = number_of_months;
        self
    }

    /// Tarih seçicinin görünümünü ayarlar; false ise tarih seçici minimal stilde olur.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }
}

impl RenderOnce for TarihSecici {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |state, cx| {
            state.set_canlendar_disabled_matcher(window, cx);
        });

        // This for keep focus border style, when click on the popup.
        let is_focused = self.focus_handle(cx).contains_focused(window, cx);
        let state = self.state.read(cx);
        let show_clean = self.cleanable && state.date.is_some();
        let placeholder = self
            .placeholder
            .clone()
            .unwrap_or_else(|| t!("TarihSecici.placeholder").into());
        let display_title = state
            .date_format
            .as_ref()
            .and_then(|format| state.date.format(format))
            .or_else(|| state.date.format_localized())
            .unwrap_or(placeholder.clone());

        let (bg, fg) = input_style(self.disabled, cx);

        div()
            .id(self.id.clone())
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle(cx).tab_stop(true))
            .on_action(window.listener_for(&self.state, TarihSeciciDurumu::on_enter))
            .on_action(window.listener_for(&self.state, TarihSeciciDurumu::on_delete))
            .when(state.open, |this| {
                this.on_action(window.listener_for(&self.state, TarihSeciciDurumu::on_escape))
            })
            .flex_none()
            .w_full()
            .relative()
            .input_text_size(self.size)
            .refine_style(&self.style)
            .child(
                div()
                    .id("date-picker-input")
                    .relative()
                    .flex()
                    .items_center()
                    .justify_between()
                    .when(self.appearance, |this| {
                        this.bg(bg)
                            .text_color(fg)
                            .when(self.disabled, |this| this.opacity(0.5))
                            .border_1()
                            .border_color(cx.theme().input)
                            .rounded(cx.theme().radius)
                            .when(cx.theme().shadow, |this| this.shadow_xs())
                            .when(is_focused, |this| this.focused_border(cx))
                    })
                    .overflow_hidden()
                    .input_text_size(self.size)
                    .input_size(self.size)
                    .when(!state.open && !self.disabled, |this| {
                        this.on_click(
                            window.listener_for(&self.state, TarihSeciciDurumu::toggle_calendar),
                        )
                    })
                    .child(
                        h_flex()
                            .w_full()
                            .items_center()
                            .justify_between()
                            .gap_1()
                            .child(
                                div()
                                    .w_full()
                                    .overflow_hidden()
                                    .when(!state.date.is_some(), |this| {
                                        this.text_color(cx.theme().muted_foreground)
                                    })
                                    .child(display_title),
                            )
                            .when(!self.disabled, |this| {
                                this.when(show_clean, |this| {
                                    this.child(clear_button(cx).on_click(
                                        window.listener_for(&self.state, TarihSeciciDurumu::clean),
                                    ))
                                })
                                .when(!show_clean, |this| {
                                    this.child(
                                        Simge::new(SimgeAdi::Calendar)
                                            .xsmall()
                                            .text_color(cx.theme().muted_foreground),
                                    )
                                })
                            }),
                    ),
            )
            .when(state.open, |this| {
                this.child(
                    deferred(
                        anchored().snap_to_window_with_margin(px(8.)).child(
                            div()
                                .occlude()
                                .mt_1p5()
                                .p_3()
                                .border_1()
                                .border_color(cx.theme().border)
                                .shadow_lg()
                                .rounded((cx.theme().radius * 2.).min(px(8.)))
                                .bg(cx.theme().popover)
                                .text_color(cx.theme().popover_foreground)
                                .on_mouse_up_out(
                                    MouseButton::Left,
                                    window.listener_for(&self.state, |view, _, window, cx| {
                                        view.on_escape(&Cancel, window, cx);
                                    }),
                                )
                                .child(
                                    h_flex()
                                        .gap_3()
                                        .h_full()
                                        .items_start()
                                        .when_some(self.presets.clone(), |this, presets| {
                                            this.child(
                                                v_flex().my_1().gap_2().justify_end().children(
                                                    presets.into_iter().enumerate().map(
                                                        |(i, preset)| {
                                                            Dugme::new(("preset", i))
                                                                .small()
                                                                .ghost()
                                                                .tab_stop(false)
                                                                .label(preset.label.clone())
                                                                .on_click(window.listener_for(
                                                                    &self.state,
                                                                    move |this, _, window, cx| {
                                                                        this.select_preset(
                                                                            &preset, window, cx,
                                                                        );
                                                                    },
                                                                ))
                                                        },
                                                    ),
                                                ),
                                            )
                                        })
                                        .child(
                                            Takvim::new(&state.calendar)
                                                .number_of_months(self.number_of_months)
                                                .border_0()
                                                .rounded_none()
                                                .p_0()
                                                .with_size(self.size),
                                        ),
                                ),
                        ),
                    )
                    .with_priority(2),
                )
            })
    }
}
