use gpui::{
    AnyElement, App, InteractiveElement as _, IntoElement, ParentElement, Pixels, RenderOnce,
    StyleRefinement, Styled, TextAlign, Window, div, prelude::FluentBuilder as _, px, relative,
};

use crate::{
    AnyChildElement, BilesenBoyutu, Boyutlandirilabilir, ChildElement, EtkinTema as _,
    StilUzantisi as _,
};

const MIN_CELL_WIDTH: Pixels = px(100.);

/// Tablo verisini doğrudan çizmek için temel tablo bileşeni.
///
/// [`VeriTablosu`] aksine bu basit, durumsuz ve bileştirilebilir bir tablodur.
/// olmadan virtual kaydırma veya sütun yönetim.
///
/// [`Boyutlandirilabilir`] ile yapılan boyut ayarları tüm alt öğelere otomatik olarak aktarılır.
///
/// # Örnek
///
/// ```rust,ignore
/// Tablo::new()
///     .small()
///     .child(TabloBasligi::new().child(
///         TabloSatiri::new()
///             .child(TabloBasHucre::new().child("Name"))
///             .child(TabloBasHucre::new().child("Email"))
///     ))
///     .child(TabloGovdesi::new()
///         .child(TabloSatiri::new()
///             .child(TabloHucresi::new().child("John"))
///             .child(TabloHucresi::new().child("john@example.com")))
///     )
///     .child(TabloAciklamasi::new().child("A list of recent invoices."))
/// ```
#[derive(IntoElement)]
pub struct Tablo {
    ix: usize,
    style: StyleRefinement,
    children: Vec<AnyChildElement>,
    size: BilesenBoyutu,
}

impl Tablo {
    pub fn new() -> Self {
        Self {
            ix: 0,
            style: StyleRefinement::default(),
            children: Vec::new(),
            size: BilesenBoyutu::default(),
        }
    }

    pub fn child(mut self, child: impl ChildElement + 'static) -> Self {
        self.children.push(AnyChildElement::new(child));
        self
    }

    pub fn children<E: ChildElement + 'static>(
        mut self,
        children: impl IntoIterator<Item = E>,
    ) -> Self {
        self.children
            .extend(children.into_iter().map(AnyChildElement::new));
        self
    }
}

impl Styled for Tablo {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Boyutlandirilabilir for Tablo {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl ChildElement for Tablo {
    fn with_ix(mut self, ix: usize) -> Self {
        self.ix = ix;
        self
    }
}

impl RenderOnce for Tablo {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(("table", self.ix))
            .w_full()
            .text_sm()
            .overflow_hidden()
            .bg(cx.theme().table)
            .refine_style(&self.style)
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(|(ix, c)| c.into_any(ix, self.size)),
            )
    }
}

/// başlık bölüm bir [`Tablo`], sarma başlık satırlar.
#[derive(IntoElement)]
pub struct TabloBasligi {
    ix: usize,
    style: StyleRefinement,
    children: Vec<AnyChildElement>,
    size: BilesenBoyutu,
}

impl TabloBasligi {
    pub fn new() -> Self {
        Self {
            ix: 0,
            style: StyleRefinement::default(),
            children: Vec::new(),
            size: BilesenBoyutu::default(),
        }
    }

    pub fn child(mut self, child: impl ChildElement + 'static) -> Self {
        self.children.push(AnyChildElement::new(child));
        self
    }

    pub fn children<E: ChildElement + 'static>(
        mut self,
        children: impl IntoIterator<Item = E>,
    ) -> Self {
        self.children
            .extend(children.into_iter().map(AnyChildElement::new));
        self
    }
}

impl Styled for TabloBasligi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ChildElement for TabloBasligi {
    fn with_ix(mut self, ix: usize) -> Self {
        self.ix = ix;
        self
    }
}

impl Boyutlandirilabilir for TabloBasligi {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for TabloBasligi {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(("table-header", self.ix))
            .w_full()
            .bg(cx.theme().table_head)
            .text_color(cx.theme().table_head_foreground)
            .refine_style(&self.style)
            .border_b_1()
            .border_color(cx.theme().table_row_border)
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(|(ix, c)| c.into_any(ix, self.size)),
            )
    }
}

/// gövde bölüm bir [`Tablo`], sarma veri satırlar.
#[derive(IntoElement)]
pub struct TabloGovdesi {
    ix: usize,
    style: StyleRefinement,
    children: Vec<AnyChildElement>,
    size: BilesenBoyutu,
}

