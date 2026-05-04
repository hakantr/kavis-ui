use gpui::Corners;
use gpui::InteractiveElement;
use gpui::ParentElement;
use gpui::{App, Axis, Edges, ElementId, IntoElement, Window};
use gpui::{
    RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, div,
    prelude::FluentBuilder as _,
};
use std::{cell::Cell, rc::Rc};

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, StilUzantisi,
    button::{Dugme, DugmeVaryanti, DugmeVaryantlari},
};

/// Bir DugmeGrubu öğe, için sarma çoklu düğmeler içinde bir grup.
#[derive(IntoElement)]
pub struct DugmeGrubu {
    id: ElementId,
    style: StyleRefinement,
    children: Vec<Dugme>,
    pub(super) multiple: bool,
    pub(super) disabled: bool,
    pub(super) layout: Axis,

    // The button props
    pub(super) compact: bool,
    pub(super) outline: bool,
    pub(super) variant: Option<DugmeVaryanti>,
    pub(super) size: Option<BilesenBoyutu>,

    on_click: Option<Box<dyn Fn(&Vec<usize>, &mut Window, &mut App) + 'static>>,
}

impl DevreDisiBirakilabilir for DugmeGrubu {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl DugmeGrubu {
    /// Yeni bir DugmeGrubu oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            children: Vec::new(),
            variant: None,
            size: None,
            compact: false,
            outline: false,
            multiple: false,
            disabled: false,
            layout: Axis::Horizontal,
            on_click: None,
        }
    }

    /// bir düğme olarak bir alt için DugmeGrubu. ekler.
    pub fn child(mut self, child: Dugme) -> Self {
        self.children.push(child.disabled(self.disabled));
        self
    }

    /// çoklu düğmeler olarak alt öğeler için DugmeGrubu. ekler.
    pub fn children(mut self, children: impl IntoIterator<Item = Dugme>) -> Self {
        self.children.extend(children);
        self
    }

    /// Çoklu seçim modunu ayarlar. Varsayılan false değeridir (tek seçim).
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Düğme grubunun yerleşimini ayarlar. Varsayılan `Axis::Horizontal` değeridir.
    pub fn layout(mut self, layout: Axis) -> Self {
        self.layout = layout;
        self
    }

    /// İle kompakt mod için DugmeGrubu.
    ///
    /// Bakınız ayrıca: [`Dugme::compact()`]
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// İle çerçeve mod için DugmeGrubu.
    ///
    /// Bakınız ayrıca: [`Dugme::çerçeve()`]
    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    /// on_click işleyici için DugmeGrubu ayarlar.
    ///
    /// İşleyicinin ilk argümanı seçili düğme indekslerini içeren bir vektördür.
    ///
    /// `&Vec<usize>`, tıklanan veya `multiple` modda seçili olan düğmelerin indeksleridir.
    /// Örneğin `[0, 2, 3]`, birinci, üçüncü ve dördüncü düğmelerin tıklandığı anlamına gelir.
    ///
    /// ```ignore
    /// DugmeGrubu::new("size-button")
    ///    .child(Dugme::new("large").label("Large").selected(self.size == BilesenBoyutu::Buyuk))
    ///    .child(Dugme::new("medium").label("Medium").selected(self.size == BilesenBoyutu::Orta))
    ///    .child(Dugme::new("small").label("Small").selected(self.size == BilesenBoyutu::Kucuk))
    ///    .on_click(cx.listener(|view, clicks: &Vec<usize>, _, cx| {
    ///        if clicks.contains(&0) {
    ///            view.size = BilesenBoyutu::Buyuk;
    ///        } else if clicks.contains(&1) {
    ///            view.size = BilesenBoyutu::Orta;
    ///        } else if clicks.contains(&2) {
    ///            view.size = BilesenBoyutu::Kucuk;
    ///        }
    ///        cx.notify();
    ///    }))
    /// ```
    pub fn on_click(
        mut self,
        handler: impl Fn(&Vec<usize>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Boyutlandirilabilir for DugmeGrubu {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl Styled for DugmeGrubu {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        &mut self.style
    }
}

impl DugmeVaryantlari for DugmeGrubu {
    fn with_variant(mut self, variant: DugmeVaryanti) -> Self {
        self.variant = Some(variant);
        self
    }
}

impl RenderOnce for DugmeGrubu {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let children_len = self.children.len();
        let mut selected_ixs: Vec<usize> = Vec::new();
        let state = Rc::new(Cell::new(None));

        for (ix, child) in self.children.iter().enumerate() {
            if child.selected {
                selected_ixs.push(ix);
            }
        }

        let vertical = self.layout == Axis::Vertical;

        div()
            .id(self.id)
            .flex()
            .when(vertical, |this| this.flex_col().justify_center())
            .when(!vertical, |this| this.items_center())
            .refine_style(&self.style)
            .children(
                self.children
                    .into_iter()
                    .enumerate()
                    .map(|(child_index, child)| {
                        let state = Rc::clone(&state);
                        let child = if children_len == 1 {
                            child
                        } else if child_index == 0 {
                            // First
                            child
                                .border_corners(Corners {
                                    top_left: true,
                                    top_right: vertical,
                                    bottom_left: !vertical,
                                    bottom_right: false,
                                })
                                .border_edges(Edges {
                                    left: true,
                                    top: true,
                                    right: true,
                                    bottom: true,
                                })
                        } else if child_index == children_len - 1 {
                            // Last
                            child
                                .border_edges(Edges {
                                    left: vertical,
                                    top: !vertical,
                                    right: true,
                                    bottom: true,
                                })
                                .border_corners(Corners {
                                    top_left: false,
                                    top_right: !vertical,
                                    bottom_left: vertical,
                                    bottom_right: true,
                                })
                        } else {
                            // Middle
                            child
                                .border_corners(Corners {
                                    top_left: false,
                                    top_right: false,
                                    bottom_left: false,
                                    bottom_right: false,
                                })
                                .border_edges(Edges {
                                    left: vertical,
                                    top: !vertical,
                                    right: true,
                                    bottom: true,
                                })
                        }
                        .when_some(self.size, |this, size| this.with_size(size))
                        .when_some(self.variant, |this, variant| this.with_variant(variant))
                        .when(self.compact, |this| this.compact())
                        .when(self.outline, |this| this.outline())
                        .when(self.on_click.is_some(), |this| {
                            this.on_click(move |_, _, _| {
                                state.set(Some(child_index));
                            })
                        });

                        child
                    }),
            )
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                move |this, on_click| {
                    this.on_click(move |_, window, cx| {
                        let mut selected_ixs = selected_ixs.clone();
                        if let Some(ix) = state.get() {
                            if self.multiple {
                                if let Some(pos) = selected_ixs.iter().position(|&i| i == ix) {
                                    selected_ixs.remove(pos);
                                } else {
                                    selected_ixs.push(ix);
                                }
                            } else {
                                selected_ixs.clear();
                                selected_ixs.push(ix);
                            }
                        }

                        on_click(&selected_ixs, window, cx);
                    })
                },
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::Axis;

    #[gpui::test]
    fn test_button_group_builder(_cx: &mut gpui::TestAppContext) {
        let group = DugmeGrubu::new("complex-group")
            .child(Dugme::new("btn1").label("One"))
            .child(Dugme::new("btn2").label("Two"))
            .child(Dugme::new("btn3").label("Three"))
            .primary()
            .large()
            .outline()
            .compact()
            .multiple(true)
            .layout(Axis::Vertical)
            .disabled(false)
            .on_click(|_, _, _| {});

        assert_eq!(group.children.len(), 3);
        assert_eq!(group.variant, Some(DugmeVaryanti::Primary));
        assert_eq!(group.size, Some(BilesenBoyutu::Buyuk));
        assert!(group.outline);
        assert!(group.compact);
        assert!(group.multiple);
        assert_eq!(group.layout, Axis::Vertical);
        assert!(!group.disabled);
        assert!(group.on_click.is_some());
    }
}
