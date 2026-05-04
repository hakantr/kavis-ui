use gpui::{
    AnyElement, App, Axis, ClickEvent, Half, InteractiveElement as _, IntoElement, ParentElement,
    Pixels, RenderOnce, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px,
    relative,
};

use crate::{
    AxisExt, BilesenBoyutu, Boyutlandirilabilir, EtkinTema as _, Simge, StilUzantisi as _,
    stepper::trigger::AdimlayiciTetikleyicisi,
};

/// Bir step öğe içinde bir [`Adimlayici`].
#[derive(IntoElement)]
pub struct AdimlayiciOgesi {
    step: usize,
    checked_step: usize,
    style: StyleRefinement,
    icon: Option<Simge>,
    children: Vec<AnyElement>,
    layout: Axis,
    disabled: bool,
    size: BilesenBoyutu,
    is_last: bool,
    text_center: bool,
    on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
}

impl AdimlayiciOgesi {
    pub fn new() -> Self {
        Self {
            step: 0,
            checked_step: 0,
            style: StyleRefinement::default(),
            icon: None,
            layout: Axis::Horizontal,
            disabled: false,
            size: BilesenBoyutu::default(),
            is_last: false,
            text_center: false,
            children: Vec::new(),
            on_click: Box::new(|_, _, _| {}),
        }
    }

    /// simge adımlayıcı öğe ayarlar.
    pub fn icon(mut self, icon: impl Into<Simge>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// devre dışı durum adımlayıcı öğe ayarlar.
    ///
    /// true olarak ayarlanırsa adımlayıcının devre dışı durumunu geçersiz kılar.
    ///
    /// Varsayılan false değeridir.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub(super) fn text_center(mut self, center: bool) -> Self {
        self.text_center = center;
        self
    }

    pub(super) fn step(mut self, ix: usize) -> Self {
        self.step = ix;
        self
    }

    pub(super) fn checked_step(mut self, checked_step: usize) -> Self {
        self.checked_step = checked_step;
        self
    }

    pub(super) fn layout(mut self, layout: Axis) -> Self {
        self.layout = layout;
        self
    }

    pub(super) fn is_last(mut self, is_last: bool) -> Self {
        self.is_last = is_last;
        self
    }

    pub(super) fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    {
        self.on_click = Box::new(f);
        self
    }
}

impl ParentElement for AdimlayiciOgesi {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Boyutlandirilabilir for AdimlayiciOgesi {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for AdimlayiciOgesi {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AdimlayiciOgesi {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let is_passed = self.step < self.checked_step;
        let icon_size = match self.size {
            BilesenBoyutu::CokKucuk => px(8.),
            BilesenBoyutu::Kucuk => px(18.),
            BilesenBoyutu::Buyuk => px(32.),
            _ => px(24.),
        };

        div()
            .id(("stepper-item", self.step))
            .relative()
            .when(self.layout.is_horizontal(), |this| this.h_flex())
            .when(self.layout.is_vertical(), |this| this.v_flex())
            .when(!self.is_last, |this| this.flex_1())
            .when(self.text_center, |this| this.flex_1().justify_center())
            .items_start()
            .refine_style(&self.style)
            .child(
                AdimlayiciTetikleyicisi::new()
                    .icon(self.icon)
                    .icon_size(icon_size)
                    .step(self.step)
                    .with_size(self.size)
                    .checked_step(self.checked_step)
                    .text_center(self.text_center)
                    .layout(self.layout)
                    .disabled(self.disabled)
                    .children(self.children)
                    .on_click({
                        let on_click = self.on_click;
                        move |e, window, cx| {
                            on_click(e, window, cx);
                        }
                    }),
            )
            .when(!self.is_last, |this| {
                this.child(
                    StepperSeparator::new()
                        .with_size(self.size)
                        .layout(self.layout)
                        .text_center(self.text_center)
                        .icon_size(icon_size)
                        .checked(is_passed),
                )
            })
    }
}

/// Bir ayırıcı arasında adımlayıcı öğeler.
///
/// Varsayılan olarak `absolute` konumlandırılır.
#[derive(IntoElement)]
struct StepperSeparator {
    size: BilesenBoyutu,
    checked: bool,
    icon_size: Pixels,
    layout: Axis,
    style: StyleRefinement,
    text_center: bool,
}

impl StepperSeparator {
    fn new() -> Self {
        Self {
            size: BilesenBoyutu::default(),
            checked: false,
            icon_size: px(24.),
            layout: Axis::Horizontal,
            style: StyleRefinement::default(),
            text_center: false,
        }
    }

    fn with_size(mut self, size: BilesenBoyutu) -> Self {
        self.size = size;
        self
    }

    fn text_center(mut self, center: bool) -> Self {
        self.text_center = center;
        self
    }

    fn layout(mut self, layout: Axis) -> Self {
        self.layout = layout;
        self
    }

    fn icon_size(mut self, size: Pixels) -> Self {
        self.icon_size = size;
        self
    }

    fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
}

impl Styled for StepperSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for StepperSeparator {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let icon_size = self.icon_size;
        let text_center = self.text_center;
        let separator_wide = match self.size {
            BilesenBoyutu::CokKucuk => px(1.5),
            BilesenBoyutu::Buyuk => px(3.),
            _ => px(2.),
        };

        let gap = px(4.);

        div()
            .absolute()
            .flex_1()
            .when(self.layout.is_horizontal(), |this| {
                this.h(separator_wide).mt(icon_size.half()).map(|this| {
                    if !text_center {
                        this.ml(icon_size + gap).mr(gap).left_0().right_0()
                    } else {
                        this.mx(icon_size.half() + gap)
                            .left(relative(0.5))
                            .right(relative(-0.5))
                    }
                })
            })
            .when(self.layout.is_vertical(), |this| {
                this.w(separator_wide).ml(icon_size.half()).map(|this| {
                    if !text_center {
                        this.mt(icon_size + gap).mb(gap).top_0().bottom_0()
                    } else {
                        this.mx(icon_size.half() + gap)
                            .top(relative(0.5))
                            .bottom(relative(-0.5))
                    }
                })
            })
            .refine_style(&self.style)
            .bg(cx.theme().border)
            .when(self.checked, |this| this.bg(cx.theme().primary))
    }
}