impl TabloGovdesi {
    pub fn new() -> Self {
        Self {
            ix: 0,
            style: StyleRefinement::default(),
            children: Vec::new(),
            size: BilesenBoyutu::default(),
        }
    }

    pub fn child(mut self, child: impl ChildElement + 'static) -> Self {
        self.children.push(AnyChildElement::new(child));
        self
    }

    pub fn children<E: ChildElement + 'static>(
        mut self,
        children: impl IntoIterator<Item = E>,
    ) -> Self {
        self.children
            .extend(children.into_iter().map(AnyChildElement::new));
        self
    }
}

impl Styled for TabloGovdesi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Boyutlandirilabilir for TabloGovdesi {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl ChildElement for TabloGovdesi {
    fn with_ix(mut self, ix: usize) -> Self {
        self.ix = ix;
        self
    }
}

impl RenderOnce for TabloGovdesi {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id(("table-body", self.ix))
            .w_full()
            .refine_style(&self.style)
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(|(ix, c)| c.into_any(ix, self.size)),
            )
    }
}

/// alt bilgi bölüm bir [`Tablo`], sarma alt bilgi satırlar.
#[derive(IntoElement)]
pub struct TabloAltligi {
    ix: usize,
    style: StyleRefinement,
    children: Vec<AnyChildElement>,
    size: BilesenBoyutu,
}

impl TabloAltligi {
    pub fn new() -> Self {
        Self {
            ix: 0,
            style: StyleRefinement::default(),
            children: Vec::new(),
            size: BilesenBoyutu::default(),
        }
    }

    pub fn child(mut self, child: impl ChildElement + 'static) -> Self {
        self.children.push(AnyChildElement::new(child));
        self
    }

    pub fn children<E: ChildElement + 'static>(
        mut self,
        children: impl IntoIterator<Item = E>,
    ) -> Self {
        self.children
            .extend(children.into_iter().map(AnyChildElement::new));
        self
    }
}

impl Styled for TabloAltligi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Boyutlandirilabilir for TabloAltligi {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl ChildElement for TabloAltligi {
    fn with_ix(mut self, ix: usize) -> Self {
        self.ix = ix;
        self
    }
}

impl RenderOnce for TabloAltligi {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(("table-footer", self.ix))
            .w_full()
            .bg(cx.theme().table_foot)
            .text_color(cx.theme().table_foot_foreground)
            .border_t_1()
            .border_color(cx.theme().table_row_border)
            .refine_style(&self.style)
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(|(ix, c)| c.into_any(ix, self.size)),
            )
    }
}

/// Bir satır içinde bir [`Tablo`].
#[derive(IntoElement)]
pub struct TabloSatiri {
    ix: usize,
    style: StyleRefinement,
    children: Vec<AnyChildElement>,
    size: BilesenBoyutu,
}

impl TabloSatiri {
    pub fn new() -> Self {
        Self {
            ix: 0,
            style: StyleRefinement::default(),
            children: Vec::new(),
            size: BilesenBoyutu::default(),
        }
    }

    pub fn child(mut self, child: impl ChildElement + 'static) -> Self {
        self.children.push(AnyChildElement::new(child));
        self
    }

    pub fn children<E: ChildElement + 'static>(
        mut self,
        children: impl IntoIterator<Item = E>,
    ) -> Self {
        self.children
            .extend(children.into_iter().map(AnyChildElement::new));
        self
    }
}

impl Styled for TabloSatiri {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Boyutlandirilabilir for TabloSatiri {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl ChildElement for TabloSatiri {
    fn with_ix(mut self, ix: usize) -> Self {
        self.ix = ix;
        self
    }
}

impl RenderOnce for TabloSatiri {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(("table-row", self.ix))
            .w_full()
            .flex()
            .flex_row()
            .refine_style(&self.style)
            .border_color(cx.theme().table_row_border)
            .when(self.ix > 0, |this| this.border_t_1())
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(|(ix, c)| c.into_any(ix, self.size)),
            )
    }
}

/// Bir başlık hücre içinde bir [`TabloSatiri`].
#[derive(IntoElement)]
pub struct TabloBasHucre {
    ix: usize,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    col_span: usize,
    align: TextAlign,
    size: BilesenBoyutu,
}

impl TabloBasHucre {
    pub fn new() -> Self {
        Self {
            ix: 0,
            style: StyleRefinement::default(),
            children: Vec::new(),
            col_span: 1,
            align: TextAlign::Left,
            size: BilesenBoyutu::default(),
        }
    }

