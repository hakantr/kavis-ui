use std::{
    cmp::Ordering,
    ops::{Deref, Range},
    usize,
};

use gpui::{App, HighlightStyle, Hsla, SharedString, UnderlineStyle, px};
use ropey::Rope;
use sum_tree::{Bias, SeekTarget, SumTree};

use crate::{
    EtkinTema,
    input::{Position, RopeExt as _},
};

pub type DiagnosticRelatedInformation = lsp_types::DiagnosticRelatedInformation;
pub type CodeDescription = lsp_types::CodeDescription;
pub type RelatedInformation = lsp_types::DiagnosticRelatedInformation;
pub type DiagnosticTag = lsp_types::DiagnosticTag;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Tani {
    /// Mesajın uygulandığı [`konum`] aralığı.
    ///
    /// Bu sütun, karakter aralık içinde tek satır.
    pub range: Range<Position>,

    /// Tanılamanın önem derecesi. Atlanabilir; atlanırsa
    /// client için interpret tanılamalar olarak hata, uyarı, bilgi veya hint.
    pub severity: TaniOnemi,

    /// tanılama's kod. Olabilir olmak omitted.
    pub code: Option<SharedString>,

    pub code_description: Option<CodeDescription>,

    /// Bir insan-readable metin describing kaynak bu
    /// tanılama, e.g. 'typescript' veya 'super lint'.
    pub source: Option<SharedString>,

    /// tanılama's mesaj.
    pub message: SharedString,

    /// Bir array related tanılama bilgi, e.g. olduğunda symbol-adlar içinde
    /// bir scope collide tüm tanımlar olabilir marked aracılığıyla bu property.
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,

    /// ek metadata about tanılama.
    pub tags: Option<Vec<DiagnosticTag>>,

    /// Bir veri entry alan olan preserved arasında bir `textDocument/publishDiagnostics`
    /// bildirim ve `textDocument/codeAction` istek.
    ///
    /// @since 3.16.0
    pub data: Option<serde_json::Value>,
}

