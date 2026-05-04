use std::{rc::Rc, time::Duration};

use gpui::{
    Animation, AnimationExt as _, AnyElement, App, ClickEvent, DefiniteLength, DismissEvent, Edges,
    EventEmitter, FocusHandle, InteractiveElement as _, IntoElement, KeyBinding, MouseButton,
    ParentElement, Pixels, RenderOnce, StyleRefinement, Styled, Window, WindowControlArea,
    anchored, div, point, prelude::FluentBuilder as _, px,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    Boyutlandirilabilir, EtkinTema, FocusTrapElement as _, PencereUzantisi as _, Placement,
    SimgeAdi, StilUzantisi as _,
    actions::Cancel,
    button::{Dugme, DugmeVaryantlari as _},
    dialog::overlay_color,
    h_flex,
    scroll::KaydirilabilirOge as _,
    title_bar::TITLE_BAR_HEIGHT,
    v_flex,
};

const CONTEXT: &str = "SayfaKatmani";
pub(crate) fn init(cx: &mut App) {
    cx.bind_keys([KeyBinding::new("escape", Cancel, Some(CONTEXT))])
}

/// ayarlar için sheets.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SayfaKatmaniAyarlari {
    /// Sayfa katmanının üst boşluğunu ayarlar. Varsayılan [`TITLE_BAR_HEIGHT`] değeridir.
    pub margin_top: Pixels,
}

impl Default for SayfaKatmaniAyarlari {
    fn default() -> Self {
        Self {
            margin_top: TITLE_BAR_HEIGHT,
        }
    }
}

/// Pencerenin bir tarafından içeri kayan SayfaKatmani bileşeni.
#[derive(IntoElement)]
pub struct SayfaKatmani {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) placement: Placement,
    pub(crate) size: DefiniteLength,
    resizable: bool,
    on_close: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
    title: Option<AnyElement>,
    footer: Option<AnyElement>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    overlay: bool,
    overlay_closable: bool,
}

impl SayfaKatmani {
    /// Yeni bir SayfaKatmani oluşturur.
    pub fn new(_: &mut Window, cx: &mut App) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            placement: Placement::Right,
            size: DefiniteLength::Absolute(px(350.).into()),
            resizable: true,
            title: None,
            footer: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
            overlay: true,
            overlay_closable: true,
            on_close: Rc::new(|_, _, _| {}),
        }
    }

    /// başlık sayfa katmanı ayarlar.
    pub fn title(mut self, title: impl IntoElement) -> Self {
        self.title = Some(title.into_any_element());
        self
    }

    /// alt bilgi sayfa katmanı ayarlar.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// Sayfa katmanı boyutunu ayarlar. Varsayılan 350pxdir.
    pub fn size(mut self, size: impl Into<DefiniteLength>) -> Self {
        self.size = size.into();
        self
    }

    /// Sayfa katmanının yeniden boyutlandırılabilir olup olmadığını ayarlar. Varsayılan `true`.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Sayfa katmanının kaplama kullanıp kullanmayacağını ayarlar. Varsayılan `true`.
    pub fn overlay(mut self, overlay: bool) -> Self {
        self.overlay = overlay;
        self
    }

    /// Kaplamaya tıklanınca sayfa katmanının kapatılıp kapatılmayacağını ayarlar. Varsayılan `true`.
    pub fn overlay_closable(mut self, overlay_closable: bool) -> Self {
        self.overlay_closable = overlay_closable;
        self
    }

    /// Dinler için kapatır olay sayfa katmanı.
    pub fn on_close(
        mut self,
        on_close: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Rc::new(on_close);
        self
    }
}

impl EventEmitter<DismissEvent> for SayfaKatmani {}
impl ParentElement for SayfaKatmani {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}
impl Styled for SayfaKatmani {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SayfaKatmani {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let placement = self.placement;
        let window_paddings = crate::window_border::window_paddings(window);
        let size = window.viewport_size()
            - gpui::size(
                window_paddings.left + window_paddings.right,
                window_paddings.top + window_paddings.bottom,
            );
        let top = cx.theme().sheet.margin_top;
        let on_close = self.on_close.clone();

