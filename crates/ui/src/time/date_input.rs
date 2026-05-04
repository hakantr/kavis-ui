use std::ops::Range;

use crate::ham_gpui::{
    AnyElement, App, AppContext, AvailableSpace, Bounds, ClickEvent, Context, Element, ElementId,
    Entity, EventEmitter, FocusHandle, Focusable, GlobalElementId, InteractiveElement, IntoElement,
    KeyDownEvent, LayoutId, MouseButton, MouseUpEvent, ParentElement as _, Pixels, Render,
    RenderOnce, SharedString, StatefulInteractiveElement as _, StyleRefinement, Styled,
    Subscription, Window, anchored, deferred, div, point, prelude::FluentBuilder as _, px,
};
use chrono::{Datelike, NaiveDate};

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, EtkinTema as _, SimgeAdi,
    StilBoyutlandirma, StilUzantisi as _,
    button::{Dugme, DugmeVaryantlari as _},
    i18n,
    input::{Input, InputEvent, InputState, MaskPattern},
};

use super::calendar::{Date, Takvim, TakvimDurumu, TakvimOlayi};

/// Olaylar yayılan ile [`TarihGirdisi`].
#[derive(Clone)]
pub enum TarihGirdisiOlayi {
    Change(Option<NaiveDate>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum DateInputPart {
    Day,
    MonthNumber,
    MonthNameShort,
    MonthName,
    YearFull,
    YearShort,
    Literal(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedDatePart {
    value: u32,
    range: Range<usize>,
    complete: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedYearPart {
    value: i32,
    range: Range<usize>,
    complete: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MonthNameInputContext {
    range: Range<usize>,
    cursor: usize,
    day: u32,
    following_literal: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MonthNameSuggestion {
    month: u32,
    name: SharedString,
}

/// Tarih girdi biçim kullanarak localized tarih tokens.
///
/// Supported tokens:
/// - `gg`: day month, için örnek `24`
/// - `aa`: numeric month, için örnek `05`
/// - `AAA`: localized short month ad, için örnek `Oca`
/// - `aaaa`: localized month ad, için örnek `Mayıs`
/// - `yyyy`: four digit yıl, için örnek `2013`
/// - `yy`: iki digit yıl, için örnek `13`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TarihGirdisiFormati {
    pattern: SharedString,
    parts: Vec<DateInputPart>,
}

impl Default for TarihGirdisiFormati {
    fn default() -> Self {
        Self::localized()
    }
}

impl From<&str> for TarihGirdisiFormati {
    fn from(pattern: &str) -> Self {
        Self::new(pattern)
    }
}

impl From<String> for TarihGirdisiFormati {
    fn from(pattern: String) -> Self {
        Self::new(pattern)
    }
}

impl From<SharedString> for TarihGirdisiFormati {
    fn from(pattern: SharedString) -> Self {
        Self::new(pattern)
    }
}

impl TarihGirdisiFormati {
    /// Bir tarih girdi biçim bir explicit belirteç desen. oluşturur.
    pub fn new(pattern: impl Into<SharedString>) -> Self {
        let pattern = pattern.into();
        let parts = Self::parse_parts(pattern.as_str());

        Self { pattern, parts }
    }

    /// Etkin yerel ayarı kullanarak ICU4X üzerinden tarih girdi biçimi oluşturur.
    pub fn localized() -> Self {
        Self::new(i18n::date_input_format())
    }

    pub fn pattern(&self) -> &str {
        self.pattern.as_str()
    }

    pub fn format(&self, date: NaiveDate) -> SharedString {
        let mut value = String::new();

        for part in &self.parts {
            match part {
                DateInputPart::Day => value.push_str(&format!("{:02}", date.day())),
                DateInputPart::MonthNumber => value.push_str(&format!("{:02}", date.month())),
                DateInputPart::MonthNameShort => {
                    value.push_str(i18n::month_short_name(date.month()).as_str())
                }
                DateInputPart::MonthName => value.push_str(i18n::month_name(date.month()).as_str()),
                DateInputPart::YearFull => value.push_str(&format!("{:04}", date.year())),
                DateInputPart::YearShort => {
                    value.push_str(&format!("{:02}", date.year().rem_euclid(100)))
                }
                DateInputPart::Literal(literal) => value.push_str(literal),
            }
        }

        value.into()
    }

    pub fn parse(&self, value: &str) -> Option<NaiveDate> {
        let value = value.trim();
        if value.is_empty() {
            return None;
        }

        let mut cursor = 0;
        let mut day = None;
        let mut month = None;
        let mut year = None;

        for part in &self.parts {
            match part {
                DateInputPart::Day => {
                    let (parsed, next_cursor) = parse_digits(value, cursor, 1, 2)?;
                    day = Some(parsed);
                    cursor = next_cursor;
                }
                DateInputPart::MonthNumber => {
                    let (parsed, next_cursor) = parse_digits(value, cursor, 1, 2)?;
                    month = Some(parsed);
                    cursor = next_cursor;
                }
                DateInputPart::MonthNameShort => {
                    let (parsed, next_cursor) =
                        parse_month_name(value, cursor, i18n::month_short_names())?;
                    month = Some(parsed);
                    cursor = next_cursor;
                }
                DateInputPart::MonthName => {
                    let (parsed, next_cursor) =
                        parse_month_name(value, cursor, i18n::month_names())?;
                    month = Some(parsed);
                    cursor = next_cursor;
                }
                DateInputPart::YearFull => {
                    let (parsed, next_cursor) = parse_digits(value, cursor, 4, 4)?;
                    year = Some(parsed as i32);
                    cursor = next_cursor;
                }
                DateInputPart::YearShort => {
                    let (parsed, next_cursor) = parse_digits(value, cursor, 2, 2)?;
                    year = Some(parse_two_digit_year(parsed));
                    cursor = next_cursor;
                }
                DateInputPart::Literal(literal) => {
                    cursor = consume_literal(value, cursor, literal)?;
                }
            }
        }

        if !value[cursor..].trim().is_empty() {
            return None;
        }

        NaiveDate::from_ymd_opt(year?, month?, day?)
    }

    fn first_validation_error(&self, value: &str) -> Option<Range<usize>> {
        let value = value.trim();
        if value.is_empty() {
            return None;
        }

        let mut cursor = 0;
        let mut day: Option<ParsedDatePart> = None;
        let mut month: Option<ParsedDatePart> = None;
        let mut year: Option<ParsedYearPart> = None;

        for part in &self.parts {
            if cursor >= value.len() {
                break;
            }

            match part {
                DateInputPart::Day => {
                    if let Some(parsed) = parse_partial_digits(value, cursor, 2) {
                        cursor = parsed.range.end;
                        day = Some(parsed);
                    }
                }
                DateInputPart::MonthNumber => {
                    if let Some(parsed) = parse_partial_digits(value, cursor, 2) {
                        cursor = parsed.range.end;
                        month = Some(parsed);
                    }
                }
                DateInputPart::MonthNameShort => {
                    if let Some(parsed) =
                        parse_partial_month_name(value, cursor, 3, i18n::month_short_names())
                    {
                        cursor = parsed.range.end;
                        month = Some(parsed);
                    }
                }
                DateInputPart::MonthName => {
                    if let Some(parsed) =
                        parse_partial_variable_month_name(value, cursor, i18n::month_names())
                    {
                        cursor = parsed.range.end;
                        month = Some(parsed);
                    }
                }
                DateInputPart::YearFull => {
                    if let Some(parsed) = parse_partial_digits(value, cursor, 4) {
                        cursor = parsed.range.end;
                        year = Some(ParsedYearPart {
                            value: parsed.value as i32,
                            range: parsed.range,
                            complete: parsed.complete,
                        });
                    }
                }
                DateInputPart::YearShort => {
                    if let Some(parsed) = parse_partial_digits(value, cursor, 2) {
                        cursor = parsed.range.end;
                        year = Some(ParsedYearPart {
                            value: parse_two_digit_year(parsed.value),
                            range: parsed.range,
                            complete: parsed.complete,
                        });
                    }
                }
                DateInputPart::Literal(literal) => {
                    if value[cursor..].starts_with(literal) {
                        cursor += literal.len();
                    } else if literal.starts_with(&value[cursor..]) {
                        break;
                    } else {
                        return Some(cursor..value.len());
                    }
                }
            }
        }

        if let Some(day) = &day
            && day.value > 31
        {
            return Some(day.range.clone());
        }

        if let Some(month) = &month
            && month.complete
            && !(1..=12).contains(&month.value)
        {
            return Some(month.range.clone());
        }

        let (Some(day), Some(month)) = (day, month) else {
            return None;
        };

        if !month.complete || !(1..=12).contains(&month.value) {
            return None;
        }

        let complete_year = year.as_ref().filter(|year| year.complete);
        let max_day = if month.value == 2 && complete_year.is_none() {
            29
        } else {
            let year = complete_year.map_or(2000, |year| year.value);
            days_in_month_for_validation(year, month.value)
        };

        if day.value > max_day {
            if month.value == 2
                && day.value == 29
                && let Some(year) = complete_year
            {
                return Some(join_ranges(
                    &join_ranges(&day.range, &month.range),
                    &year.range,
                ));
            }
            return Some(join_ranges(&day.range, &month.range));
        }

        if let Some(year) = complete_year
            && NaiveDate::from_ymd_opt(year.value, month.value, day.value).is_none()
        {
            return Some(join_ranges(
                &join_ranges(&day.range, &month.range),
                &year.range,
            ));
        }

        None
    }

    fn mask_pattern(&self) -> Option<String> {
        let mut pattern = String::new();

        for part in &self.parts {
            match part {
                DateInputPart::Day | DateInputPart::MonthNumber => pattern.push_str("##"),
                DateInputPart::MonthNameShort => pattern.push_str("AAA"),
                DateInputPart::YearFull => pattern.push_str("####"),
                DateInputPart::YearShort => pattern.push_str("##"),
                DateInputPart::Literal(literal) => pattern.push_str(literal),
                DateInputPart::MonthName => return None,
            }
        }

        Some(pattern)
    }

    fn complete_numeric_part_with_separator(
        &self,
        value: &str,
        cursor: usize,
        separator: char,
    ) -> Option<String> {
        if cursor != value.len() || !value.is_char_boundary(cursor) {
            return None;
        }

        let mut value_cursor = 0;

        for (part_ix, part) in self.parts.iter().enumerate() {
            match part {
                DateInputPart::Day | DateInputPart::MonthNumber | DateInputPart::YearShort => {
                    let part_start = value_cursor;
                    let consumed = consume_digits_up_to(value, &mut value_cursor, 2);

                    if value_cursor == cursor {
                        let literal = self.separator_literal_after(part_ix, separator)?;
                        if consumed == 1 {
                            let mut completed =
                                String::with_capacity(value.len() + literal.len() + 1);
                            completed.push_str(&value[..part_start]);
                            completed.push('0');
                            completed.push_str(&value[part_start..cursor]);
                            completed.push_str(literal);
                            return Some(completed);
                        }

                        return None;
                    }
                }
                DateInputPart::YearFull => {
                    consume_digits_up_to(value, &mut value_cursor, 4);

                    if value_cursor == cursor {
                        return None;
                    }
                }
                DateInputPart::MonthNameShort => {
                    consume_letters_up_to(value, &mut value_cursor, 3);

                    if value_cursor == cursor {
                        return None;
                    }
                }
                DateInputPart::MonthName => return None,
                DateInputPart::Literal(literal) => {
                    if value[value_cursor..].starts_with(literal) {
                        value_cursor += literal.len();
                    } else {
                        return None;
                    }
                }
            }
        }

        None
    }

    fn separator_literal_after(&self, part_ix: usize, separator: char) -> Option<&str> {
        let Some(DateInputPart::Literal(literal)) = self.parts.get(part_ix + 1) else {
            return None;
        };

        let mut chars = literal.chars();
        (chars.next() == Some(separator) && chars.next().is_none()).then_some(literal.as_str())
    }

    fn month_name_input_context(
        &self,
        value: &str,
        cursor: usize,
    ) -> Option<MonthNameInputContext> {
        if !value.is_char_boundary(cursor) {
            return None;
        }

        let mut value_cursor = 0;
        let mut day = None;

        for (part_ix, part) in self.parts.iter().enumerate() {
            match part {
                DateInputPart::Day => {
                    let parsed = parse_partial_digits(value, value_cursor, 2)?;
                    if parsed.value == 0 || parsed.value > 31 {
                        return None;
                    }
                    value_cursor = parsed.range.end;
                    day = Some(parsed.value);
                }
                DateInputPart::MonthName => {
                    let day = day?;
                    let start = value_cursor;
                    let end = consume_letters_end(value, start);
                    if cursor < start || cursor > end {
                        return None;
                    }

                    return Some(MonthNameInputContext {
                        range: start..end,
                        cursor,
                        day,
                        following_literal: self.literal_after_part(part_ix).map(ToOwned::to_owned),
                    });
                }
                DateInputPart::Literal(literal) => {
                    if value[value_cursor..].starts_with(literal) {
                        value_cursor += literal.len();
                    } else {
                        return None;
                    }
                }
                DateInputPart::MonthNumber
                | DateInputPart::MonthNameShort
                | DateInputPart::YearFull
                | DateInputPart::YearShort => {
                    return None;
                }
            }
        }

        None
    }

    fn month_name_suggestions(
        &self,
        value: &str,
        cursor: usize,
    ) -> Option<(MonthNameInputContext, Vec<MonthNameSuggestion>)> {
        let context = self.month_name_input_context(value, cursor)?;
        let query = normalize_month_name(&value[context.range.start..context.cursor]);

        let suggestions = i18n::month_names()
            .into_iter()
            .enumerate()
            .filter_map(|(ix, name)| {
                let month = (ix + 1) as u32;
                if !month_accepts_day(month, context.day) {
                    return None;
                }

                if !query.is_empty() && !normalize_month_name(name.as_str()).starts_with(&query) {
                    return None;
                }

                Some(MonthNameSuggestion { month, name })
            })
            .collect::<Vec<_>>();

        Some((context, suggestions))
    }

    fn literal_after_part(&self, part_ix: usize) -> Option<&str> {
        let Some(DateInputPart::Literal(literal)) = self.parts.get(part_ix + 1) else {
            return None;
        };

        Some(literal.as_str())
    }

    fn placeholder(&self) -> SharedString {
        if let Some(pattern) = self.mask_pattern() {
            if let Some(placeholder) = MaskPattern::new(&pattern).placeholder() {
                return placeholder.into();
            }
        }

        self.format(NaiveDate::from_ymd_opt(2013, 5, 24).unwrap())
    }

    fn apply_to_input_state(&self, state: InputState) -> InputState {
        if let Some(pattern) = self.mask_pattern() {
            state.mask_pattern(pattern.as_str())
        } else {
            state
                .mask_pattern(MaskPattern::None)
                .placeholder(self.placeholder())
        }
    }

    fn set_on_input_state(
        &self,
        state: &mut InputState,
        window: &mut Window,
        cx: &mut Context<InputState>,
    ) {
        if let Some(pattern) = self.mask_pattern() {
            state.set_mask_pattern(pattern.as_str(), window, cx);
        } else {
            state.set_mask_pattern(MaskPattern::None, window, cx);
            state.set_placeholder(self.placeholder(), window, cx);
        }
    }

    fn parse_parts(pattern: &str) -> Vec<DateInputPart> {
        let mut parts = Vec::new();
        let mut literal = String::new();
        let chars = pattern.chars().collect::<Vec<_>>();
        let mut ix = 0;

        while ix < chars.len() {
            if starts_with(&chars, ix, "yyyy") {
                push_literal(&mut parts, &mut literal);
                parts.push(DateInputPart::YearFull);
                ix += 4;
            } else if starts_with(&chars, ix, "aaaa") {
                push_literal(&mut parts, &mut literal);
                parts.push(DateInputPart::MonthName);
                ix += 4;
            } else if starts_with(&chars, ix, "AAA") {
                push_literal(&mut parts, &mut literal);
                parts.push(DateInputPart::MonthNameShort);
                ix += 3;
            } else if starts_with(&chars, ix, "gg") {
                push_literal(&mut parts, &mut literal);
                parts.push(DateInputPart::Day);
                ix += 2;
            } else if starts_with(&chars, ix, "aa") {
                push_literal(&mut parts, &mut literal);
                parts.push(DateInputPart::MonthNumber);
                ix += 2;
            } else if starts_with(&chars, ix, "yy") {
                push_literal(&mut parts, &mut literal);
                parts.push(DateInputPart::YearShort);
                ix += 2;
            } else {
                literal.push(chars[ix]);
                ix += 1;
            }
        }

        push_literal(&mut parts, &mut literal);
        parts
    }
}

/// durum için bir [`TarihGirdisi`].
pub struct TarihGirdisiDurumu {
    input: Entity<InputState>,
    calendar: Entity<TakvimDurumu>,
    date: Option<NaiveDate>,
    format: TarihGirdisiFormati,
    placeholder: Option<SharedString>,
    open: bool,
    needs_input_sync: bool,
    _subscriptions: Vec<Subscription>,
}

impl TarihGirdisiDurumu {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let format = TarihGirdisiFormati::localized();
        let input = cx.new(|cx| format.apply_to_input_state(InputState::new(window, cx)));
        let calendar = cx.new(|cx| TakvimDurumu::new(window, cx));
        let _subscriptions = vec![
            cx.subscribe_in(
                &input,
                window,
                |this, state, event: &InputEvent, window, cx| match event {
                    InputEvent::Change => {
                        let value = state.read(cx).value();
                        this.update_date_from_text(value.as_str(), true, window, cx);
                    }
                    InputEvent::Blur => {
                        let value = state.read(cx).value();
                        this.clean_invalid_text(value.as_str(), window, cx);
                    }
                    _ => {}
                },
            ),
            cx.subscribe_in(
                &calendar,
                window,
                |this, _, event: &TakvimOlayi, window, cx| match event {
                    TakvimOlayi::Selected(Date::Single(Some(date))) => {
                        this.update_date(Some(*date), true, window, cx);
                        this.open = false;
                    }
                    _ => {}
                },
            ),
        ];

        Self {
            input,
            calendar,
            date: None,
            format,
            placeholder: None,
            open: false,
            needs_input_sync: false,
            _subscriptions,
        }
    }

    /// tarih girdi biçim ayarlar.
    pub fn date_format(mut self, format: impl Into<TarihGirdisiFormati>) -> Self {
        self.format = format.into();
        self.needs_input_sync = true;
        self
    }

    /// varsayılan tarih değer ayarlar.
    pub fn default_value(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self.needs_input_sync = true;
        self
    }

    /// ayrıştırılmış tarih değer döndürür.
    pub fn date(&self) -> Option<NaiveDate> {
        self.date
    }

    /// geçerli ham metin değer döndürür.
    pub fn raw_value(&self, cx: &App) -> SharedString {
        self.input.read(cx).value()
    }

    /// Access dahili girdi durum.
    pub fn input(&self) -> Entity<InputState> {
        self.input.clone()
    }

    /// Access dahili takvim durum.
    pub fn calendar(&self) -> Entity<TakvimDurumu> {
        self.calendar.clone()
    }

    /// geçerli tarih değer ayarlar.
    pub fn set_date(
        &mut self,
        date: Option<NaiveDate>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_date(date, false, window, cx);
    }

    /// tarih girdi biçim. günceller.
    pub fn set_date_format(
        &mut self,
        format: impl Into<TarihGirdisiFormati>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.format = format.into();
        self.needs_input_sync = true;
        self.sync_input(window, cx);
    }

    /// yıl aralık için dahili takvim ayarlar.
    ///
    /// `aralık`, `end` değerinin hariç olduğu yarı açık `(start, end)` aralığını kullanır.
    pub fn set_year_range(&mut self, range: (i32, i32), cx: &mut Context<Self>) {
        self.calendar.update(cx, |state, cx| {
            state.set_year_range(range, cx);
        });
    }

    fn sync_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.needs_input_sync {
            return;
        }

        self.needs_input_sync = false;
        let format = self.format.clone();
        let value = self
            .date
            .map(|date| format.format(date))
            .unwrap_or_default();

        self.input.update(cx, |state, cx| {
            format.set_on_input_state(state, window, cx);
            if let Some(placeholder) = self.placeholder.clone() {
                state.set_placeholder(placeholder, window, cx);
            }
            state.set_value(value, window, cx);
            state.set_error_ranges(Vec::new(), cx);
        });
        self.sync_calendar_date(self.date, window, cx);
    }

    fn set_placeholder(
        &mut self,
        placeholder: Option<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.placeholder == placeholder {
            return;
        }

        self.placeholder = placeholder;
        let placeholder = self
            .placeholder
            .clone()
            .unwrap_or_else(|| self.format.placeholder());
        self.input.update(cx, |state, cx| {
            state.set_placeholder(placeholder, window, cx);
        });
    }

    fn update_date(
        &mut self,
        date: Option<NaiveDate>,
        emit: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.date = date;
        let value = date
            .map(|date| self.format.format(date))
            .unwrap_or_default();
        self.input.update(cx, |state, cx| {
            state.set_value(value, window, cx);
            state.set_error_ranges(Vec::new(), cx);
        });
        self.sync_calendar_date(date, window, cx);

        if emit {
            cx.emit(TarihGirdisiOlayi::Change(date));
        }
        cx.notify();
    }

    fn update_date_from_text(
        &mut self,
        value: &str,
        emit: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let validation_error = self.format.first_validation_error(value);
        self.set_error_range(validation_error.clone(), cx);

        let date = if value.trim().is_empty() || validation_error.is_some() {
            None
        } else {
            self.format.parse(value)
        };

        if self.date == date {
            return;
        }

        self.date = date;
        self.sync_calendar_date(date, window, cx);
        if emit {
            cx.emit(TarihGirdisiOlayi::Change(date));
        }
        cx.notify();
    }

    fn set_error_range(&mut self, range: Option<Range<usize>>, cx: &mut Context<Self>) {
        let ranges = range.into_iter().collect::<Vec<_>>();
        self.input.update(cx, |state, cx| {
            state.set_error_ranges(ranges, cx);
        });
    }

    fn clean_invalid_text(&mut self, value: &str, window: &mut Window, cx: &mut Context<Self>) {
        if self.format.first_validation_error(value).is_some() {
            self.open = false;
            self.update_date(None, true, window, cx);
        }
    }

    fn sync_calendar_date(
        &mut self,
        date: Option<NaiveDate>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(date) = date else {
            return;
        };

        if self.calendar.read(cx).can_show_date(date) {
            self.calendar.update(cx, |state, cx| {
                state.set_date(Date::Single(Some(date)), window, cx);
            });
        }
    }

    fn should_show_calendar_icon(&self, cx: &App) -> bool {
        self.date
            .map(|date| self.calendar.read(cx).can_show_date(date))
            .unwrap_or(true)
    }

    fn month_name_suggestions(
        &self,
        cx: &App,
    ) -> Option<(MonthNameInputContext, Vec<MonthNameSuggestion>)> {
        let input = self.input.read(cx);
        let value = input.value();
        let cursor = input.cursor();
        let selected_range = input.selected_range();

        if !selected_range.is_empty() {
            return None;
        }

        let (context, suggestions) = self.format.month_name_suggestions(value.as_str(), cursor)?;
        (!suggestions.is_empty()).then_some((context, suggestions))
    }

    fn toggle_calendar(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(date) = self.date {
            if !self.calendar.read(cx).can_show_date(date) {
                return;
            }
            self.sync_calendar_date(Some(date), window, cx);
        }

        self.open = !self.open;
        cx.notify();
    }

    fn close_calendar(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.open = false;
        cx.notify();
    }

    fn select_month_suggestion(&mut self, month: u32, window: &mut Window, cx: &mut Context<Self>) {
        let (value, cursor) = {
            let input = self.input.read(cx);
            (input.value(), input.cursor())
        };
        let Some(context) = self.format.month_name_input_context(value.as_str(), cursor) else {
            return;
        };

        let mut range = context.range.clone();
        let mut replacement = i18n::month_name(month).to_string();
        if let Some(literal) = context.following_literal.as_deref() {
            replacement.push_str(literal);
            if value[context.range.end..].starts_with(literal) {
                range.end += literal.len();
            }
        }

        self.input.update(cx, |state, cx| {
            let range_utf16 = byte_range_to_utf16(value.as_str(), &range);
            state.replace_text_in_range_silent(Some(range_utf16), replacement.as_str(), window, cx);
            state.focus(window, cx);
        });
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if event.keystroke.modifiers.control
            || event.keystroke.modifiers.alt
            || event.keystroke.modifiers.platform
            || event.keystroke.modifiers.function
        {
            return;
        }

        let (value, cursor, selected_range) = {
            let input = self.input.read(cx);
            (input.value(), input.cursor(), input.selected_range())
        };

        if self.format.first_validation_error(value.as_str()).is_some()
            && should_block_key_when_invalid(event)
        {
            window.prevent_default();
            cx.stop_propagation();
            return;
        }

        let Some(separator) = key_event_char(event) else {
            return;
        };

        if !selected_range.is_empty() {
            return;
        }

        let Some(completed) =
            self.format
                .complete_numeric_part_with_separator(value.as_str(), cursor, separator)
        else {
            return;
        };

        self.input.update(cx, |state, cx| {
            let utf16_len = state.text().chars().map(char::len_utf16).sum();
            state.replace_text_in_range_silent(Some(0..utf16_len), completed.as_str(), window, cx);
        });

        window.prevent_default();
        cx.stop_propagation();
    }
}

impl EventEmitter<TarihGirdisiOlayi> for TarihGirdisiDurumu {}

impl Focusable for TarihGirdisiDurumu {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input.focus_handle(cx)
    }
}

impl Render for TarihGirdisiDurumu {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.input.clone()
    }
}

struct MonthSuggestionsPopover {
    id: ElementId,
    input: Entity<InputState>,
    date_input: Entity<TarihGirdisiDurumu>,
    range: Range<usize>,
    suggestions: Vec<MonthNameSuggestion>,
    size: BilesenBoyutu,
}

impl MonthSuggestionsPopover {
    fn new(
        input: Entity<InputState>,
        date_input: Entity<TarihGirdisiDurumu>,
        range: Range<usize>,
        suggestions: Vec<MonthNameSuggestion>,
        size: BilesenBoyutu,
    ) -> Self {
        Self {
            id: "date-input-month-suggestions".into(),
            input,
            date_input,
            range,
            suggestions,
            size,
        }
    }

    fn render_content(&self, window: &mut Window, cx: &mut App) -> AnyElement {
        div()
            .id("date-input-month-suggestions-content")
            .occlude()
            .popover_style(cx)
            .shadow_md()
            .p_1()
            .min_w(px(160.))
            .max_h(px(240.))
            .overflow_y_scroll()
            .children(self.suggestions.iter().map(|suggestion| {
                let month = suggestion.month;
                div()
                    .id(("month-suggestion", month))
                    .w_full()
                    .px_2()
                    .py_1()
                    .input_text_size(self.size)
                    .rounded(cx.theme().radius)
                    .cursor_pointer()
                    .hover(|this| this.bg(cx.theme().accent))
                    .child(suggestion.name.clone())
                    .on_click(
                        window.listener_for(&self.date_input, move |this, _, window, cx| {
                            this.select_month_suggestion(month, window, cx);
                        }),
                    )
            }))
            .into_any_element()
    }
}

impl IntoElement for MonthSuggestionsPopover {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

struct MonthSuggestionsLayoutState {
    bounds: Bounds<Pixels>,
    element: Option<AnyElement>,
}

impl Element for MonthSuggestionsPopover {
    type RequestLayoutState = MonthSuggestionsLayoutState;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&crate::ham_gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let Some(trigger_bounds) = self.input.read(cx).range_to_bounds(&self.range) else {
            let mut empty = div().into_any_element();
            return (
                empty.request_layout(window, cx),
                MonthSuggestionsLayoutState {
                    bounds: Bounds::default(),
                    element: None,
                },
            );
        };

        let mut popover = deferred(self.render_content(window, cx)).into_any_element();
        let popover_size = popover.layout_as_root(AvailableSpace::min_size(), window, cx);
        let snap = px(8.);
        let gap = px(4.);
        let window_size = window.bounds().size;
        let mut pos = point(trigger_bounds.left(), trigger_bounds.bottom() + gap);

        if pos.x + popover_size.width > window_size.width - snap {
            pos.x = (window_size.width - popover_size.width - snap).max(snap);
        }
        if pos.y + popover_size.height > window_size.height - snap {
            pos.y = (trigger_bounds.top() - popover_size.height - gap).max(snap);
        }

        let mut empty = div().into_any_element();
        (
            empty.request_layout(window, cx),
            MonthSuggestionsLayoutState {
                bounds: Bounds {
                    origin: pos,
                    size: popover_size,
                },
                element: Some(popover),
            },
        )
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&crate::ham_gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let Some(popover) = request_layout.element.as_mut() else {
            return;
        };

        window.with_absolute_element_offset(request_layout.bounds.origin, |window| {
            popover.prepaint(window, cx);
        });
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&crate::ham_gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let Some(popover) = request_layout.element.as_mut() else {
            return;
        };

        popover.paint(window, cx);
    }
}

/// Bir metin girdi olan parses bir tarih değer.
#[derive(IntoElement)]
pub struct TarihGirdisi {
    id: ElementId,
    state: Entity<TarihGirdisiDurumu>,
    style: StyleRefinement,
    size: BilesenBoyutu,
    placeholder: Option<SharedString>,
    appearance: bool,
    cleanable: bool,
    disabled: bool,
}

impl TarihGirdisi {
    pub fn new(state: &Entity<TarihGirdisiDurumu>) -> Self {
        Self {
            id: ("date-input", state.entity_id()).into(),
            state: state.clone(),
            style: StyleRefinement::default(),
            size: BilesenBoyutu::default(),
            placeholder: None,
            appearance: true,
            cleanable: false,
            disabled: false,
        }
    }

    /// yer tutucu tarih girdi ayarlar.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Girdi boş değilken temizleme düğmesinin gösterilip gösterilmeyeceğini ayarlar.
    pub fn cleanable(mut self, cleanable: bool) -> Self {
        self.cleanable = cleanable;
        self
    }

    /// Tarih girdisinin görünümünü ayarlar; false ise kenarlık/arka plan olmaz.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }
}

impl Focusable for TarihGirdisi {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.state.focus_handle(cx)
    }
}

impl Boyutlandirilabilir for TarihGirdisi {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for TarihGirdisi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl DevreDisiBirakilabilir for TarihGirdisi {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for TarihGirdisi {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.state.update(cx, |state, cx| {
            state.sync_input(window, cx);
            state.set_placeholder(self.placeholder.clone(), window, cx);
        });

