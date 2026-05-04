use gpui::Corners;
use gpui::{
    Anchor, App, Context, Div, Edges, ElementId, Entity, InteractiveElement, Interactivity,
    IntoElement, ParentElement, Pixels, RenderOnce, SharedString, Stateful, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder,
};

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, ElementExt as _, Secilebilir,
    SimgeAdi, StilUzantisi as _,
    menu::{DropdownMenu, PopupMenu},
    tooltip::ComponentTooltip,
};

#[derive(Default)]
struct AcilirDugmeOlcuDurumu {
    width: Pixels,
}

use super::{Dugme, DugmeVaryanti, DugmeVaryantlari, DugmeYuvarlakligi};

/// İçinde hem ana eylem düğmesi hem de açılır chevron düğmesi bulunan, açılır
/// menünün tetikleyicisi olarak kullanılan dahili sarmalayıcı. Sarmalayıcının
/// tamamı tetikleyici olduğu için her iki düğmeye yapılan tıklama da menüyü
/// açar; aynı zamanda popover açıldığında her iki düğme de "selected" görünür.
#[derive(IntoElement)]
struct AcilirDugmeTetikleyici {
    base: Stateful<Div>,
    style: StyleRefinement,
    group_name: SharedString,
    selected: bool,
    action: Option<Dugme>,
    chevron: Option<Dugme>,
    bounds_state: Option<Entity<AcilirDugmeOlcuDurumu>>,
}

impl AcilirDugmeTetikleyici {
    fn new(id: impl Into<ElementId>, group_name: impl Into<SharedString>) -> Self {
        let id = id.into();
        Self {
            base: div().id(id).flex_shrink_0(),
            style: StyleRefinement::default(),
            group_name: group_name.into(),
            selected: false,
            action: None,
            chevron: None,
            bounds_state: None,
        }
    }

    fn action(mut self, button: Option<Dugme>) -> Self {
        self.action = button;
        self
    }

    fn chevron(mut self, button: Option<Dugme>) -> Self {
        self.chevron = button;
        self
    }

    fn track_bounds(mut self, state: Entity<AcilirDugmeOlcuDurumu>) -> Self {
        self.bounds_state = Some(state);
        self
    }

    fn refined_style(mut self, style: StyleRefinement) -> Self {
        self.style = style;
        self
    }
}

impl Styled for AcilirDugmeTetikleyici {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl InteractiveElement for AcilirDugmeTetikleyici {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl Secilebilir for AcilirDugmeTetikleyici {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl DropdownMenu for AcilirDugmeTetikleyici {}

impl RenderOnce for AcilirDugmeTetikleyici {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let selected = self.selected;
        self.base
            .group(self.group_name)
            .flex()
            .items_center()
            .when_some(self.bounds_state, |this, state| {
                this.on_prepaint(move |bounds, _, cx| {
                    state.update(cx, |s, _| s.width = bounds.size.width);
                })
            })
            .refine_style(&self.style)
            .when_some(self.action, |this, btn| this.child(btn.selected(selected)))
            .when_some(self.chevron, |this, btn| this.child(btn.selected(selected)))
    }
}

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
    size: BilesenBoyutu,
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
            size: BilesenBoyutu::default(),
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

impl DevreDisiBirakilabilir for AcilirDugme {
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

impl Boyutlandirilabilir for AcilirDugme {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
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

impl Secilebilir for AcilirDugme {
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
        let group_name: SharedString = format!("acilir-dugme-group:{:?}", &self.id).into();
        let trigger_id: ElementId =
            SharedString::from(format!("acilir-dugme-trigger:{:?}", &self.id)).into();

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

        let action_button = self.button.map(|button| {
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
                .disabled(self.disabled || self.loading)
                .when(self.compact, |this| this.compact())
                .when(self.outline, |this| this.outline())
                .with_size(self.size)
                .with_variant(self.variant)
                .group_hover_with(group_name.clone())
        });

        let chevron_button = wrapped_menu.is_some().then(|| {
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
                .disabled(self.disabled || self.loading)
                .when(self.compact, |this| this.compact())
                .when(self.outline, |this| this.outline())
                .with_size(self.size)
                .with_variant(self.variant)
                .group_hover_with(group_name.clone())
        });

        let trigger = AcilirDugmeTetikleyici::new(trigger_id, group_name)
            .selected(self.selected)
            .action(action_button)
            .chevron(chevron_button)
            .track_bounds(bounds_state.clone())
            .refined_style(self.style.clone());

        div()
            .id(self.id)
            .map(|this| match wrapped_menu {
                Some(menu) => this.child(trigger.dropdown_menu_with_anchor(self.anchor, menu)),
                None => this.child(trigger),
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
        assert_eq!(dropdown.size, BilesenBoyutu::Buyuk);
        assert!(dropdown.compact);
        assert!(!dropdown.loading);
        assert!(!dropdown.disabled);
        assert!(!dropdown.selected);
        assert!(matches!(dropdown.rounded, DugmeYuvarlakligi::Medium));
        assert!(dropdown.menu.is_some());
        assert_eq!(dropdown.anchor, Anchor::BottomLeft);
    }
}
