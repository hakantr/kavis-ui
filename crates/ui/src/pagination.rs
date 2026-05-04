use std::{ops::Range, rc::Rc};

use crate::ham_gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, prelude::FluentBuilder, px,
};
use rust_i18n::t;

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, Simge, StilUzantisi,
    button::{Dugme, DugmeVaryantlari},
    h_flex,
    icon::SimgeAdi,
    menu::{AcilirMenuOgesi, AcilirMenuTetikleyici as _},
};

/// Sonraki ve önceki bağlantılarıyla sayfa gezinmesi sağlar.
#[derive(IntoElement)]
pub struct Sayfalama {
    id: ElementId,
    style: StyleRefinement,
    size: BilesenBoyutu,
    current_page: usize,
    total_pages: usize,
    disabled: bool,
    compact: bool,
    visible_pages: usize,
    on_click: Option<Rc<dyn Fn(&usize, &mut Window, &mut App)>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PageItem {
    Page(usize),
    Ellipsis(Range<usize>),
}

impl Sayfalama {
    /// Yeni bir Sayfalama bileşen ile verilen ID oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            size: BilesenBoyutu::default(),
            current_page: 1,
            total_pages: 1,
            visible_pages: 5,
            disabled: false,
            compact: false,
            on_click: None,
        }
    }

    /// geçerli sayfa sayı (1-temelli) ayarlar.
    ///
    /// total_pages ayarlıysa değer 1 ile total_pages arasında sınırlandırılır.
    pub fn current_page(mut self, page: usize) -> Self {
        self.current_page = page.max(1);
        self
    }

    /// toplam sayı pages ayarlar.
    pub fn total_pages(mut self, pages: usize) -> Self {
        self.total_pages = pages.max(1);
        if self.current_page > self.total_pages {
            self.current_page = self.total_pages;
        }
        self
    }

    /// işleyici için sayfa değişir (olduğunda tıklama üzerinde sayfa sayılar, önceki, veya sonraki) ayarlar.
    ///
    /// Bu işleyici receives yeni sayfa sayı için navigate için.
    ///
    /// # Örnekler
    ///
    /// ```ignore
    /// Sayfalama::new("my-pagination")
    ///     .current_page(current_page)
    ///     .total_pages(total_pages)
    ///     .on_click(|page, _, cx| {
    ///         // Handle page change
    ///     })
    /// ```
    pub fn on_click(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }

    /// göstermek için olarak kompakt stil ayarlar.
    ///
    /// true, yalnızca önceki ise, sonraki düğmeler ile yalnızca simge.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// viewable en yüksek sayı sayfa düğmeler, varsayılan ayarlar.
    pub fn visible_pages(mut self, max: usize) -> Self {
        self.visible_pages = max;
        self
    }

    fn render_nav_button(&self, is_prev: bool) -> Dugme {
        let (id, label, icon, disabled) = if is_prev {
            (
                "prev",
                t!("Sayfalama.previous"),
                SimgeAdi::ChevronLeft,
                self.current_page <= 1,
            )
        } else {
            (
                "next",
                t!("Sayfalama.next"),
                SimgeAdi::ChevronRight,
                self.current_page >= self.total_pages,
            )
        };

        let target_page = if is_prev {
            self.current_page.saturating_sub(1)
        } else {
            self.current_page.saturating_add(1)
        };

        Dugme::new(id)
            .ghost()
            .compact()
            .with_size(self.size)
            .disabled(self.disabled || disabled)
            .tooltip(label.clone())
            .when(self.compact, |this| this.icon(icon.clone()))
            .when(!self.compact, |this| {
                this.child(
                    h_flex()
                        .w_full()
                        .gap_2()
                        .flex_nowrap()
                        .when(is_prev, |this| this.flex_row_reverse())
                        .child(SharedString::from(label))
                        .child(Simge::new(icon)),
                )
            })
            .when_some(self.on_click.clone(), |this, handler| {
                this.on_click(move |_, window, cx| {
                    handler(&target_page, window, cx);
                })
            })
    }
}

impl DevreDisiBirakilabilir for Sayfalama {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Boyutlandirilabilir for Sayfalama {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for Sayfalama {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Sayfalama {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let page_numbers = if !self.compact {
            calculate_page_range(self.current_page, self.total_pages, self.visible_pages)
        } else {
            vec![]
        };

