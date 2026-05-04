use gpui::{
    AnyElement, App, Axis, DefiniteLength, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled, Window, div, prelude::FluentBuilder as _, px, relative,
};

use crate::{
    AxisExt, BilesenBoyutu, Boyutlandirilabilir, EtkinTema as _, h_flex, text::Text, v_flex,
};

/// Bir açıklama liste.
#[derive(IntoElement)]
pub struct AciklamaListesi {
    items: Vec<AciklamaOgesi>,
    size: BilesenBoyutu,
    layout: Axis,
    label_width: DefiniteLength,
    bordered: bool,
    columns: usize,
}

/// öğe için [`AciklamaListesi`].
pub enum AciklamaOgesi {
    Item {
        label: AciklamaMetni,
        value: AciklamaMetni,
        span: usize,
    },
    Ayirici,
}

/// metin için etiket veya değer içinde [`AciklamaListesi`].
#[derive(IntoElement)]
pub enum AciklamaMetni {
    String(SharedString),
    Text(Text),
    AnyElement(AnyElement),
}

impl From<&str> for AciklamaMetni {
    fn from(text: &str) -> Self {
        AciklamaMetni::String(SharedString::from(text.to_string()))
    }
}

impl From<Text> for AciklamaMetni {
    fn from(text: Text) -> Self {
        AciklamaMetni::Text(text)
    }
}

impl From<AnyElement> for AciklamaMetni {
    fn from(element: AnyElement) -> Self {
        AciklamaMetni::AnyElement(element)
    }
}

impl From<SharedString> for AciklamaMetni {
    fn from(text: SharedString) -> Self {
        AciklamaMetni::String(text)
    }
}

impl From<String> for AciklamaMetni {
    fn from(text: String) -> Self {
        AciklamaMetni::String(SharedString::from(text))
    }
}

impl RenderOnce for AciklamaMetni {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        match self {
            AciklamaMetni::String(text) => div().child(text).into_any_element(),
            AciklamaMetni::Text(text) => text.into_any_element(),
            AciklamaMetni::AnyElement(element) => element,
        }
    }
}

impl AciklamaOgesi {
    /// Yeni bir açıklama öğe, ile bir etiket oluşturur.
    ///
    /// Değer boş bir öğedir.
    pub fn new(label: impl Into<AciklamaMetni>) -> Self {
        AciklamaOgesi::Item {
            label: label.into(),
            value: "".into(),
            span: 1,
        }
    }

    /// öğe değer öğe ayarlar.
    pub fn value(mut self, value: impl Into<AciklamaMetni>) -> Self {
        let new_value = value.into();
        if let AciklamaOgesi::Item { value, .. } = &mut self {
            *value = new_value;
        }
        self
    }

    /// Öğe genişliğini ayarlar.
    ///
    /// Bu yöntem yalnızca çalışır için [`AciklamaOgesi::öğe`].
    pub fn span(mut self, span: usize) -> Self {
        let val = span;
        if let AciklamaOgesi::Item { span, .. } = &mut self {
            *span = val;
        }
        self
    }

    fn _label(&self) -> Option<&AciklamaMetni> {
        match self {
            AciklamaOgesi::Item { label, .. } => Some(label),
            _ => None,
        }
    }

    fn _span(&self) -> Option<usize> {
        match self {
            AciklamaOgesi::Item { span, .. } => Some(*span),
            _ => None,
        }
    }
}

impl AciklamaListesi {
    /// Varsayılan yatay yerleşimle yeni bir açıklama listesi oluşturur.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            layout: Axis::Horizontal,
            label_width: px(120.).into(),
            size: BilesenBoyutu::default(),
            bordered: true,
            columns: 3,
        }
    }

    /// Bir dikey açıklama liste. oluşturur.
    pub fn vertical() -> Self {
        Self::new().layout(Axis::Vertical)
    }

    /// Bir yatay açıklama liste, varsayılan. oluşturur.
    pub fn horizontal() -> Self {
        Self::new().layout(Axis::Horizontal)
    }

    /// genişlik etiket, yalnızca çalışır için yatay yerleşim ayarlar.
    ///
    /// Varsayılan `120px` değeridir.
    pub fn label_width(mut self, label_width: impl Into<DefiniteLength>) -> Self {
        self.label_width = label_width.into();
        self
    }

    /// yerleşim açıklama liste ayarlar.
    pub fn layout(mut self, layout: Axis) -> Self {
        self.layout = layout;
        self
    }

    /// Açıklama listesinin kenarlığını ayarlar. Varsayılan `true` değeridir.
    ///
    /// Yalnızca `Horizontal` yerleşimde kullanılır.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Açıklama listesindeki sütun sayısını ayarlar. Varsayılan `3` değeridir.
    ///
    /// `1` ile `10` arasında bir değere izin verilir.
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.clamp(1, 10);
        self
    }

    /// Bir [`AciklamaOgesi::öğe`] için liste ekler.
    pub fn item(
        mut self,
        label: impl Into<AciklamaMetni>,
        value: impl Into<AciklamaMetni>,
        span: usize,
    ) -> Self {
        self.items.push(AciklamaOgesi::Item {
            label: label.into(),
            value: value.into(),
            span,
        });
        self
    }

    /// Bir alt için liste ekler.
    pub fn child(mut self, child: impl Into<AciklamaOgesi>) -> Self {
        self.items.push(child.into());
        self
    }

    /// alt öğeler için liste. ekler.
    pub fn children(
        mut self,
        children: impl IntoIterator<Item = impl Into<AciklamaOgesi>>,
    ) -> Self {
        self.items
            .extend(children.into_iter().map(Into::into).collect::<Vec<_>>());
        self
    }

    /// Bir divider için liste ekler.
    pub fn divider(mut self) -> Self {
        self.items.push(AciklamaOgesi::Ayirici);
        self
    }

    fn group_item_rows(items: Vec<AciklamaOgesi>, columns: usize) -> Vec<Vec<AciklamaOgesi>> {
        let mut rows = vec![];
        let mut current_span = 0;
        for item in items.into_iter() {
            let span = item._span().unwrap_or(columns);
            if rows.is_empty() {
                rows.push(vec![]);
            }
            if current_span + span > columns {
                rows.push(vec![]);
                current_span = 0;
            }
            let last_group = rows.last_mut().unwrap();
            last_group.push(item);
            current_span += span;
        }
        // Remove last empty rows if it exists
        while let Some(last_group) = rows.last() {
            if !last_group.is_empty() {
                break;
            }

            rows.pop();
        }

        rows
    }
}