impl From<lsp_types::Diagnostic> for Tani {
    fn from(value: lsp_types::Diagnostic) -> Self {
        Self {
            range: value.range.start..value.range.end,
            severity: value.severity.map(Into::into).unwrap_or(TaniOnemi::Info),
            code: value.code.map(|c| match c {
                lsp_types::NumberOrString::Number(n) => SharedString::from(n.to_string()),
                lsp_types::NumberOrString::String(s) => SharedString::from(s),
            }),
            code_description: value.code_description,
            source: value.source.map(|s| s.into()),
            message: value.message.into(),
            related_information: value.related_information,
            tags: value.tags,
            data: value.data,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaniOnemi {
    #[default]
    Hint,
    Error,
    Warning,
    Info,
}

impl From<lsp_types::DiagnosticSeverity> for TaniOnemi {
    fn from(value: lsp_types::DiagnosticSeverity) -> Self {
        match value {
            lsp_types::DiagnosticSeverity::ERROR => Self::Error,
            lsp_types::DiagnosticSeverity::WARNING => Self::Warning,
            lsp_types::DiagnosticSeverity::INFORMATION => Self::Info,
            lsp_types::DiagnosticSeverity::HINT => Self::Hint,
            _ => Self::Info, // Default to Info if unknown
        }
    }
}

impl TaniOnemi {
    pub(crate) fn bg(&self, cx: &App) -> Hsla {
        let theme = &cx.theme().highlight_theme;

        match self {
            Self::Error => theme.style.status.error_background(cx),
            Self::Warning => theme.style.status.warning_background(cx),
            Self::Info => theme.style.status.info_background(cx),
            Self::Hint => theme.style.status.hint_background(cx),
        }
    }

    pub(crate) fn fg(&self, cx: &App) -> Hsla {
        let theme = &cx.theme().highlight_theme;

        match self {
            Self::Error => theme.style.status.error(cx),
            Self::Warning => theme.style.status.warning(cx),
            Self::Info => theme.style.status.info(cx),
            Self::Hint => theme.style.status.hint(cx),
        }
    }

    pub(crate) fn border(&self, cx: &App) -> Hsla {
        let theme = &cx.theme().highlight_theme;
        match self {
            Self::Error => theme.style.status.error_border(cx),
            Self::Warning => theme.style.status.warning_border(cx),
            Self::Info => theme.style.status.info_border(cx),
            Self::Hint => theme.style.status.hint_border(cx),
        }
    }

    pub(crate) fn highlight_style(&self, cx: &App) -> HighlightStyle {
        let theme = &cx.theme().highlight_theme;

        let color = match self {
            Self::Error => Some(theme.style.status.error(cx)),
            Self::Warning => Some(theme.style.status.warning(cx)),
            Self::Info => Some(theme.style.status.info(cx)),
            Self::Hint => Some(theme.style.status.hint(cx)),
        };

        let mut style = HighlightStyle::default();
        style.underline = Some(UnderlineStyle {
            color: color,
            thickness: px(1.),
            wavy: true,
        });

        style
    }
}

impl Tani {
    pub fn new(range: Range<impl Into<Position>>, message: impl Into<SharedString>) -> Self {
        Self {
            range: range.start.into()..range.end.into(),
            message: message.into(),
            ..Default::default()
        }
    }

    pub fn with_severity(mut self, severity: impl Into<TaniOnemi>) -> Self {
        self.severity = severity.into();
        self
    }

    pub fn with_code(mut self, code: impl Into<SharedString>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_source(mut self, source: impl Into<SharedString>) -> Self {
        self.source = Some(source.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct TaniGirdisi {
    /// bayt aralık tanılama içinde rope.
    pub range: Range<usize>,
    pub diagnostic: Tani,
}

impl Deref for TaniGirdisi {
    type Target = Tani;

    fn deref(&self) -> &Self::Target {
        &self.diagnostic
    }
}

#[derive(Debug, Default, Clone)]
pub struct TaniOzeti {
    count: usize,
    start: usize,
    end: usize,
}

impl sum_tree::Item for TaniGirdisi {
    type Summary = TaniOzeti;
    fn summary(&self, _cx: &()) -> Self::Summary {
        TaniOzeti {
            count: 1,
            start: self.range.start,
            end: self.range.end,
        }
    }
}

impl sum_tree::Summary for TaniOzeti {
    type Context<'a> = &'a ();
    fn zero(_: Self::Context<'_>) -> Self {
        TaniOzeti {
            count: 0,
            start: usize::MIN,
            end: usize::MIN,
        }
    }

    fn add_summary(&mut self, other: &Self, _: Self::Context<'_>) {
        self.start = other.start;
        self.end = other.end;
        self.count += other.count;
    }
}

/// For seeking ile bayt aralık.
impl SeekTarget<'_, TaniOzeti, TaniOzeti> for usize {
    fn cmp(&self, other: &TaniOzeti, _: &()) -> Ordering {
        if *self < other.start {
            Ordering::Less
        } else if *self > other.end {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaniKumesi {
    text: Rope,
    diagnostics: SumTree<TaniGirdisi>,
}

impl TaniKumesi {
    pub fn new(text: &Rope) -> Self {
        Self {
            text: text.clone(),
            diagnostics: SumTree::new(&()),
        }
    }

    pub fn reset(&mut self, text: &Rope) {
        self.text = text.clone();
        self.clear();
    }

    pub fn push(&mut self, diagnostic: impl Into<Tani>) {
        let diagnostic = diagnostic.into();
        let start = self.text.position_to_offset(&diagnostic.range.start);
        let end = self.text.position_to_offset(&diagnostic.range.end);

        self.diagnostics.push(
            TaniGirdisi {
                range: start..end,
                diagnostic,
            },
            &(),
        );
    }

    pub fn extend<D, I>(&mut self, diagnostics: D)
    where
        D: IntoIterator<Item = I>,
        I: Into<Tani>,
    {
        for diagnostic in diagnostics {
            self.push(diagnostic.into());
        }
    }

    pub fn len(&self) -> usize {
        self.diagnostics.summary().count
    }

    pub fn clear(&mut self) {
        self.diagnostics = SumTree::new(&());
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub(crate) fn range(&self, range: Range<usize>) -> impl Iterator<Item = &TaniGirdisi> {
        let mut cursor = self.diagnostics.cursor::<TaniOzeti>(&());
        cursor.seek(&range.start, Bias::Left);
        std::iter::from_fn(move || {
            if let Some(entry) = cursor.item() {
                if entry.range.start < range.end {
                    cursor.next();
                    return Some(entry);
                }
            }
            None
        })
    }

    pub(crate) fn for_offset(&self, offset: usize) -> Option<&TaniGirdisi> {
        self.range(offset..offset + 1).next()
    }

    pub(crate) fn styles_for_range(
        &self,
        range: &Range<usize>,
        cx: &App,
    ) -> Vec<(Range<usize>, HighlightStyle)> {
        if self.diagnostics.is_empty() {
            return vec![];
        }

        let mut styles = vec![];
        for entry in self.range(range.clone()) {
            let range = entry.range.clone();
            styles.push((range, entry.diagnostic.severity.highlight_style(cx)));
        }

        styles
    }

    #[allow(unused)]
    pub(crate) fn iter(&self) -> impl Iterator<Item = &TaniGirdisi> {
        self.diagnostics.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::Position;

    #[test]
    fn test_diagnostic() {
        use ropey::Rope;

        use super::{Tani, TaniKumesi, TaniOnemi};

        let text = Rope::from("Hello, 你好warld!\nThis is a test.\nGoodbye, world!");
        let mut diagnostics = TaniKumesi::new(&text);

        diagnostics.push(
            Tani::new(
                Position::new(0, 7)..Position::new(0, 17),
                "Spelling mistake",
            )
            .with_severity(TaniOnemi::Warning),
        );
        diagnostics.push(
            Tani::new(Position::new(2, 9)..Position::new(2, 14), "Syntax error")
                .with_severity(TaniOnemi::Error),
        );

        assert_eq!(diagnostics.len(), 2);
        let items = diagnostics.iter().collect::<Vec<_>>();

        assert_eq!(items[0].message.as_str(), "Spelling mistake");
        assert_eq!(items[0].range, 7..19);

        assert_eq!(items[1].message.as_str(), "Syntax error");
        assert_eq!(items[1].range, 45..50);

        let items = diagnostics.range(6..48).collect::<Vec<_>>();
        assert_eq!(items.len(), 2);

        let item = diagnostics.for_offset(10).unwrap();
        assert_eq!(item.message.as_str(), "Spelling mistake");

        let item = diagnostics.for_offset(30);
        assert!(item.is_none());

        let item = diagnostics.for_offset(46).unwrap();
        assert_eq!(item.message.as_str(), "Syntax error");

        diagnostics.push(
            Tani::new(Position::new(1, 5)..Position::new(1, 7), "Info message")
                .with_severity(TaniOnemi::Info),
        );
        assert_eq!(diagnostics.len(), 3);

        diagnostics.clear();
        assert_eq!(diagnostics.len(), 0);
    }
}