        let current_page = self.current_page;
        let is_disabled = self.disabled;
        let on_click = self.on_click.clone();

        h_flex()
            .id(self.id.clone())
            .px_2()
            .py_2()
            .gap_1()
            .items_center()
            .refine_style(&self.style)
            .child(self.render_nav_button(true))
            .children({
                page_numbers.into_iter().map(|item| match item {
                    PageItem::Page(page) => {
                        let is_selected = page == current_page;

                        Dugme::new(page)
                            .with_size(self.size)
                            .map(|this| {
                                if is_selected {
                                    this.outline()
                                } else {
                                    this.ghost()
                                }
                            })
                            .label(page.to_string())
                            .compact()
                            .disabled(is_disabled)
                            .when(!is_selected, |this| {
                                this.when_some(on_click.clone(), |this, handler| {
                                    this.on_click(move |_, window, cx| {
                                        handler(&page, window, cx);
                                    })
                                })
                            })
                            .into_any_element()
                    }
                    PageItem::Ellipsis(range) => Dugme::new(SharedString::from(format!(
                        "ellipsis-{}-{}",
                        range.start, range.end
                    )))
                    .ghost()
                    .with_size(self.size)
                    .compact()
                    .disabled(self.disabled)
                    .icon(SimgeAdi::Ellipsis)
                    .acilir_menu({
                        let on_click = on_click.clone();
                        move |mut menu, _, _| {
                            for page in range.clone() {
                                menu = menu.item(
                                    AcilirMenuOgesi::new(format!("{}", page))
                                        .checked(page == current_page)
                                        .on_click({
                                            let on_click = on_click.clone();
                                            move |_, window, cx| {
                                                if let Some(handler) = &on_click {
                                                    handler(&page, window, cx);
                                                }
                                            }
                                        }),
                                )
                            }

                            menu.min_w(px(55.)).max_h(px(240.)).scrollable(true)
                        }
                    })
                    .into_any_element(),
                })
            })
            .child(self.render_nav_button(false))
    }
}

fn calculate_page_range(current: usize, total: usize, max_visible: usize) -> Vec<PageItem> {
    if total <= 1 {
        return vec![];
    }

    let max_visible = max_visible.max(5);

    if total <= max_visible {
        return (1..=total).map(PageItem::Page).collect();
    }

    let mut pages = vec![];
    let side_pages = (max_visible - 3) / 2;

    pages.push(PageItem::Page(1));

    let start = if current <= side_pages + 1 {
        2
    } else if current > total - side_pages - 1 {
        total - side_pages - 1
    } else {
        current - side_pages
    };

    if start > 2 {
        pages.push(PageItem::Ellipsis(2..start));
    }

    let end = if current >= total - side_pages {
        total - 1
    } else if current <= side_pages + 1 {
        side_pages + 2
    } else {
        current + side_pages
    };

    for page in start..=end {
        pages.push(PageItem::Page(page));
    }

    if end < total - 1 {
        pages.push(PageItem::Ellipsis(end + 1..total));
    }

    pages.push(PageItem::Page(total));

    pages
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_page_range() {
        use super::{PageItem, calculate_page_range};

        let result = calculate_page_range(1, 10, 7);
        let expected = vec![
            PageItem::Page(1),
            PageItem::Page(2),
            PageItem::Page(3),
            PageItem::Page(4),
            PageItem::Ellipsis(5..10),
            PageItem::Page(10),
        ];
        assert_eq!(result, expected);

        let result = calculate_page_range(5, 10, 7);
        let expected = vec![
            PageItem::Page(1),
            PageItem::Ellipsis(2..3),
            PageItem::Page(3),
            PageItem::Page(4),
            PageItem::Page(5),
            PageItem::Page(6),
            PageItem::Page(7),
            PageItem::Ellipsis(8..10),
            PageItem::Page(10),
        ];
        assert_eq!(result, expected);

        let result = calculate_page_range(10, 10, 7);
        let expected = vec![
            PageItem::Page(1),
            PageItem::Ellipsis(2..7),
            PageItem::Page(7),
            PageItem::Page(8),
            PageItem::Page(9),
            PageItem::Page(10),
        ];
        assert_eq!(result, expected);
    }
}