        let state = self.state.read(cx);
        let input = state.input();
        let calendar = state.calendar();
        let open = state.open;
        let show_calendar_icon = !self.disabled && state.should_show_calendar_icon(cx);
        let month_suggestions = state
            .focus_handle(cx)
            .is_focused(window)
            .then(|| state.month_name_suggestions(cx))
            .flatten();
        let date_input = self.state.clone();

        let input_element = Input::new(&input)
            .appearance(self.appearance)
            .cleanable(self.cleanable)
            .disabled(self.disabled)
            .with_size(self.size);
        let input_element = if show_calendar_icon {
            input_element.suffix(
                Dugme::new("calendar")
                    .icon(SimgeAdi::Calendar)
                    .xsmall()
                    .ghost()
                    .tab_stop(false)
                    .on_click(
                        window.listener_for(&self.state, TarihGirdisiDurumu::toggle_calendar),
                    ),
            )
        } else {
            input_element
        };

        div()
            .id(self.id)
            .w_full()
            .relative()
            .refine_style(&self.style)
            .capture_key_down(window.listener_for(&self.state, TarihGirdisiDurumu::on_key_down))
            .child(input_element)
            .when_some(month_suggestions, |this, (context, suggestions)| {
                this.child(MonthSuggestionsPopover::new(
                    input.clone(),
                    date_input,
                    context.cursor..context.cursor,
                    suggestions,
                    self.size,
                ))
            })
            .when(open, |this| {
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
                                    window.listener_for(
                                        &self.state,
                                        TarihGirdisiDurumu::close_calendar,
                                    ),
                                )
                                .child(
                                    Takvim::new(&calendar)
                                        .number_of_months(1)
                                        .border_0()
                                        .rounded_none()
                                        .p_0()
                                        .with_size(self.size),
                                ),
                        ),
                    )
                    .with_priority(2),
                )
            })
    }
}