        let base_size = window.text_style().font_size;
        let rem_size = window.rem_size();
        let mut paddings = Edges::all(px(16.));
        if let Some(pl) = self.style.padding.left {
            paddings.left = pl.to_pixels(base_size, rem_size);
        }
        if let Some(pr) = self.style.padding.right {
            paddings.right = pr.to_pixels(base_size, rem_size);
        }
        if let Some(pt) = self.style.padding.top {
            paddings.top = pt.to_pixels(base_size, rem_size);
        }
        if let Some(pb) = self.style.padding.bottom {
            paddings.bottom = pb.to_pixels(base_size, rem_size);
        }

        anchored()
            .position(point(window_paddings.left, window_paddings.top))
            .snap_to_window()
            .child(
                div()
                    .occlude()
                    .w(size.width)
                    .h(size.height)
                    .bg(overlay_color(self.overlay, cx))
                    .when(self.overlay, |this| {
                        this.window_control_area(WindowControlArea::Drag)
                            .on_any_mouse_down({
                                let on_close = self.on_close.clone();
                                move |event, window, cx| {
                                    if event.position.y < top {
                                        return;
                                    }

                                    cx.stop_propagation();
                                    if self.overlay_closable && event.button == MouseButton::Left {
                                        window.close_sheet(cx);
                                        on_close(&ClickEvent::default(), window, cx);
                                    }
                                }
                            })
                    })
                    .child(
                        v_flex()
                            .id("sheet")
                            .key_context(CONTEXT)
                            .track_focus(&self.focus_handle)
                            .focus_trap("sheet", &self.focus_handle)
                            .on_action({
                                let on_close = self.on_close.clone();
                                move |_: &Cancel, window, cx| {
                                    cx.propagate();

                                    window.close_sheet(cx);
                                    on_close(&ClickEvent::default(), window, cx);
                                }
                            })
                            .absolute()
                            .occlude()
                            .bg(cx.theme().background)
                            .border_color(cx.theme().border)
                            .shadow_xl()
                            .refine_style(&self.style)
                            .map(|this| {
                                // Set the size of the sheet.
                                if placement.is_horizontal() {
                                    this.w(self.size)
                                } else {
                                    this.h(self.size)
                                }
                            })
                            .map(|this| match self.placement {
                                Placement::Top => this.top(top).left_0().right_0().border_b_1(),
                                Placement::Right => this.top(top).right_0().bottom_0().border_l_1(),
                                Placement::Bottom => {
                                    this.bottom_0().left_0().right_0().border_t_1()
                                }
                                Placement::Left => this.top(top).left_0().bottom_0().border_r_1(),
                            })
                            .child(
                                // BaslikCubugu
                                h_flex()
                                    .justify_between()
                                    .pl_4()
                                    .pr_3()
                                    .py_2()
                                    .w_full()
                                    .font_semibold()
                                    .child(self.title.unwrap_or(div().into_any_element()))
                                    .child(
                                        Dugme::new("close")
                                            .small()
                                            .ghost()
                                            .icon(SimgeAdi::Close)
                                            .on_click(move |_, window, cx| {
                                                window.close_sheet(cx);
                                                on_close(&ClickEvent::default(), window, cx);
                                            }),
                                    ),
                            )
                            .child(
                                div().flex_1().overflow_hidden().child(
                                    // Body
                                    v_flex()
                                        .size_full()
                                        .overflow_y_scrollbar()
                                        .pl(paddings.left)
                                        .pr(paddings.right)
                                        .children(self.children),
                                ),
                            )
                            .when_some(self.footer, |this, footer| {
                                // Footer
                                this.child(
                                    h_flex()
                                        .justify_between()
                                        .px_4()
                                        .py_3()
                                        .w_full()
                                        .child(footer),
                                )
                            })
                            .on_any_mouse_down({
                                |_, _, cx| {
                                    cx.stop_propagation();
                                }
                            })
                            .with_animation(
                                "slide",
                                Animation::new(Duration::from_secs_f64(0.15)),
                                move |this, delta| {
                                    let y = px(-100.) + delta * px(100.);
                                    this.map(|this| match placement {
                                        Placement::Top => this.top(top + y),
                                        Placement::Right => this.right(y),
                                        Placement::Bottom => this.bottom(y),
                                        Placement::Left => this.left(y),
                                    })
                                },
                            ),
                    ),
            )
    }
}
