use crate::ham_gpui::SharedString;
use chrono::{Datelike, NaiveDate};
use icu::{
    calendar::{Gregorian, types::Weekday, week::WeekInformation},
    datetime::{
        FixedCalendarDateTimeFormatter,
        fieldsets::{E, M, YMD},
        input::Date,
    },
    decimal::{
        DecimalFormatter,
        input::Decimal,
        options::{DecimalFormatterOptions, GroupingStrategy},
    },
    locale::{Locale, locale},
};
use once_cell::sync::Lazy;
use std::sync::RwLock;
use writeable::Writeable;

static LOCALE_OVERRIDE: Lazy<RwLock<Option<String>>> = Lazy::new(|| RwLock::new(None));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct NumberSymbols {
    pub(crate) group_separator: Option<char>,
    pub(crate) decimal_separator: char,
}

impl NumberSymbols {
    pub(crate) fn for_current_locale() -> Self {
        Self::for_locale_str(&current_locale_tag())
    }

    pub(crate) fn for_locale_str(locale: &str) -> Self {
        let locale = locale_from_str(locale);
        let formatter = DecimalFormatter::try_new(
            locale.into(),
            DecimalFormatterOptions::from(GroupingStrategy::Always),
        );

        let Ok(formatter) = formatter else {
            return Self::english();
        };

        let group_separator =
            non_numeric_separator(&formatter.format(&Decimal::from(1234567)).write_to_string());

        let mut decimal = Decimal::from(11);
        decimal.multiply_pow10(-1);
        let decimal_separator =
            non_numeric_separator(&formatter.format(&decimal).write_to_string()).unwrap_or('.');

        Self {
            group_separator,
            decimal_separator,
        }
    }

    fn english() -> Self {
        Self {
            group_separator: Some(','),
            decimal_separator: '.',
        }
    }
}

pub(crate) fn set_locale_override(locale: &str) {
    let mut locale_override = LOCALE_OVERRIDE
        .write()
        .expect("yerel ayar geçersiz kılma kilidi zehirlenmiş olmamalı");
    *locale_override = canonical_locale_tag(locale);
}

#[cfg(test)]
pub(crate) fn clear_locale_override() {
    let mut locale_override = LOCALE_OVERRIDE
        .write()
        .expect("yerel ayar geçersiz kılma kilidi zehirlenmiş olmamalı");
    *locale_override = None;
}

pub(crate) fn month_name(month: u32) -> SharedString {
    let Some(date) = NaiveDate::from_ymd_opt(2024, month, 1) else {
        return SharedString::default();
    };

    format_month_name(date).unwrap_or_else(|| fallback_month_name(month).into())
}

pub(crate) fn month_short_name(month: u32) -> SharedString {
    let Some(date) = NaiveDate::from_ymd_opt(2024, month, 1) else {
        return SharedString::default();
    };

    format_month_short_name(date)
        .or_else(|| format_month_name(date))
        .map(|name| normalize_short_month_name(name.as_str()).into())
        .unwrap_or_default()
}

pub(crate) fn month_names() -> Vec<SharedString> {
    (1..=12).map(month_name).collect()
}

pub(crate) fn month_short_names() -> Vec<SharedString> {
    (1..=12).map(month_short_name).collect()
}

pub(crate) fn weekday_names() -> Vec<SharedString> {
    let first_weekday_index = first_weekday();

    (0..7)
        .filter_map(|offset| {
            NaiveDate::from_ymd_opt(2024, 1, 7 + ((first_weekday_index + offset) % 7))
        })
        .map(|date| {
            format_weekday_name(date).unwrap_or_else(|| {
                fallback_weekday_name(date.weekday().num_days_from_sunday()).into()
            })
        })
        .collect()
}

pub(crate) fn first_weekday() -> u32 {
    first_weekday_for_locale_str(&current_locale_tag())
}

pub(crate) fn format_date(date: NaiveDate) -> SharedString {
    format_default_date(date).unwrap_or_else(|| date.format("%Y/%m/%d").to_string().into())
}

pub(crate) fn date_input_format() -> SharedString {
    default_date_input_format().unwrap_or_else(|| "yyyy/aa/gg".into())
}