fn starts_with(chars: &[char], ix: usize, value: &str) -> bool {
    value
        .chars()
        .enumerate()
        .all(|(offset, ch)| chars.get(ix + offset) == Some(&ch))
}

fn push_literal(parts: &mut Vec<DateInputPart>, literal: &mut String) {
    if literal.is_empty() {
        return;
    }

    parts.push(DateInputPart::Literal(std::mem::take(literal)));
}

fn parse_digits(value: &str, cursor: usize, min: usize, max: usize) -> Option<(u32, usize)> {
    let mut count = 0;
    let mut end = cursor;

    for (offset, ch) in value[cursor..].char_indices() {
        if count == max || !ch.is_ascii_digit() {
            break;
        }

        count += 1;
        end = cursor + offset + ch.len_utf8();
    }

    if count < min {
        return None;
    }

    value[cursor..end].parse::<u32>().ok().map(|v| (v, end))
}

fn parse_partial_digits(value: &str, cursor: usize, max: usize) -> Option<ParsedDatePart> {
    let start = cursor;
    let mut count = 0;
    let mut end = cursor;

    for (offset, ch) in value[cursor..].char_indices() {
        if count == max || !ch.is_ascii_digit() {
            break;
        }

        count += 1;
        end = cursor + offset + ch.len_utf8();
    }

    if count == 0 {
        return None;
    }

    Some(ParsedDatePart {
        value: value[start..end].parse::<u32>().ok()?,
        range: start..end,
        complete: count == max,
    })
}

