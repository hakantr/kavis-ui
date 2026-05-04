use std::{cell::Cell, rc::Rc};

use gpui::{
    AnyElement, App, Corners, Edges, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _,
};
use smallvec::{SmallVec, smallvec};

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, EtkinTema, Simge, StilUzantisi,
    h_flex, tooltip::ComponentTooltip,
};

#[derive(Default, Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GecisVaryanti {
    #[default]
    Ghost,
    Outline,
}

pub trait GecisVaryantlari: Sized {
    /// varyant açıp kapatma ayarlar.
    fn with_variant(self, variant: GecisVaryanti) -> Self;
    /// varyant için ghost ayarlar.
    fn ghost(self) -> Self {
        self.with_variant(GecisVaryanti::Ghost)
    }
    /// varyant için çerçeve ayarlar.
    fn outline(self) -> Self {
        self.with_variant(GecisVaryanti::Outline)
    }
}

#[derive(IntoElement)]
pub struct Gecis {
    id: ElementId,
    style: StyleRefinement,
    checked: bool,
    size: BilesenBoyutu,
    variant: GecisVaryanti,
    disabled: bool,
    border_corners: Corners<bool>,
    border_edges: Edges<bool>,
    children: SmallVec<[AnyElement; 1]>,
    on_click: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    tooltip: ComponentTooltip,
}

impl Gecis {
    /// Yeni bir Gecis öğe oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            checked: false,
            size: BilesenBoyutu::default(),
            variant: GecisVaryanti::default(),
            disabled: false,
            border_corners: Corners {
                top_left: true,
                top_right: true,
                bottom_left: true,
                bottom_right: true,
            },
            border_edges: Edges::all(true),
            children: smallvec![],
            on_click: None,
            tooltip: ComponentTooltip::default(),
        }
    }

    /// araç ipucu metin için açıp kapatma ayarlar.
    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip.text = Some((tooltip.into(), None));
        self
    }

    /// Bir etiket için açıp kapatma ekler.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        let label: SharedString = label.into();
        self.children.push(label.into_any_element());
        self
    }

    /// simge için açıp kapatma. ekler.
    pub fn icon(mut self, icon: impl Into<Simge>) -> Self {
        let icon: Simge = icon.into();
        self.children.push(icon.into());
        self
    }

    /// Açıp kapatma öğesinin checked durumunu ayarlar. Varsayılan false değeridir.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Açıp kapatma öğesi tıklandığında çağrılacak geri çağrıyı ayarlar.
    ///
    /// `&bool` parametresi açıp kapatma öğesinin yeni işaretli durumunu temsil eder.
    pub fn on_click(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub(crate) fn border_corners(mut self, corners: impl Into<Corners<bool>>) -> Self {
        self.border_corners = corners.into();
        self
    }

    pub(crate) fn border_edges(mut self, edges: impl Into<Edges<bool>>) -> Self {
        self.border_edges = edges.into();
        self
    }
}

impl GecisVaryantlari for Gecis {
    fn with_variant(mut self, variant: GecisVaryanti) -> Self {
        self.variant = variant;
        self
    }
}

impl ParentElement for Gecis {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl DevreDisiBirakilabilir for Gecis {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Boyutlandirilabilir for Gecis {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for Gecis {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Gecis {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let checked = self.checked;
        let disabled = self.disabled;
        let hoverable = !disabled && !checked;
        let rounding = cx.theme().radius;

        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .map(|this| match self.size {
                BilesenBoyutu::CokKucuk => this.min_w_5().h_5().px_0p5().text_xs(),
                BilesenBoyutu::Kucuk => this.min_w_6().h_6().px_1().text_sm(),
                BilesenBoyutu::Buyuk => this.min_w_9().h_9().px_3().text_lg(),
                _ => this.min_w_8().h_8().px_2(),
            })
            .when(self.border_corners.top_left, |this| {
                this.rounded_tl(rounding)
            })
            .when(self.border_corners.top_right, |this| {
                this.rounded_tr(rounding)
            })
            .when(self.border_corners.bottom_left, |this| {
                this.rounded_bl(rounding)
            })
            .when(self.border_corners.bottom_right, |this| {
                this.rounded_br(rounding)
            })
            .when(self.variant == GecisVaryanti::Outline, |this| {
                this.when(self.border_edges.left, |this| this.border_l_1())
                    .when(self.border_edges.right, |this| this.border_r_1())
                    .when(self.border_edges.top, |this| this.border_t_1())
                    .when(self.border_edges.bottom, |this| this.border_b_1())
                    .border_color(cx.theme().border)
                    .bg(cx.theme().background)
                    .when(cx.theme().shadow, |this| this.shadow_xs())
            })
            .when(hoverable, |this| {
                this.hover(|this| {
                    this.bg(cx.theme().accent)
                        .text_color(cx.theme().accent_foreground)
                })
            })
            .when(checked, |this| {
                this.bg(cx.theme().accent)
                    .text_color(cx.theme().accent_foreground)
            })
            .refine_style(&self.style)
            .children(self.children)
            .when(!disabled, |this| {
                this.when_some(self.on_click, |this, on_click| {
                    this.on_click(move |_, window, cx| on_click(&!checked, window, cx))
                })
            })
            .map(|this| self.tooltip.apply(this))
    }
}

/// Bir grup toggles.
#[derive(IntoElement)]
pub struct GecisGrubu {
    id: ElementId,
    style: StyleRefinement,
    size: BilesenBoyutu,
    variant: GecisVaryanti,
    disabled: bool,
    segmented: bool,
    items: Vec<Gecis>,
    on_click: Option<Rc<dyn Fn(&Vec<bool>, &mut Window, &mut App) + 'static>>,
}

impl GecisGrubu {
    /// Yeni bir GecisGrubu öğe oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            size: BilesenBoyutu::default(),
            variant: GecisVaryanti::default(),
            disabled: false,
            segmented: false,
            items: Vec::new(),
            on_click: None,
        }
    }

