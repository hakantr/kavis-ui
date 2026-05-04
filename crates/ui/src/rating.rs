use crate::theme::EtkinTema;
use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, Simge, SimgeAdi, StilUzantisi,
    h_flex,
};
use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder as _,
};
use gpui::{ClickEvent, Hsla, StatefulInteractiveElement};

/// Basit yıldız puanlama öğesi.
#[derive(IntoElement)]
pub struct Puanlama {
    id: ElementId,
    style: StyleRefinement,
    size: BilesenBoyutu,
    disabled: bool,
    value: usize,
    max: usize,
    color: Option<Hsla>,
    on_click: Option<Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>>,
}

impl Puanlama {
    /// Yeni bir Puanlama ile bir `ElementId` oluşturur.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: StyleRefinement::default(),
            size: BilesenBoyutu::Orta,
            disabled: false,
            value: 0,
            max: 5,
            color: None,
            on_click: None,
        }
    }

    /// star boyut ayarlar.
    pub fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }

    /// Etkileşimi devre dışı bırakır.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Etkin rengi ayarlar. Varsayılan olarak tema renklerinden `yellow` kullanılır.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// başlangıç değer (0.=maksimum) ayarlar.
    pub fn value(mut self, value: usize) -> Self {
        self.value = value;
        if self.value > self.max {
            self.value = self.max;
        }
        self
    }

    /// en yüksek sayı stars ayarlar.
    pub fn max(mut self, max: usize) -> Self {
        self.max = max;
        if self.value > self.max {
            self.value = self.max;
        }
        self
    }

    /// Puanlama değiştiğinde çağrılacak on_click işleyicisi ekler.
    ///
    /// `&usize` parametresi yeni puanlama değeridir.
    pub fn on_click(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl Styled for Puanlama {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        &mut self.style
    }
}

impl Boyutlandirilabilir for Puanlama {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl DevreDisiBirakilabilir for Puanlama {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

struct RaingState {
    /// To save varsayılan değer üzerinde init durum, için algılar external değer değişimler.
    default_value: usize,
    /// To store geçerli seçili değer.
    value: usize,
    /// To store şu anda hovered değer.
    hovered_value: usize,
}

impl RenderOnce for Puanlama {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = self.id;
        let size = self.size;
        let disabled = self.disabled;
        let max = self.max;
        let default_value = self.value;
        let active_color = self.color.unwrap_or(cx.theme().yellow);
        let on_click = self.on_click.clone();

        let state = window.use_keyed_state(id.clone(), cx, |_, _| RaingState {
            default_value,
            value: default_value,
            hovered_value: 0,
        });

        // Reset state if outside has changed `value` prop.
        if state.read(cx).default_value != default_value {
            state.update(cx, |state, _| {
                state.default_value = default_value;
                state.value = default_value;
            });
        }
        let value = state.read(cx).value;

        h_flex()
            .id(id)
            .flex_nowrap()
            .refine_style(&self.style)
            .on_hover(window.listener_for(&state, move |state, hovered, _, cx| {
                if !hovered {
                    state.hovered_value = 0;
                    cx.notify();
                }
            }))
            .map(|mut this| {
                for ix in 1..=max {
                    let filled = ix <= value;
                    let hovered = state.read(cx).hovered_value >= ix;

                    this = this.child(
                        div()
                            .id(ix)
                            .p_0p5()
                            .flex_none()
                            .flex_shrink_0()
                            .when(filled || hovered, |this| this.text_color(active_color))
                            .child(
                                Simge::new(if filled {
                                    SimgeAdi::StarFill
                                } else {
                                    SimgeAdi::Star
                                })
                                .with_size(size),
                            )
                            .when(!disabled, |this| {
                                this.on_mouse_move(window.listener_for(
                                    &state,
                                    move |state, _, _, cx| {
                                        state.hovered_value = ix;
                                        cx.notify();
                                    },
                                ))
                                .on_click({
                                    let state = state.clone();
                                    let on_click = on_click.clone();
                                    move |_: &ClickEvent, window, cx| {
                                        let new = if value >= ix {
                                            ix.saturating_sub(1)
                                        } else {
                                            ix
                                        };

                                        state.update(cx, |state, cx| {
                                            state.value = new;
                                            cx.notify();
                                        });

                                        if let Some(on_click) = &on_click {
                                            on_click(&new, window, cx);
                                        }
                                    }
                                })
                            }),
                    );
                }

                this
            })
    }
}
