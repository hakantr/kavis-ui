use crate::{BilesenBoyutu, Boyutlandirilabilir, RenkAdi, StilUzantisi, theme::EtkinTema as _};
use gpui::{
    AbsoluteLength, AnyElement, App, Hsla, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, relative, rems,
    transparent_white,
};

/// varyant Cip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CipVaryanti {
    Primary,
    #[default]
    Secondary,
    Danger,
    Success,
    Warning,
    Info,
    Color(RenkAdi),
    Custom {
        color: Hsla,
        foreground: Hsla,
        border: Hsla,
    },
}

impl CipVaryanti {
    fn bg(&self, cx: &App) -> Hsla {
        match self {
            Self::Primary => cx.theme().primary,
            Self::Secondary => cx.theme().secondary,
            Self::Danger => cx.theme().danger,
            Self::Success => cx.theme().success,
            Self::Warning => cx.theme().warning,
            Self::Info => cx.theme().info,
            Self::Color(color) => {
                if cx.theme().is_dark() {
                    color.scale(950).opacity(0.5)
                } else {
                    color.scale(50)
                }
            }
            Self::Custom { color, .. } => *color,
        }
    }

    fn border(&self, cx: &App) -> Hsla {
        match self {
            Self::Primary => cx.theme().primary,
            Self::Secondary => cx.theme().border,
            Self::Danger => cx.theme().danger,
            Self::Success => cx.theme().success,
            Self::Warning => cx.theme().warning,
            Self::Info => cx.theme().info,
            Self::Color(color) => {
                if cx.theme().is_dark() {
                    color.scale(800).opacity(0.5)
                } else {
                    color.scale(200)
                }
            }
            Self::Custom { border, .. } => *border,
        }
    }

    fn fg(&self, outline: bool, cx: &App) -> Hsla {
        match self {
            Self::Primary => {
                if outline {
                    cx.theme().primary
                } else {
                    cx.theme().primary_foreground
                }
            }
            Self::Secondary => {
                if outline {
                    cx.theme().muted_foreground
                } else {
                    cx.theme().secondary_foreground
                }
            }
            Self::Danger => {
                if outline {
                    cx.theme().danger
                } else {
                    cx.theme().danger_foreground
                }
            }
            Self::Success => {
                if outline {
                    cx.theme().success
                } else {
                    cx.theme().success_foreground
                }
            }
            Self::Warning => {
                if outline {
                    cx.theme().warning
                } else {
                    cx.theme().warning_foreground
                }
            }
            Self::Info => {
                if outline {
                    cx.theme().info
                } else {
                    cx.theme().info_foreground
                }
            }
            Self::Color(color) => {
                if cx.theme().is_dark() {
                    color.scale(300)
                } else {
                    color.scale(600)
                }
            }
            Self::Custom { foreground, .. } => *foreground,
        }
    }
}

/// Cip, küçük bir durum göstergesidir.
///
/// Yalnızca destek: Medium, Small
#[derive(IntoElement)]
pub struct Cip {
    style: StyleRefinement,
    variant: CipVaryanti,
    outline: bool,
    size: BilesenBoyutu,
    rounded: Option<AbsoluteLength>,
    children: Vec<AnyElement>,
}
impl Cip {
    /// Yeni bir Cip oluşturur.
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            variant: CipVaryanti::default(),
            outline: false,
            size: BilesenBoyutu::default(),
            rounded: None,
            children: Vec::new(),
        }
    }

    /// Yeni bir etiket ile varsayılan varyant ([`CipVaryanti::Primary`]) oluşturur.
    pub fn primary() -> Self {
        Self::new().with_variant(CipVaryanti::Primary)
    }

    /// Yeni bir etiket ile varsayılan varyant ([`CipVaryanti::Secondary`]) oluşturur.
    pub fn secondary() -> Self {
        Self::new().with_variant(CipVaryanti::Secondary)
    }

    /// Yeni bir etiket ile varsayılan varyant ([`CipVaryanti::Danger`]) oluşturur.
    pub fn danger() -> Self {
        Self::new().with_variant(CipVaryanti::Danger)
    }

    /// Yeni bir etiket ile varsayılan varyant ([`CipVaryanti::Success`]) oluşturur.
    pub fn success() -> Self {
        Self::new().with_variant(CipVaryanti::Success)
    }

    /// Yeni bir etiket ile varsayılan varyant ([`CipVaryanti::Warning`]) oluşturur.
    pub fn warning() -> Self {
        Self::new().with_variant(CipVaryanti::Warning)
    }

    /// Yeni bir etiket ile varsayılan varyant ([`CipVaryanti::Info`]) oluşturur.
    pub fn info() -> Self {
        Self::new().with_variant(CipVaryanti::Info)
    }

    /// Yeni bir etiket ile varsayılan varyant ([`CipVaryanti::özel`]) oluşturur.
    pub fn custom(color: Hsla, foreground: Hsla, border: Hsla) -> Self {
        Self::new().with_variant(CipVaryanti::Custom {
            color,
            foreground,
            border,
        })
    }

    /// Yeni bir etiket ile varsayılan varyant ([`CipVaryanti::renk`]) oluşturur.
    pub fn color(color: impl Into<RenkAdi>) -> Self {
        Self::new().with_variant(CipVaryanti::Color(color.into()))
    }

    /// varyant Cip ayarlar.
    pub fn with_variant(mut self, variant: CipVaryanti) -> Self {
        self.variant = variant;
        self
    }

    /// Kullanım çerçeve stil
    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    /// rounded corners ayarlar.
    pub fn rounded(mut self, radius: impl Into<AbsoluteLength>) -> Self {
        self.rounded = Some(radius.into());
        self
    }

    /// Tam yuvarlak görünümü ayarlar.
    pub fn rounded_full(mut self) -> Self {
        self.rounded = Some(rems(1.).into());
        self
    }
}

impl Boyutlandirilabilir for Cip {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl ParentElement for Cip {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Cip {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Cip {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let bg = if self.outline {
            transparent_white()
        } else {
            self.variant.bg(cx)
        };
        let fg = self.variant.fg(self.outline, cx);
        let border = self.variant.border(cx);
        let rounded = self.rounded.unwrap_or(
            match self.size {
                BilesenBoyutu::CokKucuk | BilesenBoyutu::Kucuk => cx.theme().radius / 2.,
                _ => cx.theme().radius,
            }
            .into(),
        );

        div()
            .flex()
            .items_center()
            .border_1()
            .line_height(relative(1.))
            .text_xs()
            .map(|this| match self.size {
                BilesenBoyutu::CokKucuk | BilesenBoyutu::Kucuk => this.px_1p5().py_0p5(),
                _ => this.px_2p5().py_1(),
            })
            .bg(bg)
            .text_color(fg)
            .border_color(border)
            .rounded(rounded)
            .hover(|this| this.opacity(0.9))
            .refine_style(&self.style)
            .children(self.children)
    }
}