    /// Bir alt [`Gecis`] için grup ekler.
    pub fn child(mut self, toggle: impl Into<Gecis>) -> Self {
        self.items.push(toggle.into());
        self
    }

    /// Birden çok [`Gecis`]s için grup ekler.
    pub fn children(mut self, children: impl IntoIterator<Item = impl Into<Gecis>>) -> Self {
        self.items.extend(children.into_iter().map(Into::into));
        self
    }

    /// geri çağrı olmak için çağrılır olduğunda açıp kapatma grup değişimler ayarlar.
    ///
    /// `&Vec<bool>` parametresi gruptaki her [`Gecis`] için yeni işaretli durumu temsil eder.
    pub fn on_click(
        mut self,
        on_click: impl Fn(&Vec<bool>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(on_click));
        self
    }

    /// grup olarak bir connected segmented kontrol. çizer.
    ///
    /// Bu korur existing multi-açıp kapatma davranış, ama removes varsayılan
    /// gap ve joins adjacent öğe kenarlıklar içine tek segmented çerçeve.
    pub fn segmented(mut self) -> Self {
        self.segmented = true;
        self
    }
}

impl Boyutlandirilabilir for GecisGrubu {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl GecisVaryantlari for GecisGrubu {
    fn with_variant(mut self, variant: GecisVaryanti) -> Self {
        self.variant = variant;
        self
    }
}

impl DevreDisiBirakilabilir for GecisGrubu {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for GecisGrubu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for GecisGrubu {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let disabled = self.disabled;
        let items_len = self.items.len();
        let checks = self
            .items
            .iter()
            .map(|item| item.checked)
            .collect::<Vec<bool>>();
        let state = Rc::new(Cell::new(None));

        h_flex()
            .id(self.id)
            .when(!self.segmented, |this| this.gap_2())
            .refine_style(&self.style)
            .children(self.items.into_iter().enumerate().map({
                |(ix, item)| {
                    let state = state.clone();
                    let item = if !self.segmented || items_len == 1 {
                        item
                    } else if ix == 0 {
                        item.border_corners(Corners {
                            top_left: true,
                            top_right: false,
                            bottom_left: true,
                            bottom_right: false,
                        })
                        .border_edges(Edges {
                            left: true,
                            top: true,
                            right: true,
                            bottom: true,
                        })
                    } else if ix == items_len - 1 {
                        item.border_corners(Corners {
                            top_left: false,
                            top_right: true,
                            bottom_left: false,
                            bottom_right: true,
                        })
                        .border_edges(Edges {
                            left: false,
                            top: true,
                            right: true,
                            bottom: true,
                        })
                    } else {
                        item.border_corners(Corners {
                            top_left: false,
                            top_right: false,
                            bottom_left: false,
                            bottom_right: false,
                        })
                        .border_edges(Edges {
                            left: false,
                            top: true,
                            right: true,
                            bottom: true,
                        })
                    };

                    item.disabled(disabled)
                        .with_size(self.size)
                        .with_variant(self.variant)
                        .on_click(move |_, _, _| {
                            state.set(Some(ix));
                        })
                }
            }))
            .when(!disabled, |this| {
                this.when_some(self.on_click, |this, on_click| {
                    this.on_click(move |_, window, cx| {
                        if let Some(ix) = state.get() {
                            let mut checks = checks.clone();
                            checks[ix] = !checks[ix];
                            on_click(&checks, window, cx);
                        }
                    })
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimgeAdi;

    #[gpui::test]
    fn test_toggle_builder(_cx: &mut gpui::TestAppContext) {
        let toggle = Gecis::new("complex-toggle")
            .label("Enable Feature")
            .icon(SimgeAdi::Check)
            .checked(true)
            .outline()
            .large()
            .disabled(false)
            .on_click(|_, _, _| {});

        assert_eq!(toggle.children.len(), 2); // label + icon
        assert!(toggle.checked);
        assert_eq!(toggle.variant, GecisVaryanti::Outline);
        assert_eq!(toggle.size, BilesenBoyutu::Buyuk);
        assert!(!toggle.disabled);
        assert!(toggle.on_click.is_some());
    }

    #[gpui::test]
    fn test_toggle_group_builder(_cx: &mut gpui::TestAppContext) {
        let group = GecisGrubu::new("complex-group")
            .child(Gecis::new("toggle1").label("Option 1"))
            .child(Gecis::new("toggle2").label("Option 2").checked(true))
            .child(Gecis::new("toggle3").label("Option 3"))
            .outline()
            .large()
            .segmented()
            .disabled(false)
            .on_click(|_, _, _| {});

        assert_eq!(group.items.len(), 3);
        assert_eq!(group.variant, GecisVaryanti::Outline);
        assert_eq!(group.size, BilesenBoyutu::Buyuk);
        assert!(group.segmented);
        assert!(!group.disabled);
        assert!(group.on_click.is_some());
    }
}