fn parse_partial_month_name(
    value: &str,
    cursor: usize,
    width: usize,
    names: impl IntoIterator<Item = SharedString>,
) -> Option<ParsedDatePart> {
    let start = cursor;
    let mut count = 0;
    let mut end = cursor;

    for (offset, ch) in value[cursor..].char_indices() {
        if count == width || !ch.is_alphabetic() {
            break;
        }

        count += 1;
        end = cursor + offset + ch.len_utf8();
    }

    if count == 0 {
        return None;
    }

    if count < width {
        return Some(ParsedDatePart {
            value: 0,
            range: start..end,
            complete: false,
        });
    }

    let candidate = &value[start..end];
    let month = names
        .into_iter()
        .enumerate()
        .find_map(|(ix, name)| {
            (normalize_month_name(candidate) == normalize_month_name(name.as_str()))
                .then_some((ix + 1) as u32)
        })
        .unwrap_or(13);

    Some(ParsedDatePart {
        value: month,
        range: start..end,
        complete: true,
    })
}

fn parse_partial_variable_month_name(
    value: &str,
    cursor: usize,
    names: impl IntoIterator<Item = SharedString>,
) -> Option<ParsedDatePart> {
    let start = cursor;
    let mut end = cursor;

    for (offset, ch) in value[cursor..].char_indices() {
        if !ch.is_alphabetic() {
            break;
        }

        end = cursor + offset + ch.len_utf8();
    }

    if start == end {
        return None;
    }

    let candidate = normalize_month_name(&value[start..end]);
    let names = names
        .into_iter()
        .enumerate()
        .map(|(ix, name)| ((ix + 1) as u32, normalize_month_name(name.as_str())))
        .collect::<Vec<_>>();

    if let Some((month, _)) = names.iter().find(|(_, name)| *name == candidate) {
        return Some(ParsedDatePart {
            value: *month,
            range: start..end,
            complete: true,
        });
    }

    let is_prefix = names.iter().any(|(_, name)| name.starts_with(&candidate));
    Some(ParsedDatePart {
        value: if is_prefix { 0 } else { 13 },
        range: start..end,
        complete: !is_prefix,
    })
}

