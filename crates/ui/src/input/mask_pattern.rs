use crate::ham_gpui::SharedString;
use crate::i18n::NumberSymbols;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MaskDisplayRun {
    pub(crate) range: Range<usize>,
    pub(crate) is_placeholder: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MaskDisplay {
    pub(crate) text: SharedString,
    pub(crate) runs: Vec<MaskDisplayRun>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum MaskToken {
    /// 0 Digit, equivalent için `[0]`
    // Digit0,
    /// Digit, equivalent için `[0-9]`
    Digit,
    /// Letter, dahil yerel ayar-özel Unicode letters.
    Letter,
    /// Letter veya digit, equivalent için `[a-zA-Z0-9]`
    LetterOrDigit,
    /// Separator
    Sep(char),
    /// Herhangi bir karakter
    Any,
}

#[allow(unused)]
impl MaskToken {
    /// Belirtecin herhangi bir karakterle eşleşip eşleşmediğini kontrol eder.
    pub fn is_any(&self) -> bool {
        matches!(self, MaskToken::Any)
    }

    /// Belirtecin verilen karakterle eşleşip eşleşmediğini kontrol eder.
    ///
    /// Ayırıcı her zaman herhangi bir girdi karakteriyle eşleşir.
    fn is_match(&self, ch: char) -> bool {
        match self {
            MaskToken::Digit => ch.is_ascii_digit(),
            MaskToken::Letter => ch.is_alphabetic(),
            MaskToken::LetterOrDigit => ch.is_alphanumeric(),
            MaskToken::Any => true,
            MaskToken::Sep(c) => *c == ch,
        }
    }

    /// Is belirteç bir ayırıcı (Olabilir olmak yok sayılır)
    fn is_sep(&self) -> bool {
        matches!(self, MaskToken::Sep(_))
    }

    /// Belirtecin sayı olup olmadığını kontrol eder.
    pub fn is_number(&self) -> bool {
        matches!(self, MaskToken::Digit)
    }

    pub fn placeholder(&self) -> char {
        match self {
            MaskToken::Sep(c) => *c,
            _ => '_',
        }
    }

    fn mask_char(&self, ch: char) -> char {
        match self {
            MaskToken::Digit | MaskToken::LetterOrDigit | MaskToken::Letter => ch,
            MaskToken::Sep(c) => *c,
            MaskToken::Any => ch,
        }
    }

    fn unmask_char(&self, ch: char) -> Option<char> {
        match self {
            MaskToken::Digit => Some(ch),
            MaskToken::Letter => Some(ch),
            MaskToken::LetterOrDigit => Some(ch),
            MaskToken::Any => Some(ch),
            _ => None,
        }
    }
}

#[derive(Clone, Default)]
pub enum MaskPattern {
    #[default]
    None,
    Pattern {
        pattern: SharedString,
        tokens: Vec<MaskToken>,
    },
    Number {
        /// grup ayırıcı, e.g. "," veya " "
        separator: Option<char>,
        /// Sayı fraction digits, e.g. 2 için 123.45
        fraction: Option<usize>,
    },
    LocalizedNumber {
        /// grup ayırıcı, e.g. "," veya "."
        group_separator: Option<char>,
        /// Decimal ayırıcı, e.g. "." veya ","
        decimal_separator: char,
        /// Sayı fraction digits, e.g. 2 için 123.45
        fraction: Option<usize>,
    },
}

impl From<&str> for MaskPattern {
    fn from(pattern: &str) -> Self {
        Self::new(pattern)
    }
}

impl MaskPattern {
    /// Yeni bir maske desen oluşturur.
    ///
    /// - `9` - Digit
    /// - `A` - Letter
    /// - `#` - Digit
    /// - `*` - Herhangi bir karakter
    /// - diğer karakterler - Separator
    ///
    /// Örneğin:
    ///
    /// - `(999)999-9999` - US phone sayı: (123)456-7890
    /// - `99999-9999` - ZIP kod: 12345-6789
    /// - `AAA-###-AAA` - özel desen: ABC-123-DEF
    /// - `*999*` - özel desen: (123) veya [123]
    pub fn new(pattern: &str) -> Self {
        let tokens = pattern
            .chars()
            .map(|ch| match ch {
                // '0' => MaskToken::Digit0,
                '9' => MaskToken::Digit,
                'A' => MaskToken::Letter,
                '#' => MaskToken::Digit,
                '*' => MaskToken::Any,
                _ => MaskToken::Sep(ch),
            })
            .collect();

        Self::Pattern {
            pattern: pattern.to_owned().into(),
            tokens,
        }
    }

    #[allow(unused)]
    fn tokens(&self) -> Option<&Vec<MaskToken>> {
        match self {
            Self::Pattern { tokens, .. } => Some(tokens),
            Self::Number { .. } | Self::LocalizedNumber { .. } => None,
            Self::None => None,
        }
    }

    /// Yeni bir maske desen ile grup ayırıcı, e.g. "," veya " " oluşturur.
    pub fn number(sep: Option<char>) -> Self {
        Self::Number {
            separator: sep,
            fraction: None,
        }
    }

    /// Bir sayı maske kullanarak operating sistem yerel ayar conventions. oluşturur.
    ///
    /// Örneğin, Turkish locales kullanım `.` için thousands ve `,` için decimals,
    /// iken English locales kullanım `,` için thousands ve `.` için decimals.
    pub fn localized_number(fraction: Option<usize>) -> Self {
        let symbols = NumberSymbols::for_current_locale();
        Self::number_with_format(symbols.group_separator, symbols.decimal_separator, fraction)
    }

    /// Bir sayı maske kullanarak verilen yerel ayar metin. oluşturur.
    ///
    /// Bu mainly kullanışlıdır olduğunda bir uygulama wants için bind formatting için onun
    /// kendi yerel ayar ayar bunun yerine process/OS yerel ayar.
    pub fn localized_number_for_locale(locale: &str, fraction: Option<usize>) -> Self {
        let symbols = NumberSymbols::for_locale_str(locale);
        Self::number_with_format(symbols.group_separator, symbols.decimal_separator, fraction)
    }

    /// Bir sayı maske ile explicit grup ve decimal ayırıcılar. oluşturur.
    pub fn number_with_format(
        group_separator: Option<char>,
        decimal_separator: char,
        fraction: Option<usize>,
    ) -> Self {
        Self::LocalizedNumber {
            group_separator,
            decimal_separator,
            fraction,
        }
    }

    pub fn placeholder(&self) -> Option<String> {
        match self {
            Self::Pattern { tokens, .. } => {
                Some(tokens.iter().map(|token| token.placeholder()).collect())
            }
            Self::Number { .. } | Self::LocalizedNumber { .. } => None,
            Self::None => None,
        }
    }

    pub(crate) fn display(&self, mask_text: &str) -> Option<MaskDisplay> {
        let Self::Pattern { tokens, .. } = self else {
            return None;
        };

        let unmasked_text = self.unmask(mask_text);
        let mut chars = unmasked_text.chars();
        let mut text = String::new();
        let mut runs: Vec<MaskDisplayRun> = Vec::with_capacity(tokens.len());

        for token in tokens {
            let start = text.len();
            let is_placeholder = match token {
                MaskToken::Sep(ch) => {
                    text.push(*ch);
                    true
                }
                _ => {
                    if let Some(ch) = chars.next().filter(|ch| token.is_match(*ch)) {
                        text.push(token.mask_char(ch));
                        false
                    } else {
                        text.push(token.placeholder());
                        true
                    }
                }
            };
            let end = text.len();

            if let Some(last) = runs.last_mut()
                && last.is_placeholder == is_placeholder
                && last.range.end == start
            {
                last.range.end = end;
                continue;
            }

            runs.push(MaskDisplayRun {
                range: start..end,
                is_placeholder,
            });
        }

        Some(MaskDisplay {
            text: text.into(),
            runs,
        })
    }

    /// Maske deseni None ise veya desen yoksa true döndürür.
    pub fn is_none(&self) -> bool {
        match self {
            Self::Pattern { tokens, .. } => tokens.is_empty(),
            Self::Number { .. } | Self::LocalizedNumber { .. } => false,
            Self::None => true,
        }
    }

    /// Maskelenmiş metnin geçerli olup olmadığını kontrol eder.
    ///
    /// Maske deseni None ise her zaman true döndürür.
    pub fn is_valid(&self, mask_text: &str) -> bool {
        if self.is_none() {
            return true;
        }

        let mut text_index = 0;
        let mask_text_chars: Vec<char> = mask_text.chars().collect();
        match self {
            Self::Pattern { tokens, .. } => {
                for token in tokens {
                    if text_index >= mask_text_chars.len() {
                        break;
                    }

                    let ch = mask_text_chars[text_index];
                    if token.is_match(ch) {
                        text_index += 1;
                    }
                }
                text_index == mask_text_chars.len()
            }
            Self::Number { separator, .. } => Self::is_valid_number(mask_text, *separator, '.'),
            Self::LocalizedNumber {
                group_separator,
                decimal_separator,
                ..
            } => Self::is_valid_number(mask_text, *group_separator, *decimal_separator),
            Self::None => true,
        }
    }

    fn is_valid_number(
        mask_text: &str,
        group_separator: Option<char>,
        decimal_separator: char,
    ) -> bool {
        if mask_text.is_empty() {
            return true;
        }

        let mut parts = mask_text.split(decimal_separator);
        let int_part = parts.next().unwrap_or("");
        let frac_part = parts.next();

        if parts.next().is_some() || int_part.is_empty() {
            return false;
        }

        let sign_positions: Vec<usize> = int_part
            .chars()
            .enumerate()
            .filter_map(|(i, ch)| match is_sign(&ch) {
                true => Some(i),
                false => None,
            })
            .collect();

        if sign_positions.len() > 1 || sign_positions.first() > Some(&0) {
            return false;
        }

        if !int_part.chars().enumerate().all(|(i, ch)| {
            ch.is_ascii_digit() || is_sign(&ch) && i == 0 || Some(ch) == group_separator
        }) {
            return false;
        }

        if let Some(frac) = frac_part
            && !frac.chars().all(|ch| ch.is_ascii_digit())
        {
            return false;
        }

        true
    }

    /// Verilen konumdaki geçerli girdi karakteri olup olmadığını kontrol eder.
    pub fn is_valid_at(&self, ch: char, pos: usize) -> bool {
        if self.is_none() {
            return true;
        }

        match self {
            Self::Pattern { tokens, .. } => {
                if let Some(token) = tokens.get(pos) {
                    if token.is_match(ch) {
                        return true;
                    }

                    if token.is_sep() {
                        // If next token is match, it's valid
                        if let Some(next_token) = tokens.get(pos + 1) {
                            if next_token.is_match(ch) {
                                return true;
                            }
                        }
                    }
                }

                false
            }
            Self::Number { .. } | Self::LocalizedNumber { .. } => true,
            Self::None => true,
        }
    }

    /// Biçim metin according için maske desen
    ///
    /// Örneğin:
    ///
    /// - desen: (999)999-999
    /// - metin: 123456789
    /// - mask_text: (123)456-789
    pub fn mask(&self, text: &str) -> SharedString {
        if self.is_none() {
            return text.to_owned().into();
        }

        match self {
            Self::Number {
                separator,
                fraction,
            } => Self::mask_number(text, *separator, '.', *fraction),
            Self::LocalizedNumber {
                group_separator,
                decimal_separator,
                fraction,
            } => Self::mask_number(text, *group_separator, *decimal_separator, *fraction),
            Self::Pattern { tokens, .. } => {
                let mut result = String::new();
                let mut text_index = 0;
                let text_chars: Vec<char> = text.chars().collect();
                for token in tokens {
                    if text_index >= text_chars.len() {
                        break;
                    }

                    if let MaskToken::Sep(sep) = token {
                        result.push(*sep);
                        if text_chars.get(text_index) == Some(sep) {
                            text_index += 1;
                        }
                        continue;
                    }

                    loop {
                        let Some(ch) = text_chars.get(text_index) else {
                            break;
                        };

                        if token.is_match(*ch) {
                            result.push(token.mask_char(*ch));
                            text_index += 1;
                            break;
                        }

                        if tokens
                            .iter()
                            .any(|token| token.is_sep() && token.is_match(*ch))
                        {
                            text_index += 1;
                            continue;
                        }

                        return result.into();
                    }
                }
                result.into()
            }
            Self::None => text.to_owned().into(),
        }
    }

    pub(crate) fn append_next_separator_after_cursor(
        &self,
        mask_text: &str,
        cursor: usize,
    ) -> Option<(String, usize)> {
        let Self::Pattern { tokens, .. } = self else {
            return None;
        };

        if cursor != mask_text.len() || !mask_text.is_char_boundary(cursor) {
            return None;
        }

        let token_ix = mask_text[..cursor].chars().count();
        let mut result = mask_text.to_owned();
        let mut next_cursor = cursor;
        for token in tokens.iter().skip(token_ix) {
            let MaskToken::Sep(ch) = token else {
                break;
            };

            result.push(*ch);
            next_cursor += ch.len_utf8();
        }

        (next_cursor > cursor).then_some((result, next_cursor))
    }

    pub(crate) fn first_editable_display_offset(&self) -> Option<usize> {
        let Self::Pattern { tokens, .. } = self else {
            return None;
        };

        let mut offset = 0;
        for token in tokens {
            if token.is_sep() {
                offset += token.placeholder().len_utf8();
            } else {
                return Some(offset);
            }
        }

        None
    }

    pub(crate) fn trim_trailing_separators(&self, mask_text: &str) -> Option<(String, usize)> {
        let Self::Pattern { tokens, .. } = self else {
            return None;
        };

        let mut chars = mask_text
            .char_indices()
            .map(|(start, ch)| (start, start + ch.len_utf8(), ch))
            .collect::<Vec<_>>();
        let mut removed = 0;

        while let Some((start, end, _)) = chars.last().copied() {
            let token_ix = chars.len() - 1;
            if !tokens.get(token_ix).is_some_and(MaskToken::is_sep) {
                break;
            }

            removed += end - start;
            chars.pop();
        }

        if removed == 0 {
            return None;
        }

        let keep_len = mask_text.len() - removed;
        Some((mask_text[..keep_len].to_owned(), removed))
    }

    pub(crate) fn previous_editable_range(
        &self,
        mask_text: &str,
        cursor: usize,
    ) -> Option<Range<usize>> {
        let Self::Pattern { tokens, .. } = self else {
            return None;
        };

        if !mask_text.is_char_boundary(cursor) {
            return None;
        }

        let chars = mask_text[..cursor]
            .char_indices()
            .map(|(start, ch)| (start, start + ch.len_utf8()))
            .collect::<Vec<_>>();

        for (token_ix, (start, end)) in chars.into_iter().enumerate().rev() {
            if !tokens.get(token_ix).is_some_and(MaskToken::is_sep) {
                return Some(start..end);
            }
        }

        None
    }

    pub(crate) fn next_editable_range(
        &self,
        mask_text: &str,
        cursor: usize,
    ) -> Option<Range<usize>> {
        let Self::Pattern { tokens, .. } = self else {
            return None;
        };

        if !mask_text.is_char_boundary(cursor) {
            return None;
        }

        for (token_ix, (start, ch)) in mask_text.char_indices().enumerate() {
            if start < cursor {
                continue;
            }

            if !tokens.get(token_ix).is_some_and(MaskToken::is_sep) {
                return Some(start..start + ch.len_utf8());
            }
        }

        None
    }

    fn mask_number(
        text: &str,
        group_separator: Option<char>,
        decimal_separator: char,
        fraction: Option<usize>,
    ) -> SharedString {
        if let Some(sep) = group_separator {
            let text = text.replace(sep, "");
            let mut parts = text.split(decimal_separator);
            let int_part = parts.next().unwrap_or("");
            let frac_part = parts.next().map(|part| {
                part.chars()
                    .take(fraction.unwrap_or(usize::MAX))
                    .collect::<String>()
            });
            let mut chars: Vec<char> = int_part.chars().rev().collect();
            let maybe_signed = if let Some(pos) = chars.iter().position(is_sign) {
                Some(chars.remove(pos))
            } else {
                None
            };

            let mut result = String::new();
            for (i, ch) in chars.iter().enumerate() {
                if i > 0 && i % 3 == 0 {
                    result.push(sep);
                }
                result.push(*ch);
            }
            let int_with_sep: String = result.chars().rev().collect();

            let final_str = if let Some(frac) = frac_part {
                if fraction == Some(0) {
                    int_with_sep
                } else {
                    format!("{}{}{}", int_with_sep, decimal_separator, frac)
                }
            } else {
                int_with_sep
            };

            let final_str = if let Some(sign) = maybe_signed {
                format!("{}{}", sign, final_str)
            } else {
                final_str
            };

            return final_str.into();
        }

        text.to_owned().into()
    }

    /// Maskelenmiş metni özgün metinden çıkarır.
    pub fn unmask(&self, mask_text: &str) -> String {
        match self {
            Self::Number { separator, .. } => Self::unmask_number(mask_text, *separator, '.'),
            Self::LocalizedNumber {
                group_separator,
                decimal_separator,
                ..
            } => Self::unmask_number(mask_text, *group_separator, *decimal_separator),
            Self::Pattern { tokens, .. } => {
                let mut result = String::new();
                let mask_text_chars: Vec<char> = mask_text.chars().collect();
                for (text_index, token) in tokens.iter().enumerate() {
                    if text_index >= mask_text_chars.len() {
                        break;
                    }
                    let ch = mask_text_chars[text_index];
                    let unmask_ch = token.unmask_char(ch);
                    if let Some(ch) = unmask_ch {
                        result.push(ch);
                    }
                }
                result
            }
            Self::None => mask_text.to_owned(),
        }
    }

    fn unmask_number(
        mask_text: &str,
        group_separator: Option<char>,
        decimal_separator: char,
    ) -> String {
        if let Some(sep) = group_separator {
            let mut result = String::new();
            for ch in mask_text.chars() {
                if ch == sep {
                    continue;
                }
                result.push(ch);
            }

            if result.contains(decimal_separator) {
                result = result.trim_end_matches('0').to_string();
            }
            return result;
        }

        mask_text.to_owned()
    }
}

#[inline]
fn is_sign(ch: &char) -> bool {
    matches!(ch, '+' | '-')
}

#[cfg(test)]
mod tests {
    use crate::input::mask_pattern::{MaskPattern, MaskToken};

    #[test]
    fn test_is_match() {
        assert_eq!(MaskToken::Sep('(').is_match('('), true);
        assert_eq!(MaskToken::Sep('-').is_match('('), false);
        assert_eq!(MaskToken::Sep('-').is_match('3'), false);

        assert_eq!(MaskToken::Digit.is_match('0'), true);
        assert_eq!(MaskToken::Digit.is_match('9'), true);
        assert_eq!(MaskToken::Digit.is_match('a'), false);
        assert_eq!(MaskToken::Digit.is_match('C'), false);

        assert_eq!(MaskToken::Letter.is_match('a'), true);
        assert_eq!(MaskToken::Letter.is_match('Z'), true);
        assert_eq!(MaskToken::Letter.is_match('Ş'), true);
        assert_eq!(MaskToken::Letter.is_match('ı'), true);
        assert_eq!(MaskToken::Letter.is_match('3'), false);
        assert_eq!(MaskToken::Letter.is_match('-'), false);

        assert_eq!(MaskToken::LetterOrDigit.is_match('0'), true);
        assert_eq!(MaskToken::LetterOrDigit.is_match('9'), true);
        assert_eq!(MaskToken::LetterOrDigit.is_match('a'), true);
        assert_eq!(MaskToken::LetterOrDigit.is_match('Z'), true);
        assert_eq!(MaskToken::LetterOrDigit.is_match('3'), true);

        assert_eq!(MaskToken::Any.is_match('a'), true);
        assert_eq!(MaskToken::Any.is_match('3'), true);
        assert_eq!(MaskToken::Any.is_match('-'), true);
        assert_eq!(MaskToken::Any.is_match(' '), true);
    }

    #[test]
    fn test_mask_none() {
        let mask = MaskPattern::None;
        assert_eq!(mask.is_none(), true);
        assert_eq!(mask.is_valid("1124124ASLDJKljk"), true);
        assert_eq!(mask.mask("hello-world"), "hello-world");
        assert_eq!(mask.unmask("hello-world"), "hello-world");
    }

    #[test]
    fn test_mask_pattern1() {
        let mask = MaskPattern::new("(AA)999-999");
        assert_eq!(
            mask.tokens(),
            Some(&vec![
                MaskToken::Sep('('),
                MaskToken::Letter,
                MaskToken::Letter,
                MaskToken::Sep(')'),
                MaskToken::Digit,
                MaskToken::Digit,
                MaskToken::Digit,
                MaskToken::Sep('-'),
                MaskToken::Digit,
                MaskToken::Digit,
                MaskToken::Digit,
            ])
        );

        assert_eq!(mask.is_valid_at('(', 0), true);
        assert_eq!(mask.is_valid_at('H', 0), true);
        assert_eq!(mask.is_valid_at('3', 0), false);
        assert_eq!(mask.is_valid_at('-', 0), false);
        assert_eq!(mask.is_valid_at(')', 1), false);
        assert_eq!(mask.is_valid_at('H', 1), true);
        assert_eq!(mask.is_valid_at('1', 1), false);
        assert_eq!(mask.is_valid_at('e', 2), true);
        assert_eq!(mask.is_valid_at(')', 3), true);
        assert_eq!(mask.is_valid_at('1', 3), true);
        assert_eq!(mask.is_valid_at('2', 4), true);

        assert_eq!(mask.is_valid("(AB)123-456"), true);

        assert_eq!(mask.mask("AB123456"), "(AB)123-456");
        assert_eq!(mask.mask("(AB)123-456"), "(AB)123-456");
        assert_eq!(mask.mask("(AB123456"), "(AB)123-456");
        assert_eq!(mask.mask("AB123-456"), "(AB)123-456");
        assert_eq!(mask.mask("AB123-"), "(AB)123-");
        assert_eq!(mask.mask("AB123--"), "(AB)123-");
        assert_eq!(mask.mask("AB123-4"), "(AB)123-4");
        assert_eq!(mask.mask("(AB)12-456"), "(AB)124-56");

        let unmasked_text = mask.unmask("(AB)123-456");
        assert_eq!(unmasked_text, "AB123456");

        assert_eq!(mask.is_valid("12AB345"), false);
        assert_eq!(mask.is_valid("(11)123-456"), false);
        assert_eq!(mask.is_valid("##"), false);
        assert_eq!(mask.is_valid("(AB)123456"), true);
    }

    #[test]
    fn test_mask_pattern_display_keeps_remaining_placeholders() {
        let mask = MaskPattern::new("(999)-999-9999");

        let display = mask.display("").unwrap();
        assert_eq!(display.text, "(___)-___-____");
        assert_eq!(
            display.runs,
            vec![crate::input::mask_pattern::MaskDisplayRun {
                range: 0..14,
                is_placeholder: true,
            }]
        );

        let display = mask.display("(53").unwrap();
        assert_eq!(display.text, "(53_)-___-____");
        assert_eq!(
            display.runs,
            vec![
                crate::input::mask_pattern::MaskDisplayRun {
                    range: 0..1,
                    is_placeholder: true,
                },
                crate::input::mask_pattern::MaskDisplayRun {
                    range: 1..3,
                    is_placeholder: false,
                },
                crate::input::mask_pattern::MaskDisplayRun {
                    range: 3..14,
                    is_placeholder: true,
                },
            ]
        );

        let display = mask.display("(538)-697").unwrap();
        assert_eq!(display.text, "(538)-697-____");
        assert_eq!(
            display.runs,
            vec![
                crate::input::mask_pattern::MaskDisplayRun {
                    range: 0..1,
                    is_placeholder: true,
                },
                crate::input::mask_pattern::MaskDisplayRun {
                    range: 1..4,
                    is_placeholder: false,
                },
                crate::input::mask_pattern::MaskDisplayRun {
                    range: 4..6,
                    is_placeholder: true,
                },
                crate::input::mask_pattern::MaskDisplayRun {
                    range: 6..9,
                    is_placeholder: false,
                },
                crate::input::mask_pattern::MaskDisplayRun {
                    range: 9..14,
                    is_placeholder: true,
                },
            ]
        );
    }

    #[test]
    fn test_mask_pattern_reflows_after_middle_delete() {
        let mask = MaskPattern::new("(999)-999-9999");

        assert_eq!(mask.mask("(538)-67-7934"), "(538)-677-934");
        assert_eq!(mask.mask("(538)-6977934"), "(538)-697-7934");
    }

    #[test]
    fn test_mask_pattern_cursor_separator_helpers() {
        let mask = MaskPattern::new("##/##/####");

        assert_eq!(
            mask.append_next_separator_after_cursor("25", 2),
            Some(("25/".to_string(), 3))
        );
        assert_eq!(
            mask.append_next_separator_after_cursor("25/12", 5),
            Some(("25/12/".to_string(), 6))
        );
        assert_eq!(mask.previous_editable_range("25/12/", 6), Some(4..5));
        assert_eq!(mask.previous_editable_range("25/", 3), Some(1..2));
        assert_eq!(mask.next_editable_range("25/12/", 2), Some(3..4));
        assert_eq!(
            mask.trim_trailing_separators("25/"),
            Some(("25".to_string(), 1))
        );
    }

    #[test]
    fn test_mask_pattern_leading_separator_helpers() {
        let mask = MaskPattern::new("(999)-999-9999");

        assert_eq!(mask.first_editable_display_offset(), Some(1));
        assert_eq!(
            mask.append_next_separator_after_cursor("(538", 4),
            Some(("(538)-".to_string(), 6))
        );

        let mask = MaskPattern::new("AAA-###-AAA");
        assert_eq!(mask.first_editable_display_offset(), Some(0));
        assert_eq!(
            mask.append_next_separator_after_cursor("ABC", 3),
            Some(("ABC-".to_string(), 4))
        );
        assert_eq!(
            mask.append_next_separator_after_cursor("ABC-123", 7),
            Some(("ABC-123-".to_string(), 8))
        );
    }

    #[test]
    fn test_mask_pattern2() {
        let mask = MaskPattern::new("999-999-******");
        assert_eq!(
            mask.tokens(),
            Some(&vec![
                MaskToken::Digit,
                MaskToken::Digit,
                MaskToken::Digit,
                MaskToken::Sep('-'),
                MaskToken::Digit,
                MaskToken::Digit,
                MaskToken::Digit,
                MaskToken::Sep('-'),
                MaskToken::Any,
                MaskToken::Any,
                MaskToken::Any,
                MaskToken::Any,
                MaskToken::Any,
                MaskToken::Any,
            ])
        );

        let text = "123456A(111)";
        let masked_text = mask.mask(text);
        assert_eq!(masked_text, "123-456-A(111)");
        let unmasked_text = mask.unmask(&masked_text);
        assert_eq!(unmasked_text, "123456A(111)");
        assert_eq!(mask.is_valid(&masked_text), true);
    }

    #[test]
    fn test_letter_digit_mask_pattern() {
        let mask = MaskPattern::new("AAA-###-AAA");
        assert_eq!(
            mask.tokens(),
            Some(&vec![
                MaskToken::Letter,
                MaskToken::Letter,
                MaskToken::Letter,
                MaskToken::Sep('-'),
                MaskToken::Digit,
                MaskToken::Digit,
                MaskToken::Digit,
                MaskToken::Sep('-'),
                MaskToken::Letter,
                MaskToken::Letter,
                MaskToken::Letter,
            ])
        );

        assert_eq!(mask.is_valid_at('Ş', 0), true);
        assert_eq!(mask.is_valid_at('ı', 1), true);
        assert_eq!(mask.is_valid_at('1', 0), false);
        assert_eq!(mask.is_valid_at('1', 4), true);
        assert_eq!(mask.is_valid_at('A', 4), false);
        assert_eq!(mask.is_valid("Şİa-123-Abı"), true);
        assert_eq!(mask.is_valid("ABC-AB1-DEF"), false);
        assert_eq!(mask.mask("Şİa123Abı"), "Şİa-123-Abı");
    }

    #[test]
    fn test_number_with_group_separator() {
        // Use comma as group separator
        let mask = MaskPattern::number(Some(','));
        assert_eq!(mask.mask("1234567"), "1,234,567");
        assert_eq!(mask.mask("1,234,567"), "1,234,567");
        assert_eq!(mask.unmask("1,234,567"), "1234567");
        let mask = MaskPattern::number(Some(','));
        assert_eq!(mask.mask("1234567.89"), "1,234,567.89");
        assert_eq!(mask.unmask("1,234,567.89"), "1234567.89");

        // Use space as group separator
        let mask = MaskPattern::number(Some(' '));
        assert_eq!(mask.mask("1234567"), "1 234 567");
        assert_eq!(mask.unmask("1 234 567"), "1234567");
        let mask = MaskPattern::number(Some(' '));
        assert_eq!(mask.mask("1234567.89"), "1 234 567.89");
        assert_eq!(mask.unmask("1 234 567.89"), "1234567.89");

        // No group separator
        let mask = MaskPattern::number(None);
        assert_eq!(mask.mask("1234567"), "1234567");
        assert_eq!(mask.unmask("1234567"), "1234567");
        let mask = MaskPattern::number(None);
        assert_eq!(mask.mask("1234567.89"), "1234567.89");
        assert_eq!(mask.unmask("1234567.89"), "1234567.89");
    }

    #[test]
    fn test_number_with_fraction_digits() {
        let mask = MaskPattern::Number {
            separator: Some(','),
            fraction: Some(4),
        };

        assert_eq!(mask.mask("1234567"), "1,234,567");
        assert_eq!(mask.unmask("1,234,567"), "1234567");
        assert_eq!(mask.mask("1234567."), "1,234,567.");
        assert_eq!(mask.mask("1234567.89"), "1,234,567.89");
        assert_eq!(mask.unmask("1,234,567.890"), "1234567.89");
        assert_eq!(mask.mask("1234567.891"), "1,234,567.891");
        assert_eq!(mask.mask("1234567.891234"), "1,234,567.8912");

        let mask = MaskPattern::Number {
            separator: Some(','),
            fraction: None,
        };

        assert_eq!(mask.mask("1234567.1234567"), "1,234,567.1234567");

        let mask = MaskPattern::Number {
            separator: Some(','),
            fraction: Some(0),
        };

        assert_eq!(mask.mask("1234567.1234567"), "1,234,567");
    }

    #[test]
    fn test_localized_number_with_turkish_separators() {
        let mask = MaskPattern::localized_number_for_locale("tr_TR.UTF-8", Some(2));

        assert_eq!(mask.mask("1234567"), "1.234.567");
        assert_eq!(mask.mask("1234567,"), "1.234.567,");
        assert_eq!(mask.mask("1234567,89"), "1.234.567,89");
        assert_eq!(mask.mask("1234567,891"), "1.234.567,89");
        assert_eq!(mask.unmask("1.234.567,890"), "1234567,89");
        assert_eq!(mask.is_valid("-1.234.567,89"), true);
        assert_eq!(mask.is_valid("1,234.56"), false);
    }

    #[test]
    fn test_localized_number_with_english_separators() {
        let mask = MaskPattern::localized_number_for_locale("en_US.UTF-8", Some(2));

        assert_eq!(mask.mask("1234567.89"), "1,234,567.89");
        assert_eq!(mask.unmask("1,234,567.890"), "1234567.89");
    }

    #[test]
    fn test_signed_number_numbers() {
        let mask = MaskPattern::Number {
            separator: Some(','),
            fraction: Some(2),
        };

        assert_eq!(mask.is_valid("-"), true);
        assert_eq!(mask.is_valid("-1234567"), true);
        assert_eq!(mask.is_valid("-1,234,567"), true);
        assert_eq!(mask.is_valid("-1234567."), true);
        assert_eq!(mask.is_valid("-1234567.89"), true);

        assert_eq!(mask.is_valid("+"), true);
        assert_eq!(mask.is_valid("+1234567"), true);
        assert_eq!(mask.is_valid("+1,234,567"), true);
        assert_eq!(mask.is_valid("+1234567."), true);
        assert_eq!(mask.is_valid("+1234567.89"), true);

        // Only one sign is valid
        assert_eq!(mask.is_valid("+-"), false);
        assert_eq!(mask.is_valid("-+"), false);
        assert_eq!(mask.is_valid("+-1234567"), false);

        // No sign is valid in the middle of the number
        assert_eq!(mask.is_valid("1,-234,567"), false);
        assert_eq!(mask.is_valid("12-34567.89"), false);

        // Signs in fractions are invalid
        assert_eq!(mask.is_valid("+1234567.-"), false);

        // The separator does not show up before the sign i.e. -,123
        assert_eq!(mask.mask("-123"), "-123");

        assert_eq!(mask.mask("-1234567"), "-1,234,567");
        assert_eq!(mask.mask("+1234567"), "+1,234,567");
        assert_eq!(mask.unmask("-1,234,567"), "-1234567");
        assert_eq!(mask.mask("-1234567."), "-1,234,567.");
        assert_eq!(mask.mask("-1234567.89"), "-1,234,567.89");
    }
}
