use gpui::Corners;
use gpui::{
    Anchor, App, Context, Edges, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder,
};

use crate::{
    Disableable, EtkinTema as _, Renklendir as _, Selectable, SimgeAdi, Sizable, Size,
    StyledExt as _, h_flex,
    menu::{DropdownMenuPopover, PopupAppearance, PopupMenu},
    popover::Yon,
    tooltip::ComponentTooltip,
};

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
    match_trigger_width: bool,
    auto_flip: bool,
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
            // Sol kenar hizali asagi acilim — popup, dugmenin sol kenarindan baslar.
            anchor: Anchor::TopLeft,
            // Acilan menu varsayilan olarak dugme genisligine kilitlenir.
            match_trigger_width: true,
            // Pencere/ekran sinirina sigmazsa anchor'i otomatik flip et.
            auto_flip: true,
            tooltip: ComponentTooltip::default(),
        }
    }

    /// Acilan menunun, dugme genisligine kilitlenip kilitlenmeyecegini belirler.
    /// Varsayilan: `true`.
    pub fn match_trigger_width(mut self, value: bool) -> Self {
        self.match_trigger_width = value;
        self
    }

    /// Acilis yonunu Turkce ad ile ayarlar (`Yon::Asagi` veya `Yon::Yukari`).
    ///
    /// Yatay hiza icin [`AcilirDugme::dropdown_menu_with_anchor`] dogrudan
    /// [`Anchor`] alir.
    pub fn yon(mut self, yon: impl Into<Yon>) -> Self {
        self.anchor = yon.into().into();
        self
    }

    /// True ise pencere/ekran sinirina sigmazsa popup zit kenara otomatik flip eder.
    /// Ornegin asagi acilim icin yer yoksa yukari acilir. Varsayilan: `true`.
    pub fn otomatik_yon(mut self, value: bool) -> Self {
        self.auto_flip = value;
        self
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

/// Hem ana dugme hem chevron'u tek bir tetik olarak sunar; dropdown popover'i
/// bu birlesik tetigin uzerine baglandiginda her iki dugmeye yapilan tiklamalar
/// ayni menuyu acar/kapatir.
#[derive(IntoElement)]
struct AcilirDugmeTetik {
    button: Dugme,
    selected: bool,
    rounded: DugmeYuvarlakligi,
    /// Ghost varyantta ic kose yuvarlatilsin mi.
    inner_rounded: bool,
    compact: bool,
    outline: bool,
    loading: bool,
    disabled: bool,
    size: Size,
    variant: DugmeVaryanti,
}

impl Selectable for AcilirDugmeTetik {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}

impl RenderOnce for AcilirDugmeTetik {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        h_flex()
            .child(
                self.button
                    .rounded(self.rounded)
                    .border_corners(Corners {
                        top_left: true,
                        top_right: self.inner_rounded,
                        bottom_left: true,
                        bottom_right: self.inner_rounded,
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
            .child(
                Dugme::new("popup")
                    .icon(SimgeAdi::ChevronDown)
                    .rounded(self.rounded)
                    .border_edges(Edges {
                        left: self.inner_rounded,
                        top: true,
                        right: true,
                        bottom: true,
                    })
                    .border_corners(Corners {
                        top_left: self.inner_rounded,
                        top_right: true,
                        bottom_left: self.inner_rounded,
                        bottom_right: true,
                    })
                    .selected(self.selected)
                    .disabled(self.disabled || self.loading)
                    .when(self.compact, |this| this.compact())
                    .when(self.outline, |this| this.outline())
                    .with_size(self.size)
                    .with_variant(self.variant),
            )
    }
}

/// Tetikleyici varyantindan popup goruntu setine cevirim.
///
/// - Default/Ghost/Link/Text: tema default (None) — nötr popover.
/// - Primary/Secondary: varyantin solid bg + tema `*_foreground` ile WCAG
///   kontrasti dogrulanir; yetersizse `uyumlu_metin_rengi` yedegi devreye girer.
/// - Status (Danger/Warning/Success/Info): translucent brand bg + brand fg
///   (Faz 1 kontrast kuralina uyumlu).
/// - Custom: tema default (None).
fn popup_appearance_for_variant(variant: DugmeVaryanti, cx: &App) -> Option<PopupAppearance> {
    use gpui::Hsla;
    let mix_translucent = |c: Hsla| c.mix_oklab(cx.theme().transparent, 0.85);

    match variant {
        DugmeVaryanti::Default
        | DugmeVaryanti::Ghost
        | DugmeVaryanti::Link
        | DugmeVaryanti::Text => None,
        // Primary/Secondary icin oncelikle tema `*_foreground` denenir; WCAG AA
        // kontrasti (>= 4.5:1) saglanmazsa `uyumlu_metin_rengi` yedegine dusulur.
        // Bu sayede tema tasarimcisinin secimi baskin kalir, yalnizca yanlis
        // override durumunda kontrast garantisi devreye girer.
        DugmeVaryanti::Primary => {
            let bg = cx.theme().button_primary;
            Some(PopupAppearance {
                bg,
                fg: crate::akilli_yazi_rengi(bg, cx.theme().button_primary_foreground),
                border: bg,
            })
        }
        DugmeVaryanti::Secondary => {
            let bg = cx.theme().secondary;
            Some(PopupAppearance {
                bg,
                fg: crate::akilli_yazi_rengi(bg, cx.theme().secondary_foreground),
                border: cx.theme().border,
            })
        }
        DugmeVaryanti::Danger => Some(PopupAppearance {
            bg: mix_translucent(cx.theme().danger),
            fg: cx.theme().danger,
            border: cx.theme().danger.mix_oklab(cx.theme().transparent, 0.6),
        }),
        DugmeVaryanti::Warning => Some(PopupAppearance {
            bg: mix_translucent(cx.theme().warning),
            fg: cx.theme().warning,
            border: cx.theme().warning.mix_oklab(cx.theme().transparent, 0.6),
        }),
        DugmeVaryanti::Success => Some(PopupAppearance {
            bg: mix_translucent(cx.theme().success),
            fg: cx.theme().success,
            border: cx.theme().success.mix_oklab(cx.theme().transparent, 0.6),
        }),
        DugmeVaryanti::Info => Some(PopupAppearance {
            bg: mix_translucent(cx.theme().info),
            fg: cx.theme().info,
            border: cx.theme().info.mix_oklab(cx.theme().transparent, 0.6),
        }),
        // Custom varyantta button renkleri private; popup'i tema default'unda
        // birakmak guvenli — kullanici kendi temasini korur.
        DugmeVaryanti::Custom(_) => None,
    }
}

impl RenderOnce for AcilirDugme {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let inner_rounded = self.variant.is_ghost() && !self.selected;
        let style = self.style;
        let id = self.id;
        let tooltip = self.tooltip;
        let appearance = popup_appearance_for_variant(self.variant, cx);

        div()
            .id(id.clone())
            .h_flex()
            .refine_style(&style)
            .when_some(self.button, |this, button| match self.menu {
                Some(menu) => {
                    let trigger = AcilirDugmeTetik {
                        button,
                        selected: self.selected,
                        rounded: self.rounded,
                        inner_rounded,
                        compact: self.compact,
                        outline: self.outline,
                        loading: self.loading,
                        disabled: self.disabled,
                        size: self.size,
                        variant: self.variant,
                    };
                    this.child(
                        DropdownMenuPopover::new(id, self.anchor, trigger, menu)
                            .match_trigger_width(self.match_trigger_width)
                            .auto_flip(self.auto_flip)
                            .appearance(appearance),
                    )
                }
                None => this.child(
                    button
                        .rounded(self.rounded)
                        .border_corners(Corners {
                            top_left: true,
                            top_right: inner_rounded,
                            bottom_left: true,
                            bottom_right: inner_rounded,
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
                ),
            })
            .map(|this| tooltip.apply(this))
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