fn days_in_month_for_validation(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .and_then(|date| date.pred_opt())
        .map(|date| date.day())
        .unwrap_or(31)
}

fn join_ranges(a: &Range<usize>, b: &Range<usize>) -> Range<usize> {
    a.start.min(b.start)..a.end.max(b.end)
}

fn byte_range_to_utf16(value: &str, range: &Range<usize>) -> Range<usize> {
    let start = value[..range.start].chars().map(char::len_utf16).sum();
    let end = value[..range.end].chars().map(char::len_utf16).sum();
    start..end
}

fn consume_digits_up_to(value: &str, cursor: &mut usize, max: usize) -> usize {
    let mut count = 0;

    while count < max {
        let Some(ch) = value[*cursor..].chars().next() else {
            break;
        };

        if !ch.is_ascii_digit() {
            break;
        }

        count += 1;
        *cursor += ch.len_utf8();
    }

    count
}

fn consume_letters_up_to(value: &str, cursor: &mut usize, max: usize) -> usize {
    let mut count = 0;

    while count < max {
        let Some(ch) = value[*cursor..].chars().next() else {
            break;
        };

        if !ch.is_alphabetic() {
            break;
        }

        count += 1;
        *cursor += ch.len_utf8();
    }

    count
}

fn consume_letters_end(value: &str, cursor: usize) -> usize {
    let mut end = cursor;

    for (offset, ch) in value[cursor..].char_indices() {
        if !ch.is_alphabetic() {
            break;
        }

        end = cursor + offset + ch.len_utf8();
    }

    end
}

