use gpui::Corners;
use gpui::{
    Anchor, App, Context, Edges, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    Pixels, RenderOnce, SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder,
};

use crate::{
    Disableable, ElementExt as _, Selectable, SimgeAdi, Sizable, Size, StyledExt as _,
    menu::{DropdownMenu, PopupMenu},
    tooltip::ComponentTooltip,
};

#[derive(Default)]
struct AcilirDugmeOlcuDurumu {
    width: Pixels,
}

use super::{Dugme, DugmeVaryanti, DugmeVaryantlari, DugmeYuvarlakligi};

#[derive(IntoElement)]
pub struct AcilirDugme {
    id: ElementId,
    style: StyleRefinement,
    button: Option<Dugme>,
    menu:
        Option<Box<dyn Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu + 'static>>,
    selected: bool,
    disabled: bool,
    // The button props
    compact: bool,
    outline: bool,
    loading: bool,
    variant: DugmeVaryanti,
    size: Size,
    rounded: DugmeYuvarlakligi,
    anchor: Anchor,
    tooltip: ComponentTooltip,
}

impl AcilirDugme {
    /// Yeni bir AcilirDugme oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            button: None,
            menu: None,
            selected: false,
            disabled: false,
            compact: false,
            outline: false,
            loading: false,
            variant: DugmeVaryanti::default(),
            size: Size::default(),
            rounded: DugmeYuvarlakligi::default(),
            anchor: Anchor::TopRight,
            tooltip: ComponentTooltip::default(),
        }
    }

    /// araç ipucu metin için açılır düğme ayarlar.
    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip.text = Some((tooltip.into(), None));
        self
    }

    /// sol düğme açılır düğme ayarlar.
    pub fn button(mut self, button: Dugme) -> Self {
        self.button = Some(button);
        self
    }

    /// açılır menü düğme ayarlar.
    pub fn dropdown_menu(
        mut self,
        menu: impl Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu + 'static,
    ) -> Self {
        self.menu = Some(Box::new(menu));
        self
    }

    /// açılır menü düğme ile sabitleyici köşe ayarlar.
    pub fn dropdown_menu_with_anchor(
        mut self,
        anchor: impl Into<Anchor>,
        menu: impl Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu + 'static,
    ) -> Self {
        self.menu = Some(Box::new(menu));
        self.anchor = anchor.into();
        self
    }

    /// rounded stil düğme ayarlar.
    pub fn rounded(mut self, rounded: impl Into<DugmeYuvarlakligi>) -> Self {
        self.rounded = rounded.into();
        self
    }

    /// düğme için kompakt stil ayarlar.
    ///
    /// Bakınız ayrıca: [`Dugme::compact`]
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// düğme için çerçeve stil ayarlar.
    ///
    /// Bakınız ayrıca: [`Dugme::çerçeve`]
    pub fn outline(mut self) -> Self {
        self.outline = true;
        self
    }

    /// düğme için yükleme durum ayarlar.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}

impl Disableable for AcilirDugme {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for AcilirDugme {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        &mut self.style
    }
}

impl Sizable for AcilirDugme {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}

impl DugmeVaryantlari for AcilirDugme {
    fn with_variant(mut self, variant: DugmeVaryanti) -> Self {
        self.variant = variant;
        self
    }
}

impl Selectable for AcilirDugme {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl RenderOnce for AcilirDugme {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let rounded = self.variant.is_ghost() && !self.selected;

        let bounds_id: ElementId =
            SharedString::from(format!("acilir-dugme-bounds:{:?}", &self.id)).into();
        let bounds_state =
            window.use_keyed_state(bounds_id, cx, |_, _| AcilirDugmeOlcuDurumu::default());

        let wrapped_menu = self.menu.map(|builder| {
            let bounds_state = bounds_state.clone();
            let f: Box<dyn Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu> =
                Box::new(move |menu, window, cx| {
                    let width = bounds_state.read(cx).width;
                    let menu = if width > Pixels::ZERO {
                        menu.min_w(width)
                    } else {
                        menu
                    };
                    builder(menu, window, cx)
                });
            f
        });

        div()
            .id(self.id)
            .h_flex()
            .on_prepaint({
                let bounds_state = bounds_state.clone();
                move |bounds, _, cx| {
                    bounds_state.update(cx, |s, _| s.width = bounds.size.width);
                }
            })
            .refine_style(&self.style)
            .when_some(self.button, |this, button| {
                this.child(
                    button
                        .rounded(self.rounded)
                        .border_corners(Corners {
                            top_left: true,
                            top_right: rounded,
                            bottom_left: true,
                            bottom_right: rounded,
                        })
                        .border_edges(Edges {
                            left: true,
                            top: true,
                            right: true,
                            bottom: true,
                        })
                        .loading(self.loading)
                        .selected(self.selected)
                        .disabled(self.disabled || self.loading)
                        .when(self.compact, |this| this.compact())
                        .when(self.outline, |this| this.outline())
                        .with_size(self.size)
                        .with_variant(self.variant),
                )
                .when_some(wrapped_menu, |this, menu| {
                    this.child(
                        Dugme::new("popup")
                            .icon(SimgeAdi::ChevronDown)
                            .rounded(self.rounded)
                            .border_edges(Edges {
                                left: rounded,
                                top: true,
                                right: true,
                                bottom: true,
                            })
                            .border_corners(Corners {
                                top_left: rounded,
                                top_right: true,
                                bottom_left: rounded,
                                bottom_right: true,
                            })
                            .selected(self.selected)
                            .disabled(self.disabled || self.loading)
                            .when(self.compact, |this| this.compact())
                            .when(self.outline, |this| this.outline())
                            .with_size(self.size)
                            .with_variant(self.variant)
                            .dropdown_menu_with_anchor(self.anchor, menu),
                    )
                })
            })
            .map(|this| self.tooltip.apply(this))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[gpui::test]
    fn test_dropdown_button_builder(_cx: &mut gpui::TestAppContext) {
        let button = Dugme::new("inner").label("Action");
        let dropdown = AcilirDugme::new("complex-dropdown")
            .button(button)
            .primary()
            .outline()
            .large()
            .compact()
            .loading(false)
            .disabled(false)
            .selected(false)
            .rounded(DugmeYuvarlakligi::Medium)
            .dropdown_menu_with_anchor(Anchor::BottomLeft, |menu, _, _| menu);

        assert!(dropdown.button.is_some());
        assert_eq!(dropdown.variant, DugmeVaryanti::Primary);
        assert!(dropdown.outline);
        assert_eq!(dropdown.size, Size::Large);
        assert!(dropdown.compact);
        assert!(!dropdown.loading);
        assert!(!dropdown.disabled);
        assert!(!dropdown.selected);
        assert!(matches!(dropdown.rounded, DugmeYuvarlakligi::Medium));
        assert!(dropdown.menu.is_some());
        assert_eq!(dropdown.anchor, Anchor::BottomLeft);
    }
}
