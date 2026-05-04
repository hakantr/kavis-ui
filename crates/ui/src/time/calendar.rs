use std::rc::Rc;

use chrono::{Datelike, Local, NaiveDate};
use gpui::{
    AnyElement, App, ClickEvent, Context, Div, ElementId, Empty, Entity, EventEmitter, FocusHandle,
    InteractiveElement, IntoElement, ParentElement, Render, RenderOnce, SharedString, Stateful,
    StatefulInteractiveElement, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
    px, relative,
};

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir as _, EtkinTema, Secilebilir,
    SimgeAdi, StilUzantisi as _,
    button::{Dugme, DugmeVaryantlari as _},
    h_flex, i18n, v_flex,
};

use super::utils::days_in_month;

/// Olaylar yayılan ile takvim.
pub enum TakvimOlayi {
    /// kullanıcı seçili bir tarih.
    Selected(Date),
}

/// tarih takvim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Date {
    Single(Option<NaiveDate>),
    Range(Option<NaiveDate>, Option<NaiveDate>),
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(Some(date)) => write!(f, "{}", date),
            Self::Single(None) => write!(f, "nil"),
            Self::Range(Some(start), Some(end)) => write!(f, "{} - {}", start, end),
            Self::Range(None, None) => write!(f, "nil"),
            Self::Range(Some(start), None) => write!(f, "{} - nil", start),
            Self::Range(None, Some(end)) => write!(f, "nil - {}", end),
        }
    }
}

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Self::Single(Some(date))
    }
}

impl From<(NaiveDate, NaiveDate)> for Date {
    fn from((start, end): (NaiveDate, NaiveDate)) -> Self {
        Self::Range(Some(start), Some(end))
    }
}

impl Date {
    /// Tarihin ayarlanıp ayarlanmadığını kontrol eder.
    pub fn is_some(&self) -> bool {
        match self {
            Self::Single(Some(_)) | Self::Range(Some(_), _) => true,
            _ => false,
        }
    }

    /// Tarihin tamamlanıp tamamlanmadığını kontrol eder.
    pub fn is_complete(&self) -> bool {
        match self {
            Self::Range(Some(_), Some(_)) => true,
            Self::Single(Some(_)) => true,
            _ => false,
        }
    }

    /// başlangıç tarih döndürür.
    pub fn start(&self) -> Option<NaiveDate> {
        match self {
            Self::Single(Some(date)) => Some(*date),
            Self::Range(Some(start), _) => Some(*start),
            _ => None,
        }
    }

    /// bitiş tarih döndürür.
    pub fn end(&self) -> Option<NaiveDate> {
        match self {
            Self::Range(_, Some(end)) => Some(*end),
            _ => None,
        }
    }

    /// formatted tarih metin. döndürür.
    pub fn format(&self, format: &str) -> Option<SharedString> {
        match self {
            Self::Single(Some(date)) => Some(date.format(format).to_string().into()),
            Self::Range(Some(start), Some(end)) => {
                Some(format!("{} - {}", start.format(format), end.format(format)).into())
            }
            _ => None,
        }
    }

    /// bir localized tarih metin kullanarak ICU4X ve etkin yerel ayar. döndürür.
    pub fn format_localized(&self) -> Option<SharedString> {
        match self {
            Self::Single(Some(date)) => Some(i18n::format_date(*date)),
            Self::Range(Some(start), Some(end)) => Some(
                format!(
                    "{} - {}",
                    i18n::format_date(*start),
                    i18n::format_date(*end)
                )
                .into(),
            ),
            _ => None,
        }
    }

    fn is_active(&self, v: &NaiveDate) -> bool {
        let v = *v;
        match self {
            Self::Single(d) => Some(v) == *d,
            Self::Range(start, end) => Some(v) == *start || Some(v) == *end,
        }
    }

    fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }

    fn is_in_range(&self, v: &NaiveDate) -> bool {
        let v = *v;
        match self {
            Self::Range(start, end) => {
                if let Some(start) = start {
                    if let Some(end) = end {
                        v >= *start && v <= *end
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Day,
    Month,
    Year,
}

impl ViewMode {
    fn is_day(&self) -> bool {
        matches!(self, Self::Day)
    }

    fn is_month(&self) -> bool {
        matches!(self, Self::Month)
    }

    fn is_year(&self) -> bool {
        matches!(self, Self::Year)
    }
}

/// Esleyici için eşleşme dates önce ve sonra interval.
pub struct AralikEsleyici {
    before: Option<NaiveDate>,
    after: Option<NaiveDate>,
}

/// Esleyici için eşleşme dates içinde aralık.
pub struct KapsamEsleyici {
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
}

/// Esleyici için eşleşme dates.
pub enum Esleyici {
    /// eşleşme declare günler week.
    ///
    /// Esleyici::DayOfWeek(vec![0, 6])
    /// Haftanın Sunday ve Saturday günleriyle eşleşir.
    DayOfWeek(Vec<u32>),
    /// eşleşme included günler, except için those önce ve sonra interval.
    ///
    /// Esleyici::Interval(AralikEsleyici {
    /// önce: Some(NaiveDate::from_ymd(2020, 1, 2)),
    /// sonra: Some(NaiveDate::from_ymd(2020, 1, 3)),
    /// })
    /// 2020-01-02 ile 2020-01-03 arasında olmayan günlerle eşleşir.
    Interval(AralikEsleyici),
    /// eşleşme günler içinde aralık.
    ///
    /// Esleyici::aralık(KapsamEsleyici {
    /// den: Some(NaiveDate::from_ymd(2020, 1, 1)),
    /// için: Some(NaiveDate::from_ymd(2020, 1, 3)),
    /// })
    /// 2020-01-01 ile 2020-01-03 arasındaki günlerle eşleşir.
    Range(KapsamEsleyici),
    /// eşleşme dates kullanarak özel fonksiyon.
    ///
    /// let eşleyici = Esleyici::özel(Box::yeni(|tarih: &NaiveDate| {
    /// tarih.day0() < 5
    /// }));
    /// Her ayın ilk 5 günüyle eşleşir.
    Custom(Box<dyn Fn(&NaiveDate) -> bool + Send + Sync>),
}

impl From<Vec<u32>> for Esleyici {
    fn from(days: Vec<u32>) -> Self {
        Esleyici::DayOfWeek(days)
    }
}

impl<F> From<F> for Esleyici
where
    F: Fn(&NaiveDate) -> bool + Send + Sync + 'static,
{
    fn from(f: F) -> Self {
        Esleyici::Custom(Box::new(f))
    }
}

impl Esleyici {
    /// Yeni bir interval eşleyici oluşturur.
    pub fn interval(before: Option<NaiveDate>, after: Option<NaiveDate>) -> Self {
        Esleyici::Interval(AralikEsleyici { before, after })
    }

    /// Yeni aralık eşleyici oluşturur.
    pub fn range(from: Option<NaiveDate>, to: Option<NaiveDate>) -> Self {
        Esleyici::Range(KapsamEsleyici { from, to })
    }

    /// Yeni özel eşleyici oluşturur.
    pub fn custom<F>(f: F) -> Self
    where
        F: Fn(&NaiveDate) -> bool + Send + Sync + 'static,
    {
        Esleyici::Custom(Box::new(f))
    }

    /// tarih matches eşleyici. olup olmadığını kontrol eder.
    pub fn is_match(&self, date: &Date) -> bool {
        match date {
            Date::Single(Some(date)) => self.matched(date),
            Date::Range(Some(start), Some(end)) => self.matched(start) || self.matched(end),
            _ => false,
        }
    }

    fn matched(&self, date: &NaiveDate) -> bool {
        match self {
            Esleyici::DayOfWeek(days) => days.contains(&date.weekday().num_days_from_sunday()),
            Esleyici::Interval(interval) => {
                let before_check = interval.before.map_or(false, |before| date < &before);
                let after_check = interval.after.map_or(false, |after| date > &after);
                before_check || after_check
            }
            Esleyici::Range(range) => {
                let from_check = range.from.map_or(false, |from| date < &from);
                let to_check = range.to.map_or(false, |to| date > &to);
                !from_check && !to_check
            }
            Esleyici::Custom(f) => f(date),
        }
    }
}

#[derive(IntoElement)]
pub struct Takvim {
    id: ElementId,
    size: BilesenBoyutu,
    state: Entity<TakvimDurumu>,
    style: StyleRefinement,
    /// Görünümde gösterilecek ay sayısı.
    number_of_months: usize,
}

/// Kullanım için store durum takvim.
pub struct TakvimDurumu {
    focus_handle: FocusHandle,
    view_mode: ViewMode,
    active_month_offset: usize,
    date: Date,
    current_year: i32,
    current_month: u8,
    years: Vec<Vec<i32>>,
    year_page: i32,
    today: NaiveDate,
    /// Görünümde gösterilecek ay sayısı.
    number_of_months: usize,
    pub(crate) disabled_matcher: Option<Rc<Esleyici>>,
}

fn offset_year_month(year: i32, month: u32, offset_month: i32) -> (i32, u32) {
    let month_index = year * 12 + month as i32 - 1 + offset_month;

    (
        month_index.div_euclid(12),
        month_index.rem_euclid(12) as u32 + 1,
    )
}

fn year_chunks_contain(years: &[Vec<i32>], year: i32) -> bool {
    years.iter().any(|years| years.contains(&year))
}

impl TakvimDurumu {
    /// Yeni bir takvim durum oluşturur.
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        let today = Local::now().naive_local().date();
        Self {
            focus_handle: cx.focus_handle(),
            view_mode: ViewMode::Day,
            active_month_offset: 0,
            date: Date::Single(None),
            current_month: today.month() as u8,
            current_year: today.year(),
            years: vec![],
            year_page: 0,
            today,
            number_of_months: 1,
            disabled_matcher: None,
        }
        .year_range((today.year() - 50, today.year() + 50))
    }

    /// devre dışı eşleyici takvim durum ayarlar.
    pub fn disabled_matcher(mut self, matcher: impl Into<Esleyici>) -> Self {
        self.disabled_matcher = Some(Rc::new(matcher.into()));
        self
    }

    /// devre dışı eşleyici takvim ayarlar.
    ///
    /// Devre dışı eşleyici, eşleşen günleri devre dışı bırakmak için kullanılır.
    pub fn set_disabled_matcher(
        &mut self,
        disabled: impl Into<Esleyici>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
        self.disabled_matcher = Some(Rc::new(disabled.into()));
    }

    /// tarih takvim ayarlar.
    ///
    /// Aralık tarihi ayarladığınızda mod otomatik olarak `Mode::aralık` değerine ayarlanır.
    pub fn set_date(&mut self, date: impl Into<Date>, _: &mut Window, cx: &mut Context<Self>) {
        let date = date.into();

        let invalid = self
            .disabled_matcher
            .as_ref()
            .map_or(false, |matcher| matcher.is_match(&date));

        if invalid {
            return;
        }

        self.date = date;
        match self.date {
            Date::Single(Some(date)) => {
                self.current_month = date.month() as u8;
                self.current_year = date.year();
            }
            Date::Range(Some(start), _) => {
                self.current_month = start.month() as u8;
                self.current_year = start.year();
            }
            _ => {}
        }

        cx.notify()
    }

    /// tarih takvim döndürür.
    pub fn date(&self) -> Date {
        self.date
    }

    /// true olduğunda bir tarih olabilir represented ile configured takvim yıl aralık. döndürür.
    pub fn can_show_date(&self, date: NaiveDate) -> bool {
        year_chunks_contain(&self.years, date.year())
    }

    /// Gösterilecek ay sayısını ayarlar.
    pub fn set_number_of_months(
        &mut self,
        number_of_months: usize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.number_of_months = number_of_months;
        cx.notify();
    }

    /// Yıl aralığını ayarlar. Varsayılan, geçerli yıldan 50 yıl önce ve sonrasıdır.
    ///
    /// Her yıl sayfası 20 yıl içerir; bu yüzden aralığı 20 yıllık parçalara bölmek daha uygundur.
    pub fn year_range(mut self, range: (i32, i32)) -> Self {
        self.apply_year_range(range);
        self
    }

    /// yıl aralık takvim ayarlar.
    pub fn set_year_range(&mut self, range: (i32, i32), cx: &mut Context<Self>) {
        self.apply_year_range(range);
        cx.notify();
    }

    fn apply_year_range(&mut self, range: (i32, i32)) {
        self.years = (range.0..range.1)
            .collect::<Vec<_>>()
            .chunks(20)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();
        self.year_page = self
            .years
            .iter()
            .position(|years| years.contains(&self.current_year))
            .unwrap_or(0) as i32;
    }

    /// yıl ve month ile ofset month. döndürür.
    fn offset_year_month(&self, offset_month: usize) -> (i32, u32) {
        offset_year_month(
            self.current_year,
            self.current_month as u32,
            offset_month as i32,
        )
    }

    fn active_month_offset(&self) -> usize {
        self.active_month_offset
            .min(self.number_of_months.saturating_sub(1))
    }

    fn active_year_month(&self) -> (i32, u32) {
        self.offset_year_month(self.active_month_offset())
    }

    fn set_active_month_offset(&mut self, offset_month: usize) {
        self.active_month_offset = offset_month.min(self.number_of_months.saturating_sub(1));
        let (year, _) = self.active_year_month();
        self.set_year_page_for_year(year);
    }

    fn set_year_page_for_year(&mut self, year: i32) {
        self.year_page = self
            .years
            .iter()
            .position(|years| years.contains(&year))
            .unwrap_or(0) as i32;
    }

    fn set_year_month_for_offset(&mut self, year: i32, month: u32, offset_month: usize) {
        let (year, month) = offset_year_month(year, month, -(offset_month as i32));
        self.current_year = year;
        self.current_month = month as u8;
    }

    /// günler month içinde bir 2D vector çizmek için üzerinde takvim döndürür.
    fn days(&self) -> Vec<Vec<NaiveDate>> {
        let first_weekday = i18n::first_weekday();

        (0..self.number_of_months)
            .flat_map(|offset| {
                days_in_month(
                    self.current_year,
                    self.current_month as u32 + offset as u32,
                    first_weekday,
                )
            })
            .collect()
    }

    fn has_prev_year_page(&self) -> bool {
        self.year_page > 0
    }

    fn has_next_year_page(&self) -> bool {
        self.year_page < self.years.len() as i32 - 1
    }

    fn prev_year_page(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if !self.has_prev_year_page() {
            return;
        }

        self.year_page -= 1;
        cx.notify()
    }

    fn next_year_page(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if !self.has_next_year_page() {
            return;
        }

        self.year_page += 1;
        cx.notify()
    }

    fn prev_month(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.current_month = if self.current_month == 1 {
            12
        } else {
            self.current_month - 1
        };
        self.current_year = if self.current_month == 12 {
            self.current_year - 1
        } else {
            self.current_year
        };
        cx.notify()
    }

    fn next_month(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.current_month = if self.current_month == 12 {
            1
        } else {
            self.current_month + 1
        };
        self.current_year = if self.current_month == 1 {
            self.current_year + 1
        } else {
            self.current_year
        };
        cx.notify()
    }

    fn month_name(&self, offset_month: usize) -> SharedString {
        let (_, month) = self.offset_year_month(offset_month);
        i18n::month_name(month)
    }

    fn year_name(&self, offset_month: usize) -> SharedString {
        let (year, _) = self.offset_year_month(offset_month);
        year.to_string().into()
    }

    fn set_view_mode(&mut self, mode: ViewMode, _: &mut Window, cx: &mut Context<Self>) {
        self.set_active_month_offset(0);
        self.view_mode = mode;
        cx.notify();
    }

    fn set_view_mode_for_month(
        &mut self,
        mode: ViewMode,
        offset_month: usize,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_active_month_offset(offset_month);
        self.view_mode = mode;
        cx.notify();
    }

    fn select_month(&mut self, month: u32, _: &mut Window, cx: &mut Context<Self>) {
        let offset_month = self.active_month_offset();
        let (year, _) = self.active_year_month();
        self.set_year_month_for_offset(year, month, offset_month);
        self.view_mode = ViewMode::Day;
        cx.notify();
    }

    fn select_year(&mut self, year: i32, _: &mut Window, cx: &mut Context<Self>) {
        let offset_month = self.active_month_offset();
        let (_, month) = self.active_year_month();
        self.set_year_month_for_offset(year, month, offset_month);
        self.view_mode = ViewMode::Day;
        cx.notify();
    }

    fn months(&self) -> Vec<SharedString> {
        i18n::month_names()
    }
}

impl Render for TakvimDurumu {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

impl Takvim {
    /// Yeni bir takvim öğe ile [`TakvimDurumu`] oluşturur.
    pub fn new(state: &Entity<TakvimDurumu>) -> Self {
        Self {
            id: ("calendar", state.entity_id()).into(),
            size: BilesenBoyutu::default(),
            state: state.clone(),
            style: StyleRefinement::default(),
            number_of_months: 1,
        }
    }

    /// Gösterilecek ay sayısını ayarlar. Varsayılan 1dir.
    pub fn number_of_months(mut self, number_of_months: usize) -> Self {
        self.number_of_months = number_of_months;
        self
    }

    fn render_day(
        &self,
        d: &NaiveDate,
        offset_month: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> Stateful<Div> {
        let state = self.state.read(cx);
        let (_, month) = state.offset_year_month(offset_month);
        let day = d.day();
        let is_current_month = d.month() == month;
        let is_active = state.date.is_active(d);
        let is_in_range = state.date.is_in_range(d);

        let date = *d;
        let is_today = *d == state.today;
        let disabled = state
            .disabled_matcher
            .as_ref()
            .map_or(false, |disabled| disabled.matched(&date));

        let date_id: SharedString = format!("{}_{}", date.format("%Y-%m-%d"), offset_month).into();

        self.item_button(
            date_id.clone(),
            day.to_string(),
            is_active,
            is_in_range,
            !is_current_month || disabled,
            disabled,
            window,
            cx,
        )
        .when(is_today && !is_active, |this| {
            this.border_1().border_color(cx.theme().border)
        }) // Add border for today
        .when(!disabled, |this| {
            this.on_click(window.listener_for(
                &self.state,
                move |view, _: &ClickEvent, window, cx| {
                    if view.date.is_single() {
                        view.set_date(date, window, cx);
                        cx.emit(TakvimOlayi::Selected(view.date()));
                    } else {
                        let start = view.date.start();
                        let end = view.date.end();

                        if start.is_none() && end.is_none() {
                            view.set_date(Date::Range(Some(date), None), window, cx);
                        } else if start.is_some() && end.is_none() {
                            if date < start.unwrap() {
                                view.set_date(Date::Range(Some(date), None), window, cx);
                            } else {
                                view.set_date(
                                    Date::Range(Some(start.unwrap()), Some(date)),
                                    window,
                                    cx,
                                );
                            }
                        } else {
                            view.set_date(Date::Range(Some(date), None), window, cx);
                        }

                        if view.date.is_complete() {
                            cx.emit(TakvimOlayi::Selected(view.date()));
                        }
                    }
                },
            ))
        })
    }

    fn render_header(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);
        let view_mode = state.view_mode;
        let disabled = view_mode.is_month();
        let multiple_months = self.number_of_months > 1;
        let icon_size = match self.size {
            BilesenBoyutu::Kucuk => BilesenBoyutu::Kucuk,
            BilesenBoyutu::Buyuk => BilesenBoyutu::Orta,
            _ => BilesenBoyutu::Orta,
        };

        h_flex()
            .gap_0p5()
            .justify_between()
            .items_center()
            .child(
                Dugme::new("prev")
                    .icon(SimgeAdi::ArrowLeft)
                    .tab_stop(false)
                    .ghost()
                    .disabled(disabled)
                    .with_size(icon_size)
                    .when(view_mode.is_day(), |this| {
                        this.on_click(window.listener_for(&self.state, TakvimDurumu::prev_month))
                    })
                    .when(view_mode.is_year(), |this| {
                        this.when(!state.has_prev_year_page(), |this| this.disabled(true))
                            .on_click(
                                window.listener_for(&self.state, TakvimDurumu::prev_year_page),
                            )
                    }),
            )
            .when(!multiple_months, |this| {
                this.child(
                    h_flex()
                        .justify_center()
                        .gap_3()
                        .child(
                            Dugme::new("month")
                                .ghost()
                                .label(state.month_name(0))
                                .compact()
                                .tab_stop(false)
                                .with_size(self.size)
                                .selected(view_mode.is_month())
                                .on_click(window.listener_for(
                                    &self.state,
                                    move |view, _, window, cx| {
                                        if view_mode.is_month() {
                                            view.set_view_mode(ViewMode::Day, window, cx);
                                        } else {
                                            view.set_view_mode(ViewMode::Month, window, cx);
                                        }
                                        cx.notify();
                                    },
                                )),
                        )
                        .child(
                            Dugme::new("year")
                                .ghost()
                                .label(state.year_name(0))
                                .compact()
                                .tab_stop(false)
                                .with_size(self.size)
                                .selected(view_mode.is_year())
                                .on_click(window.listener_for(
                                    &self.state,
                                    |view, _, window, cx| {
                                        if view.view_mode.is_year() {
                                            view.set_view_mode(ViewMode::Day, window, cx);
                                        } else {
                                            view.set_view_mode(ViewMode::Year, window, cx);
                                        }
                                        cx.notify();
                                    },
                                )),
                        ),
                )
            })
            .when(multiple_months, |this| {
                this.child(h_flex().flex_1().justify_around().children(
                    (0..self.number_of_months).map(|n| {
                        let is_active_month = state.active_month_offset() == n;

                        h_flex()
                            .justify_center()
                            .map(|this| match self.size {
                                BilesenBoyutu::Kucuk => this.gap_2(),
                                BilesenBoyutu::Buyuk => this.gap_4(),
                                _ => this.gap_3(),
                            })
                            .child(
                                Dugme::new(("month", n))
                                    .ghost()
                                    .label(state.month_name(n))
                                    .compact()
                                    .tab_stop(false)
                                    .with_size(self.size)
                                    .selected(view_mode.is_month() && is_active_month)
                                    .on_click(window.listener_for(
                                        &self.state,
                                        move |view, _, window, cx| {
                                            if view.view_mode.is_month()
                                                && view.active_month_offset() == n
                                            {
                                                view.set_view_mode_for_month(
                                                    ViewMode::Day,
                                                    n,
                                                    window,
                                                    cx,
                                                );
                                            } else {
                                                view.set_view_mode_for_month(
                                                    ViewMode::Month,
                                                    n,
                                                    window,
                                                    cx,
                                                );
                                            }
                                        },
                                    )),
                            )
                            .child(
                                Dugme::new(("year", n))
                                    .ghost()
                                    .label(state.year_name(n))
                                    .compact()
                                    .tab_stop(false)
                                    .with_size(self.size)
                                    .selected(view_mode.is_year() && is_active_month)
                                    .on_click(window.listener_for(
                                        &self.state,
                                        move |view, _, window, cx| {
                                            if view.view_mode.is_year()
                                                && view.active_month_offset() == n
                                            {
                                                view.set_view_mode_for_month(
                                                    ViewMode::Day,
                                                    n,
                                                    window,
                                                    cx,
                                                );
                                            } else {
                                                view.set_view_mode_for_month(
                                                    ViewMode::Year,
                                                    n,
                                                    window,
                                                    cx,
                                                );
                                            }
                                        },
                                    )),
                            )
                    }),
                ))
            })
            .child(
                Dugme::new("next")
                    .icon(SimgeAdi::ArrowRight)
                    .ghost()
                    .tab_stop(false)
                    .disabled(disabled)
                    .with_size(icon_size)
                    .when(view_mode.is_day(), |this| {
                        this.on_click(window.listener_for(&self.state, TakvimDurumu::next_month))
                    })
                    .when(view_mode.is_year(), |this| {
                        this.when(!state.has_next_year_page(), |this| this.disabled(true))
                            .on_click(
                                window.listener_for(&self.state, TakvimDurumu::next_year_page),
                            )
                    }),
            )
    }

    #[allow(clippy::too_many_arguments)]
    fn item_button(
        &self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        active: bool,
        secondary_active: bool,
        muted: bool,
        disabled: bool,
        _: &mut Window,
        cx: &mut App,
    ) -> Stateful<Div> {
        h_flex()
            .id(id.into())
            .map(|this| match self.size {
                BilesenBoyutu::Kucuk => this.size_7().rounded(cx.theme().radius / 2.),
                BilesenBoyutu::Buyuk => this.size_10().rounded(cx.theme().radius * 2.),
                _ => this.size_9().rounded(cx.theme().radius),
            })
            .justify_center()
            .when(muted, |this| {
                this.text_color(if disabled {
                    cx.theme().muted_foreground.opacity(0.3)
                } else {
                    cx.theme().muted_foreground
                })
            })
            .when(secondary_active, |this| {
                this.bg(if muted {
                    cx.theme().accent.opacity(0.5)
                } else {
                    cx.theme().accent
                })
                .text_color(cx.theme().accent_foreground)
            })
            .when(!active && !disabled, |this| {
                this.hover(|this| {
                    this.bg(cx.theme().accent)
                        .text_color(cx.theme().accent_foreground)
                })
            })
            .when(active, |this| {
                this.bg(cx.theme().primary)
                    .text_color(cx.theme().primary_foreground)
            })
            .child(label.into())
    }

    fn render_days(&self, window: &mut Window, cx: &mut App) -> Div {
        let state = self.state.read(cx);
        let weeks = i18n::weekday_names();

        h_flex()
            .map(|this| match self.size {
                BilesenBoyutu::Kucuk => this.gap_3().text_sm(),
                BilesenBoyutu::Buyuk => this.gap_5().text_base(),
                _ => this.gap_4().text_sm(),
            })
            .justify_between()
            .children(
                state
                    .days()
                    .chunks(5)
                    .enumerate()
                    .map(|(offset_month, days)| {
                        v_flex()
                            .gap_0p5()
                            .child(
                                h_flex().gap_0p5().justify_between().children(
                                    weeks
                                        .iter()
                                        .map(|week| self.render_week(week.clone(), window, cx)),
                                ),
                            )
                            .children(days.iter().map(|week| {
                                h_flex().gap_0p5().justify_between().children(
                                    week.iter()
                                        .map(|d| self.render_day(d, offset_month, window, cx)),
                                )
                            }))
                    }),
            )
    }

    fn render_week(&self, week: impl Into<SharedString>, _: &mut Window, cx: &mut App) -> Div {
        h_flex()
            .map(|this| match self.size {
                BilesenBoyutu::Kucuk => this.size_7().rounded(cx.theme().radius / 2.0),
                BilesenBoyutu::Buyuk => this.size_10().rounded(cx.theme().radius),
                _ => this.size_9().rounded(cx.theme().radius),
            })
            .justify_center()
            .text_color(cx.theme().muted_foreground)
            .text_sm()
            .child(week.into())
    }

    fn render_selection_columns<F>(&self, active_child: F, window: &mut Window, cx: &mut App) -> Div
    where
        F: Fn(&Self, &mut Window, &mut App) -> AnyElement,
    {
        let active_offset = self.state.read(cx).active_month_offset();
        let mut children = Vec::with_capacity(self.number_of_months);

        for offset in 0..self.number_of_months {
            if offset == active_offset {
                children.push(active_child(self, window, cx));
            } else {
                children.push(self.render_selection_placeholder(cx).into_any_element());
            }
        }

        h_flex()
            .map(|this| match self.size {
                BilesenBoyutu::Kucuk => this.gap_3(),
                BilesenBoyutu::Buyuk => this.gap_5(),
                _ => this.gap_4(),
            })
            .justify_between()
            .children(children)
    }

    fn render_selection_placeholder(&self, _: &mut App) -> Div {
        v_flex()
            .map(|this| match self.size {
                BilesenBoyutu::Kucuk => this.w(px(208.)),
                BilesenBoyutu::Buyuk => this.w(px(292.)),
                _ => this.w(px(264.)),
            })
            .invisible()
            .child(" ")
    }

    fn render_selection_body<F>(&self, active_child: F, window: &mut Window, cx: &mut App) -> Div
    where
        F: Fn(&Self, &mut Window, &mut App) -> AnyElement,
    {
        div()
            .relative()
            .child(self.render_days(window, cx).invisible())
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .child(self.render_selection_columns(active_child, window, cx)),
            )
    }

    fn render_months(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);
        let months = state.months();
        let (_, active_month) = state.active_year_month();

        h_flex()
            .mt_3()
            .gap_0p5()
            .gap_y_3()
            .map(|this| match self.size {
                BilesenBoyutu::Kucuk => this.mt_2().gap_y_2().w(px(208.)),
                BilesenBoyutu::Buyuk => this.mt_4().gap_y_4().w(px(292.)),
                _ => this.mt_3().gap_y_3().w(px(264.)),
            })
            .justify_between()
            .flex_wrap()
            .children(
                months
                    .iter()
                    .enumerate()
                    .map(|(ix, month)| {
                        let month_number = (ix + 1) as u32;
                        let active = month_number == active_month;

                        self.item_button(
                            ix,
                            month.to_string(),
                            active,
                            false,
                            false,
                            false,
                            window,
                            cx,
                        )
                        .w(relative(0.3))
                        .text_sm()
                        .on_click(window.listener_for(
                            &self.state,
                            move |view, _, window, cx| {
                                view.select_month(month_number, window, cx);
                            },
                        ))
                    })
                    .collect::<Vec<_>>(),
            )
    }

    fn render_years(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.read(cx);
        let (active_year, _) = state.active_year_month();
        let current_page_years = &self.state.read(cx).years[state.year_page as usize].clone();

        h_flex()
            .id("years")
            .gap_0p5()
            .map(|this| match self.size {
                BilesenBoyutu::Kucuk => this.mt_2().gap_y_2().w(px(208.)),
                BilesenBoyutu::Buyuk => this.mt_4().gap_y_4().w(px(292.)),
                _ => this.mt_3().gap_y_3().w(px(264.)),
            })
            .justify_between()
            .flex_wrap()
            .children(
                current_page_years
                    .iter()
                    .enumerate()
                    .map(|(ix, year)| {
                        let year = *year;
                        let active = year == active_year;

                        self.item_button(
                            ix,
                            year.to_string(),
                            active,
                            false,
                            false,
                            false,
                            window,
                            cx,
                        )
                        .w(relative(0.18))
                        .on_click(window.listener_for(
                            &self.state,
                            move |view, _, window, cx| {
                                view.select_year(year, window, cx);
                            },
                        ))
                    })
                    .collect::<Vec<_>>(),
            )
    }
}

impl Boyutlandirilabilir for Takvim {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for Takvim {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl EventEmitter<TakvimOlayi> for TakvimDurumu {}
impl RenderOnce for Takvim {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let view_mode = self.state.read(cx).view_mode;
        let number_of_months = self.number_of_months;
        self.state.update(cx, |state, _| {
            state.number_of_months = number_of_months;
        });

        v_flex()
            .id(self.id.clone())
            .track_focus(&self.state.read(cx).focus_handle)
            .border_1()
            .border_color(cx.theme().border)
            .rounded(cx.theme().radius_lg)
            .p_3()
            .gap_0p5()
            .refine_style(&self.style)
            .child(self.render_header(window, cx))
            .child(
                v_flex()
                    .when(view_mode.is_day(), |this| {
                        this.child(self.render_days(window, cx))
                    })
                    .when(view_mode.is_month(), |this| {
                        if number_of_months > 1 {
                            this.child(self.render_selection_body(
                                |this, window, cx| {
                                    this.render_months(window, cx).into_any_element()
                                },
                                window,
                                cx,
                            ))
                        } else {
                            this.child(self.render_months(window, cx))
                        }
                    })
                    .when(view_mode.is_year(), |this| {
                        if number_of_months > 1 {
                            this.child(self.render_selection_body(
                                |this, window, cx| this.render_years(window, cx).into_any_element(),
                                window,
                                cx,
                            ))
                        } else {
                            this.child(self.render_years(window, cx))
                        }
                    }),
            )
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::{Date, offset_year_month, year_chunks_contain};

    #[test]
    fn test_date_to_string() {
        let date = Date::Single(Some(NaiveDate::from_ymd_opt(2024, 8, 3).unwrap()));
        assert_eq!(date.to_string(), "2024-08-03");

        let date = Date::Single(None);
        assert_eq!(date.to_string(), "nil");

        let date = Date::Range(
            Some(NaiveDate::from_ymd_opt(2024, 8, 3).unwrap()),
            Some(NaiveDate::from_ymd_opt(2024, 8, 5).unwrap()),
        );
        assert_eq!(date.to_string(), "2024-08-03 - 2024-08-05");

        let date = Date::Range(Some(NaiveDate::from_ymd_opt(2024, 8, 3).unwrap()), None);
        assert_eq!(date.to_string(), "2024-08-03 - nil");

        let date = Date::Range(None, Some(NaiveDate::from_ymd_opt(2024, 8, 5).unwrap()));
        assert_eq!(date.to_string(), "nil - 2024-08-05");

        let date = Date::Range(None, None);
        assert_eq!(date.to_string(), "nil");
    }

    #[test]
    fn test_offset_year_month() {
        assert_eq!(offset_year_month(2024, 1, 1), (2024, 2));
        assert_eq!(offset_year_month(2024, 12, 1), (2025, 1));
        assert_eq!(offset_year_month(2025, 1, -1), (2024, 12));
        assert_eq!(offset_year_month(2025, 3, -2), (2025, 1));
    }

    #[test]
    fn test_can_show_date_uses_year_range() {
        let years = (2020..2023)
            .collect::<Vec<_>>()
            .chunks(20)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();

        assert!(year_chunks_contain(&years, 2020));
        assert!(year_chunks_contain(&years, 2022));
        assert!(!year_chunks_contain(&years, 2023));
        assert!(!year_chunks_contain(&years, 2019));
    }
}