fn format_month_name(date: NaiveDate) -> Option<SharedString> {
    let date = icu_date(date)?;
    let locale = locale_from_str(&current_locale_tag());
    let formatter =
        FixedCalendarDateTimeFormatter::<Gregorian, M>::try_new(locale.into(), M::long()).ok()?;

    Some(
        formatter
            .format(&date)
            .write_to_string()
            .into_owned()
            .into(),
    )
}

fn format_month_short_name(date: NaiveDate) -> Option<SharedString> {
    let date = icu_date(date)?;
    let locale = locale_from_str(&current_locale_tag());
    let formatter =
        FixedCalendarDateTimeFormatter::<Gregorian, M>::try_new(locale.into(), M::medium()).ok()?;

    Some(
        formatter
            .format(&date)
            .write_to_string()
            .into_owned()
            .into(),
    )
}

fn format_weekday_name(date: NaiveDate) -> Option<SharedString> {
    let date = icu_date(date)?;
    let locale = locale_from_str(&current_locale_tag());
    let formatter =
        FixedCalendarDateTimeFormatter::<Gregorian, E>::try_new(locale.into(), E::short()).ok()?;

    Some(
        formatter
            .format(&date)
            .write_to_string()
            .into_owned()
            .into(),
    )
}

fn format_default_date(date: NaiveDate) -> Option<SharedString> {
    let date = icu_date(date)?;
    let locale = locale_from_str(&current_locale_tag());
    let formatter =
        FixedCalendarDateTimeFormatter::<Gregorian, YMD>::try_new(locale.into(), YMD::medium())
            .ok()?;

    Some(
        formatter
            .format(&date)
            .write_to_string()
            .into_owned()
            .into(),
    )
}

fn default_date_input_format() -> Option<SharedString> {
    let date = icu_date(NaiveDate::from_ymd_opt(2006, 11, 24)?)?;
    let locale = locale_from_str(&current_locale_tag());
    let formatter =
        FixedCalendarDateTimeFormatter::<Gregorian, YMD>::try_new(locale.into(), YMD::short())
            .ok()?;
    let formatted = formatter.format(&date);
    let pattern = formatted.pattern();
    let pattern = pattern.write_to_string();

    Some(date_input_format_from_icu_pattern(&pattern).into())
}

fn date_input_format_from_icu_pattern(pattern: &str) -> String {
    let mut result = String::new();
    let chars = pattern.chars().collect::<Vec<_>>();
    let mut ix = 0;
    let mut in_quote = false;

    while ix < chars.len() {
        let ch = chars[ix];

        if ch == '\'' {
            if chars.get(ix + 1) == Some(&'\'') {
                result.push('\'');
                ix += 2;
            } else {
                in_quote = !in_quote;
                ix += 1;
            }
            continue;
        }

        if !in_quote && matches!(ch, 'd' | 'M' | 'L' | 'y') {
            let field = ch;
            let start = ix;
            while ix < chars.len() && chars[ix] == field {
                ix += 1;
            }
            let width = ix - start;

            match field {
                'd' => result.push_str("gg"),
                'M' | 'L' if width >= 4 => result.push_str("aaaa"),
                'M' | 'L' if width == 3 => result.push_str("AAA"),
                'M' | 'L' => result.push_str("aa"),
                'y' if width == 2 => result.push_str("yy"),
                'y' => result.push_str("yyyy"),
                _ => {}
            }
            continue;
        }

        result.push(ch);
        ix += 1;
    }

    result
}

fn first_weekday_for_locale_str(locale: &str) -> u32 {
    let locale = locale_from_str(locale);

    WeekInformation::try_new(locale.into())
        .map(|info| weekday_days_from_sunday(info.first_weekday))
        .unwrap_or(0)
}

fn weekday_days_from_sunday(weekday: Weekday) -> u32 {
    match weekday {
        Weekday::Sunday => 0,
        Weekday::Monday => 1,
        Weekday::Tuesday => 2,
        Weekday::Wednesday => 3,
        Weekday::Thursday => 4,
        Weekday::Friday => 5,
        Weekday::Saturday => 6,
    }
}