fn month_accepts_day(month: u32, day: u32) -> bool {
    day <= days_in_month_for_validation(2000, month)
}

fn parse_two_digit_year(year: u32) -> i32 {
    if year >= 50 {
        1900 + year as i32
    } else {
        2000 + year as i32
    }
}

fn consume_literal(value: &str, cursor: usize, literal: &str) -> Option<usize> {
    if literal.chars().all(char::is_whitespace) {
        let mut consumed = false;
        let mut end = cursor;

        for (offset, ch) in value[cursor..].char_indices() {
            if !ch.is_whitespace() {
                break;
            }

            consumed = true;
            end = cursor + offset + ch.len_utf8();
        }

        return consumed.then_some(end);
    }

    value[cursor..]
        .strip_prefix(literal)
        .map(|_| cursor + literal.len())
}

fn parse_month_name(
    value: &str,
    cursor: usize,
    names: impl IntoIterator<Item = SharedString>,
) -> Option<(u32, usize)> {
    let remaining = &value[cursor..];
    let mut months = names
        .into_iter()
        .enumerate()
        .map(|(ix, name)| ((ix + 1) as u32, name.to_string()))
        .collect::<Vec<_>>();
    months.sort_by_key(|(_, name)| std::cmp::Reverse(name.chars().count()));

    for (month, name) in months {
        let candidate = remaining
            .chars()
            .take(name.chars().count())
            .collect::<String>();
        if normalize_month_name(&candidate) == normalize_month_name(&name) {
            return Some((month, cursor + candidate.len()));
        }
    }

    None
}

