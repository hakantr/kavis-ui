use std::f32;

use gpui::{
    Bounds, Context, Edges, Empty, EntityId, IntoElement, ParentElement as _, Pixels, Render,
    SharedString, Styled as _, TextAlign, Window, div, prelude::FluentBuilder, px,
};

use crate::EtkinTema as _;

/// Tablodaki bir sütunu temsil eder ve tablo sütunlarını başlatmak için kullanılır.
#[derive(Debug, Clone)]
pub struct Column {
    /// unique key sütun.
    ///
    /// Bu için kullanılır identify sütun içinde tablo ve your veri kaynak.
    ///
    /// Çoğu durumda veri kaynağınızdaki alan adıyla eşleşmelidir.
    pub key: SharedString,
    /// gösterim ad sütun.
    pub name: SharedString,
    /// metin hizalama sütun.
    pub align: TextAlign,
    /// sıralama davranış sütun, ise herhangi bir.
    ///
    /// `None` ise sütun sıralanamaz.
    pub sort: Option<ColumnSort>,
    /// dolgu sütun.
    pub paddings: Option<Edges<Pixels>>,
    /// genişlik sütun.
    pub width: Pixels,
    /// Sütunun sabit olup olmadığını belirtir. Sabit sütun yatay kaydırmada sol tarafa sabitlenir.
    pub fixed: Option<ColumnFixed>,
    /// Sütunun yeniden boyutlandırılabilir olup olmadığını belirtir.
    pub resizable: bool,
    /// Sütunun taşınabilir olup olmadığını belirtir.
    pub movable: bool,
    /// Sütunun seçilebilir olup olmadığını belirtir.
    ///
    /// Olduğunda `true`:
    /// - In sütun seçim mod: entire sütun olabilir seçili
    /// - In hücre seçim mod: Individual hücreler içinde bu sütun olabilir seçili
    ///
    /// Olduğunda `false`:
    /// - sütun ve onun hücreler olamaz seçili
    /// - Seçime katılmaması gereken eylem sütunları (ör. düğmeler, onay kutuları) için kullanışlıdır.
    pub selectable: bool,
    /// minimum genişlik sütun.
    pub min_width: Pixels,
    /// en yüksek genişlik sütun.
    pub max_width: Pixels,
}

/// Bir sütun grup olabilir için kullanılır grup çoklu sütunlar altında tek başlık.
#[derive(Debug, Clone)]
pub struct ColumnGroup {
    pub label: SharedString,
    pub span: usize,
}

impl ColumnGroup {
    pub fn new(label: impl Into<SharedString>, span: usize) -> Self {
        Self {
            label: label.into(),
            span,
        }
    }
}

impl Default for Column {
    fn default() -> Self {
        Self {
            key: SharedString::new(""),
            name: SharedString::new(""),
            align: TextAlign::Left,
            sort: None,
            paddings: None,
            width: px(100.),
            fixed: None,
            resizable: true,
            movable: true,
            selectable: true,
            min_width: px(20.0),
            max_width: px(f32::MAX),
        }
    }
}

impl Column {
    /// Yeni bir sütun ile verilen key ve ad oluşturur.
    pub fn new(key: impl Into<SharedString>, name: impl Into<SharedString>) -> Self {
        Self {
            key: key.into(),
            name: name.into(),
            ..Default::default()
        }
    }

    /// Özel sıralama fonksiyonuyla sütunu sıralanabilir yapar. Varsayılan None değeridir (sıralanamaz).
    ///
    /// Bakınız ayrıca [`Column::sortable`] için etkinleştirir sıralama ile varsayılan.
    pub fn sort(mut self, sort: ColumnSort) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Sütunun sıralanabilir olup olmadığını ayarlar. Varsayılan true değeridir.
    ///
    /// Bakınız ayrıca [`Column::sort`].
    pub fn sortable(mut self) -> Self {
        self.sort = Some(ColumnSort::Default);
        self
    }

    /// Sütunun artan sırayla sıralanıp sıralanmayacağını ayarlar.
    pub fn ascending(mut self) -> Self {
        self.sort = Some(ColumnSort::Ascending);
        self
    }

    /// Sütunun azalan sırayla sıralanıp sıralanmayacağını ayarlar.
    pub fn descending(mut self) -> Self {
        self.sort = Some(ColumnSort::Descending);
        self
    }

    /// Sütun metin hizalamasını ortalar.
    pub fn text_center(mut self) -> Self {
        self.align = TextAlign::Center;
        self
    }

    /// Sütun metin hizalamasını ayarlar. Varsayılan soldur.
    ///
    /// Yalnızca `text_left` ve `text_right` desteklenir.
    pub fn text_right(mut self) -> Self {
        self.align = TextAlign::Right;
        self
    }