    /// Bu başlık hücresinin kaç sütuna yayılacağını ayarlar.
    pub fn col_span(mut self, span: usize) -> Self {
        self.col_span = span.max(1);
        self
    }

    /// Metin hizalamasını ortalar.
    pub fn text_center(mut self) -> Self {
        self.align = TextAlign::Center;
        self
    }

    /// metin hizalama için sağ ayarlar.
    pub fn text_right(mut self) -> Self {
        self.align = TextAlign::Right;
        self
    }
}

impl ParentElement for TabloBasHucre {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Boyutlandirilabilir for TabloBasHucre {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl ChildElement for TabloBasHucre {
    fn with_ix(mut self, ix: usize) -> Self {
        self.ix = ix;
        self
    }
}

impl Styled for TabloBasHucre {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TabloBasHucre {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let paddings = self.size.table_cell_padding();

        div()
            .id(("table-head", self.ix))
            .flex()
            .items_center()
            .when(self.style.size.width.is_none(), |this| {
                this.flex_shrink()
                    .flex_basis(relative(self.col_span as f32))
            })
            .min_w(MIN_CELL_WIDTH * self.col_span)
            .px(paddings.left)
            .py(paddings.top)
            .when(self.align == TextAlign::Center, |this| {
                this.justify_center()
            })
            .when(self.align == TextAlign::Right, |this| this.justify_end())
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Bir veri hücre içinde bir [`TabloSatiri`].
#[derive(IntoElement)]
pub struct TabloHucresi {
    ix: usize,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    col_span: usize,
    align: TextAlign,
    size: BilesenBoyutu,
}

impl TabloHucresi {
    pub fn new() -> Self {
        Self {
            ix: 0,
            style: StyleRefinement::default(),
            children: Vec::new(),
            col_span: 1,
            align: TextAlign::Left,
            size: BilesenBoyutu::default(),
        }
    }

    /// Bu hücrenin kaç sütuna yayılacağını ayarlar.
    pub fn col_span(mut self, span: usize) -> Self {
        self.col_span = span.max(1);
        self
    }

    /// Metin hizalamasını ortalar.
    pub fn text_center(mut self) -> Self {
        self.align = TextAlign::Center;
        self
    }

    /// metin hizalama için sağ ayarlar.
    pub fn text_right(mut self) -> Self {
        self.align = TextAlign::Right;
        self
    }
}

impl ParentElement for TabloHucresi {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Boyutlandirilabilir for TabloHucresi {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl ChildElement for TabloHucresi {
    fn with_ix(mut self, ix: usize) -> Self {
        self.ix = ix;
        self
    }
}

impl Styled for TabloHucresi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TabloHucresi {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let paddings = self.size.table_cell_padding();

        div()
            .id(("table-cell", self.ix))
            .flex()
            .items_center()
            .when(self.style.size.width.is_none(), |this| {
                this.flex_shrink()
                    .flex_basis(relative(self.col_span as f32))
            })
            .min_w(MIN_CELL_WIDTH * self.col_span)
            .px(paddings.left)
            .py(paddings.top)
            .when(self.align == TextAlign::Center, |this| {
                this.justify_center()
            })
            .when(self.align == TextAlign::Right, |this| this.justify_end())
            .refine_style(&self.style)
            .children(self.children)
    }
}

/// Bir caption gösterilen below [`Tablo`].
#[derive(IntoElement)]
pub struct TabloAciklamasi {
    ix: usize,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    size: BilesenBoyutu,
}

impl TabloAciklamasi {
    pub fn new() -> Self {
        Self {
            ix: 0,
            style: StyleRefinement::default(),
            children: Vec::new(),
            size: BilesenBoyutu::default(),
        }
    }
}

impl ParentElement for TabloAciklamasi {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Boyutlandirilabilir for TabloAciklamasi {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl ChildElement for TabloAciklamasi {
    fn with_ix(mut self, ix: usize) -> Self {
        self.ix = ix;
        self
    }
}

impl Styled for TabloAciklamasi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for TabloAciklamasi {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let paddings = self.size.table_cell_padding();

        div()
            .id(("table-caption", self.ix))
            .w_full()
            .px(paddings.left)
            .py(paddings.top)
            .text_sm()
            .text_color(cx.theme().muted_foreground)
            .text_center()
            .refine_style(&self.style)
            .children(self.children)
    }
}