fn normalize_month_name(value: &str) -> String {
    value.trim().trim_end_matches('.').to_lowercase()
}

fn key_event_char(event: &KeyDownEvent) -> Option<char> {
    let key = event.keystroke.key.as_str();
    single_char(key).or_else(|| event.keystroke.key_char.as_deref().and_then(single_char))
}

fn should_block_key_when_invalid(event: &KeyDownEvent) -> bool {
    let key = event.keystroke.key.as_str();
    if matches!(
        key,
        "backspace"
            | "delete"
            | "left"
            | "right"
            | "up"
            | "down"
            | "home"
            | "end"
            | "tab"
            | "escape"
    ) {
        return false;
    }

    key_event_char(event).is_some()
}

fn single_char(value: &str) -> Option<char> {
    let mut chars = value.chars();
    let ch = chars.next()?;
    chars.next().is_none().then_some(ch)
}

#[cfg(test)]
mod tests {
    use super::{TarihGirdisiFormati, parse_two_digit_year};
    use crate::i18n::{clear_locale_override, set_locale_override};
    use chrono::NaiveDate;

    struct LocaleOverrideGuard;

    impl Drop for LocaleOverrideGuard {
        fn drop(&mut self) {
            clear_locale_override();
        }
    }

    fn set_test_locale(locale: &str) -> LocaleOverrideGuard {
        set_locale_override(locale);
        LocaleOverrideGuard
    }

    fn month_suggestion_names(format: &TarihGirdisiFormati, value: &str) -> Vec<String> {
        let (_, suggestions) = format
            .month_name_suggestions(value, value.len())
            .expect("ay adı önerileri kullanılabilir olmalı");

        suggestions
            .into_iter()
            .map(|suggestion| super::normalize_month_name(suggestion.name.as_str()))
            .collect()
    }

    #[test]
    fn test_numeric_date_format() {
        let format = TarihGirdisiFormati::new("gg/aa/yyyy");
        let date = NaiveDate::from_ymd_opt(2013, 5, 24).unwrap();

        assert_eq!(format.format(date), "24/05/2013");
        assert_eq!(format.parse("24/05/2013"), Some(date));
        assert_eq!(format.parse("31/02/2013"), None);
        assert_eq!(format.mask_pattern().as_deref(), Some("##/##/####"));
    }

    #[test]
    fn test_reordered_numeric_date_formats() {
        let date = NaiveDate::from_ymd_opt(2013, 5, 24).unwrap();

        let year_month_day = TarihGirdisiFormati::new("yyyy-aa-gg");
        assert_eq!(year_month_day.format(date), "2013-05-24");
        assert_eq!(year_month_day.parse("2013-05-24"), Some(date));
        assert_eq!(year_month_day.mask_pattern().as_deref(), Some("####-##-##"));

        let month_day_year = TarihGirdisiFormati::new("aa-gg-yyyy");
        assert_eq!(month_day_year.format(date), "05-24-2013");
        assert_eq!(month_day_year.parse("05-24-2013"), Some(date));
        assert_eq!(month_day_year.mask_pattern().as_deref(), Some("##-##-####"));
    }

    #[test]
    fn test_two_digit_year_date_format() {
        let format = TarihGirdisiFormati::new("aa-gg-yy");
        let date = NaiveDate::from_ymd_opt(2013, 5, 24).unwrap();

        assert_eq!(format.format(date), "05-24-13");
        assert_eq!(format.parse("05-24-13"), Some(date));
        assert_eq!(format.mask_pattern().as_deref(), Some("##-##-##"));
    }