    /// Sütun dolgusunu ayarlar. Varsayılan None değeridir.
    pub fn paddings(mut self, paddings: impl Into<Edges<Pixels>>) -> Self {
        self.paddings = Some(paddings.into());
        self
    }

    /// dolgu sütun için 0px ayarlar.
    pub fn p_0(mut self) -> Self {
        self.paddings = Some(Edges::all(px(0.)));
        self
    }

    /// Sütun genişliğini ayarlar. Varsayılan 100pxdir.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into();
        self
    }

    /// Sütunun sabit olup olmadığını ayarlar. Varsayılan false değeridir.
    pub fn fixed(mut self, fixed: impl Into<ColumnFixed>) -> Self {
        self.fixed = Some(fixed.into());
        self
    }

    /// Sütunun sol tarafta sabit olup olmadığını ayarlar. Varsayılan false değeridir.
    pub fn fixed_left(mut self) -> Self {
        self.fixed = Some(ColumnFixed::Left);
        self
    }

    /// Sütunun yeniden boyutlandırılabilir olup olmadığını ayarlar. Varsayılan true değeridir.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Sütunun taşınabilir olup olmadığını ayarlar. Varsayılan true.
    pub fn movable(mut self, movable: bool) -> Self {
        self.movable = movable;
        self
    }

    /// Sütunun seçilebilir olup olmadığını ayarlar. Varsayılan true değeridir.
    ///
    /// Olduğunda `false`, bu sütun ve onun hücreler olmayacak participate içinde seçim:
    /// - In sütun seçim mod: sütun başlık olamaz tıklandığında için seçim
    /// - In hücre seçim mod: Cells içinde bu sütun olamaz seçili
    ///
    /// Bu, düğme veya onay kutusu gibi seçim sisteminin parçası olmaması gereken eylem sütunları için kullanışlıdır.
    /// olmamalıdır part seçim sistem.
    ///
    /// # Örnek
    ///
    /// ```rust,ignore
    /// Column::new("actions", "Actions")
    ///     .width(px(100.))
    ///     .selectable(false)  // Prevent selection of action buttons
    /// ```
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Sütunun minimum genişliğini ayarlar. Varsayılan: 20px.
    pub fn min_width(mut self, min_width: impl Into<Pixels>) -> Self {
        let min_width = min_width.into();
        self.min_width = min_width;

        // If the current width is smaller than the new minimum,
        // bump the width up to match the minimum.
        if self.width < min_width {
            self.width = min_width;
        }
        self
    }

    /// Sütunun minimum genişliğini ayarlar. Varsayılan: 1200px.
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        let max_width = max_width.into();
        self.max_width = max_width;

        // If the current width is larger than the new maximum,
        // pull the width down to match the maximum.
        if self.width > max_width {
            self.width = max_width;
        }
        self
    }
}

impl FluentBuilder for Column {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnFixed {
    Left,
}

/// Tablo içinde sıralama sütununun çalışma zamanı bilgisi için kullanılır.
#[derive(Debug, Clone)]
pub(crate) struct ColGroup {
    pub(crate) column: Column,
    /// Bu, sütunun çalışma zamanı genişliğidir; sütun yeniden boyutlandırıldığında güncellenebilir.
    ///
    /// Including genişlik ile sonraki sütunlar ile col_span.
    pub(crate) width: Pixels,
    /// sınırlar sütun içinde tablo sonra onu renders.
    pub(crate) bounds: Bounds<Pixels>,
}

impl ColGroup {
    pub(crate) fn is_resizable(&self) -> bool {
        self.column.resizable
    }
}

#[derive(Clone)]
pub(crate) struct DragColumn {
    pub(crate) entity_id: EntityId,
    pub(crate) name: SharedString,
    pub(crate) width: Pixels,
    pub(crate) col_ix: usize,
}

/// sıralama davranış bir sütun.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum ColumnSort {
    /// Hayır sıralama.
    #[default]
    Default,
    /// Artan sırayla sıralar.
    Ascending,
    /// Azalan sırayla sıralar.
    Descending,
}

impl Render for DragColumn {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px_4()
            .py_1()
            .bg(cx.theme().table_head)
            .text_color(cx.theme().muted_foreground)
            .opacity(0.9)
            .border_1()
            .border_color(cx.theme().border)
            .shadow_md()
            .w(self.width)
            .min_w(px(100.))
            .max_w(px(450.))
            .child(self.name.clone())
    }
}

#[derive(Clone)]
pub(crate) struct ResizeColumn(pub (EntityId, usize));
impl Render for ResizeColumn {
    fn render(&mut self, _window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}