impl Boyutlandirilabilir for AciklamaListesi {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl RenderOnce for AciklamaListesi {
    fn render(self, _: &mut Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let base_gap = match self.size {
            BilesenBoyutu::CokKucuk | BilesenBoyutu::Kucuk => px(2.),
            BilesenBoyutu::Orta => px(4.),
            BilesenBoyutu::Buyuk => px(8.),
            _ => px(4.),
        };

        // Only for Horizontal layout
        let (mut padding_x, mut padding_y) = match self.size {
            BilesenBoyutu::CokKucuk | BilesenBoyutu::Kucuk => (px(4.), px(2.)),
            BilesenBoyutu::Orta => (px(8.), px(4.)),
            BilesenBoyutu::Buyuk => (px(12.), px(6.)),
            _ => (px(8.), px(4.)),
        };

        let label_width = if self.layout.is_horizontal() {
            Some(self.label_width)
        } else {
            None
        };
        if !self.bordered {
            padding_x = px(0.);
            padding_y = px(0.);
        }
        let gap = if self.bordered { px(0.) } else { base_gap };

        // Group items by columns
        let rows = Self::group_item_rows(self.items, self.columns);
        let rows_len = rows.len();

        v_flex()
            .gap(gap)
            .overflow_hidden()
            .when(self.bordered, |this| {
                this.rounded(padding_x)
                    .border_1()
                    .border_color(cx.theme().border)
            })
            .children(rows.into_iter().enumerate().map(|(ix, items)| {
                let is_last = ix == rows_len - 1;
                h_flex()
                    .when(self.bordered && !is_last, |this| {
                        this.border_b_1().border_color(cx.theme().border)
                    })
                    .children({
                        items.into_iter().enumerate().map(|(item_ix, item)| {
                            let is_first_col = item_ix == 0;

                            match item {
                                AciklamaOgesi::Item { label, value, span } => {
                                    let el = if self.layout.is_vertical() {
                                        v_flex()
                                    } else {
                                        div().flex().flex_row().h_full()
                                    };

                                    el.flex_1()
                                        .flex_basis(relative((span as f32) / (self.columns as f32)))
                                        .overflow_x_hidden()
                                        .child(
                                            div()
                                                .when(self.layout.is_horizontal(), |this| {
                                                    this.h_full()
                                                })
                                                .text_color(
                                                    cx.theme().description_list_label_foreground,
                                                )
                                                .text_sm()
                                                .px(padding_x)
                                                .py(padding_y)
                                                .when(self.bordered, |this| {
                                                    this.when(self.layout.is_horizontal(), |this| {
                                                        this.border_r_1()
                                                            .when(!is_first_col, |this| {
                                                                this.border_l_1()
                                                            })
                                                    })
                                                    .when(self.layout.is_vertical(), |this| {
                                                        this.border_b_1()
                                                    })
                                                    .border_color(cx.theme().border)
                                                    .bg(cx.theme().description_list_label)
                                                })
                                                .map(|this| match label_width {
                                                    Some(label_width) => {
                                                        this.w(label_width).flex_shrink_0()
                                                    }
                                                    None => this,
                                                })
                                                .child(label),
                                        )
                                        .child(
                                            div()
                                                .flex_1()
                                                .px(padding_x)
                                                .py(padding_y)
                                                .overflow_hidden()
                                                .child(value),
                                        )
                                }
                                _ => div().h_2().w_full().when(self.bordered, |this| {
                                    this.bg(cx.theme().description_list_label)
                                }),
                            }
                        })
                    })
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::AciklamaOgesi;

    #[test]
    fn test_group_item_rows() {
        let items = vec![
            AciklamaOgesi::new("test1"),
            AciklamaOgesi::new("test2").span(2),
            AciklamaOgesi::new("test3"),
            AciklamaOgesi::new("test4"),
            AciklamaOgesi::new("test5"),
            AciklamaOgesi::new("test6").span(3),
            AciklamaOgesi::new("test7"),
        ];
        let rows = super::AciklamaListesi::group_item_rows(items, 3);
        assert_eq!(rows.len(), 4);
        assert_eq!(rows[0].len(), 2);
        assert_eq!(rows[1].len(), 3);
        assert_eq!(rows[2].len(), 1);
        assert_eq!(rows[3].len(), 1);
    }
}