    #[test]
    fn test_separator_completes_single_digit_numeric_parts() {
        let format = TarihGirdisiFormati::new("gg/aa/yyyy");

        assert_eq!(
            format.complete_numeric_part_with_separator("5", 1, '/'),
            Some("05/".to_string())
        );
        assert_eq!(
            format.complete_numeric_part_with_separator("05/8", 4, '/'),
            Some("05/08/".to_string())
        );
        assert_eq!(
            format.complete_numeric_part_with_separator("5", 1, '-'),
            None
        );
        assert_eq!(
            format.complete_numeric_part_with_separator("05", 2, '/'),
            None
        );
    }

    #[test]
    fn test_separator_completion_follows_date_part_order() {
        let year_month_day = TarihGirdisiFormati::new("yyyy-aa-gg");
        assert_eq!(
            year_month_day.complete_numeric_part_with_separator("2013-5", 6, '-'),
            Some("2013-05-".to_string())
        );
        assert_eq!(
            year_month_day.complete_numeric_part_with_separator("2", 1, '-'),
            None
        );

        let month_day_year = TarihGirdisiFormati::new("aa-gg-yyyy");
        assert_eq!(
            month_day_year.complete_numeric_part_with_separator("5", 1, '-'),
            Some("05-".to_string())
        );
        assert_eq!(
            month_day_year.complete_numeric_part_with_separator("05-8", 4, '-'),
            Some("05-08-".to_string())
        );
    }

    #[test]
    fn test_separator_completion_after_short_month_name() {
        let format = TarihGirdisiFormati::new("AAA-gg-yy");

        assert_eq!(
            format.complete_numeric_part_with_separator("Oca-5", 5, '-'),
            Some("Oca-05-".to_string())
        );
    }

    #[test]
    fn test_numeric_date_validation_ranges() {
        let format = TarihGirdisiFormati::new("gg/aa/yyyy");

        assert_eq!(format.first_validation_error("32"), Some(0..2));
        assert_eq!(format.first_validation_error("12/13"), Some(3..5));
        assert_eq!(format.first_validation_error("30/02"), Some(0..5));
        assert_eq!(format.first_validation_error("29/02"), None);
        assert_eq!(format.first_validation_error("29/02/2013"), Some(0..10));
        assert_eq!(format.first_validation_error("29/02/2012"), None);
        assert_eq!(format.first_validation_error("31/04/2012"), Some(0..5));
    }

    #[test]
    fn test_reordered_date_validation_ranges() {
        let year_month_day = TarihGirdisiFormati::new("yyyy-aa-gg");
        assert_eq!(
            year_month_day.first_validation_error("2013-02-30"),
            Some(5..10)
        );

        let month_day_year = TarihGirdisiFormati::new("aa-gg-yyyy");
        assert_eq!(month_day_year.first_validation_error("13"), Some(0..2));
        assert_eq!(
            month_day_year.first_validation_error("02-29-2013"),
            Some(0..10)
        );
    }

    #[test]
    fn test_short_month_name_date_format() {
        set_locale_override("tr-TR");
        let format = TarihGirdisiFormati::new("gg-AAA-yyyy");
        let date = NaiveDate::from_ymd_opt(1958, 1, 24).unwrap();
        let value = format.format(date);

        assert_eq!(format.parse("24-Oca-1958"), Some(date));
        assert_eq!(format.parse(value.as_str()), Some(date));
        assert_eq!(format.mask_pattern().as_deref(), Some("##-AAA-####"));
        clear_locale_override();
    }

    #[test]
    fn test_short_month_name_with_two_digit_year() {
        set_locale_override("tr-TR");
        let format = TarihGirdisiFormati::new("AAA-gg-yy");
        let date = NaiveDate::from_ymd_opt(1958, 1, 24).unwrap();

        assert_eq!(format.format(date), "Oca-24-58");
        assert_eq!(format.parse("Oca-24-58"), Some(date));
        assert_eq!(format.mask_pattern().as_deref(), Some("AAA-##-##"));
        clear_locale_override();
    }

    #[test]
    fn test_parse_two_digit_year() {
        assert_eq!(parse_two_digit_year(13), 2013);
        assert_eq!(parse_two_digit_year(49), 2049);
        assert_eq!(parse_two_digit_year(50), 1950);
        assert_eq!(parse_two_digit_year(58), 1958);
    }

    #[test]
    fn test_month_name_date_format() {
        let format = TarihGirdisiFormati::new("gg-aaaa-yyyy");
        let date = NaiveDate::from_ymd_opt(2013, 5, 24).unwrap();
        let value = format.format(date);

        assert_eq!(format.parse(value.as_str()), Some(date));
        assert_eq!(format.mask_pattern(), None);
    }

    #[test]
    fn test_month_name_with_spaces() {
        let format = TarihGirdisiFormati::new("gg aaaa yyyy");
        let date = NaiveDate::from_ymd_opt(2013, 5, 24).unwrap();
        let value = format.format(date);

        assert_eq!(format.parse(value.as_str()), Some(date));
    }

    #[test]
    fn test_month_name_suggestions_follow_day_count() {
        let _locale = set_test_locale("tr-TR");
        let format = TarihGirdisiFormati::new("gg-aaaa-yyyy");

        let day_29 = month_suggestion_names(&format, "29-");
        assert_eq!(day_29.len(), 12);
        assert!(day_29.contains(&"şubat".to_string()));

        let day_30 = month_suggestion_names(&format, "30-");
        assert_eq!(day_30.len(), 11);
        assert!(!day_30.contains(&"şubat".to_string()));

        assert_eq!(
            month_suggestion_names(&format, "31-"),
            vec![
                "ocak", "mart", "mayıs", "temmuz", "ağustos", "ekim", "aralık"
            ]
        );
    }

    #[test]
    fn test_month_name_suggestions_filter_turkish_months() {
        let _locale = set_test_locale("tr-TR");
        let format = TarihGirdisiFormati::new("gg-aaaa-yyyy");

        assert_eq!(
            month_suggestion_names(&format, "12-A"),
            vec!["ağustos", "aralık"]
        );
    }

    #[test]
    fn test_month_name_suggestions_support_space_separator() {
        let _locale = set_test_locale("tr-TR");
        let format = TarihGirdisiFormati::new("gg aaaa yyyy");

        assert_eq!(
            month_suggestion_names(&format, "12 A"),
            vec!["ağustos", "aralık"]
        );
    }
}