fn icu_date(date: NaiveDate) -> Option<Date<Gregorian>> {
    Date::try_new_gregorian(date.year(), date.month() as u8, date.day() as u8).ok()
}

fn current_locale_tag() -> String {
    if let Some(locale) = LOCALE_OVERRIDE
        .read()
        .expect("yerel ayar geçersiz kılma kilidi zehirlenmiş olmamalı")
        .clone()
    {
        return locale;
    }

    let app_locale = crate::locale();
    system_locale_tag()
        .or_else(|| canonical_locale_tag(&app_locale))
        .unwrap_or_else(|| "en".to_string())
}

fn locale_from_str(locale: &str) -> Locale {
    canonical_locale_tag(locale)
        .and_then(|locale| locale.parse::<Locale>().ok())
        .unwrap_or_else(|| locale!("en").into())
}

fn canonical_locale_tag(locale: &str) -> Option<String> {
    let locale = locale
        .split(['.', '@'])
        .next()
        .unwrap_or(locale)
        .trim()
        .replace('_', "-");

    if locale.is_empty() || locale.eq_ignore_ascii_case("c") || locale.eq_ignore_ascii_case("posix")
    {
        return None;
    }

    Some(locale)
}

fn non_numeric_separator(value: &str) -> Option<char> {
    value
        .chars()
        .find(|ch| !ch.is_numeric() && !matches!(ch, '+' | '-' | '\u{200e}' | '\u{200f}'))
}

#[cfg(not(target_family = "wasm"))]
fn system_locale_tag() -> Option<String> {
    ["LC_NUMERIC", "LC_ALL", "LANG"]
        .iter()
        .filter_map(|key| std::env::var(key).ok())
        .find_map(|locale| canonical_locale_tag(&locale))
}

#[cfg(target_family = "wasm")]
fn system_locale_tag() -> Option<String> {
    None
}

fn fallback_month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

fn normalize_short_month_name(value: &str) -> String {
    let value = value.trim().trim_end_matches('.');
    value.chars().take(3).collect()
}

fn fallback_weekday_name(weekday: u32) -> &'static str {
    match weekday {
        0 => "Sun",
        1 => "Mon",
        2 => "Tue",
        3 => "Wed",
        4 => "Thu",
        5 => "Fri",
        6 => "Sat",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        NumberSymbols, date_input_format_from_icu_pattern, first_weekday_for_locale_str,
        format_date, normalize_short_month_name,
    };
    use chrono::NaiveDate;

    #[test]
    fn test_number_symbols_for_locale() {
        assert_eq!(
            NumberSymbols::for_locale_str("tr_TR.UTF-8"),
            NumberSymbols {
                group_separator: Some('.'),
                decimal_separator: ',',
            }
        );
        assert_eq!(
            NumberSymbols::for_locale_str("en-US"),
            NumberSymbols {
                group_separator: Some(','),
                decimal_separator: '.',
            }
        );
    }

    #[test]
    fn test_format_date_falls_back_to_current_locale() {
        assert!(!format_date(NaiveDate::from_ymd_opt(2024, 5, 17).unwrap()).is_empty());
    }

    #[test]
    fn test_first_weekday_for_locale() {
        assert_eq!(first_weekday_for_locale_str("en-US"), 0);
        assert_eq!(first_weekday_for_locale_str("tr-TR"), 1);
    }

    #[test]
    fn test_date_input_format_from_icu_pattern() {
        assert_eq!(date_input_format_from_icu_pattern("M/d/y"), "aa/gg/yyyy");
        assert_eq!(date_input_format_from_icu_pattern("M/d/yy"), "aa/gg/yy");
        assert_eq!(date_input_format_from_icu_pattern("dd.MM.y"), "gg.aa.yyyy");
        assert_eq!(date_input_format_from_icu_pattern("d MMM y"), "gg AAA yyyy");
        assert_eq!(
            date_input_format_from_icu_pattern("d MMMM y"),
            "gg aaaa yyyy"
        );
    }

    #[test]
    fn test_normalize_short_month_name() {
        assert_eq!(normalize_short_month_name("Oca."), "Oca");
        assert_eq!(normalize_short_month_name("Mayıs"), "May");
    }
}
