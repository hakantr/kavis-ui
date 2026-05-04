use crate::theme::EtkinTema;
use crate::ham_gpui::Corners;
use crate::ham_gpui::Window;
use crate::ham_gpui::{AnyElement, App, Context, Edges, Entity, EventEmitter, FocusHandle, Focusable};
use crate::ham_gpui::{
    InteractiveElement, IntoElement, KeyBinding, ParentElement, RenderOnce, SharedString,
    StyleRefinement, Styled, TextAlign, actions, prelude::FluentBuilder as _,
};

use crate::{
    BilesenBoyutu, Boyutlandirilabilir, DevreDisiBirakilabilir, SimgeAdi, StilUzantisi as _,
    button::Dugme, h_flex,
};

use super::{Input, InputState};

actions!(number_input, [Increment, Decrement]);

const CONTEXT: &str = "NumberInput";
pub fn init(cx: &mut App) {
    cx.bind_keys(vec![
        KeyBinding::new("up", Increment, Some(CONTEXT)),
        KeyBinding::new("down", Decrement, Some(CONTEXT)),
    ]);
}

/// Bir sayı girdi öğe ile increment ve decrement düğmeler.
#[derive(IntoElement)]
pub struct NumberInput {
    state: Entity<InputState>,
    placeholder: SharedString,
    size: BilesenBoyutu,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    appearance: bool,
    disabled: bool,
    style: StyleRefinement,
}

impl NumberInput {
    /// Yeni bir [`NumberInput`] öğe bind için [`InputState`] oluşturur.
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            size: BilesenBoyutu::default(),
            placeholder: SharedString::default(),
            prefix: None,
            suffix: None,
            appearance: true,
            disabled: false,
            style: StyleRefinement::default(),
        }
    }

    /// yer tutucu metin sayı girdi ayarlar.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// ön ek öğe sayı girdi ayarlar.
    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.prefix = Some(prefix.into_any_element());
        self
    }

    /// son ek öğe sayı girdi ayarlar.
    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_any_element());
        self
    }

    /// Sayı girdisinin görünümünü ayarlar; false ise kenarlık ve arka plan olmaz.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }

    fn on_increment(state: &Entity<InputState>, window: &mut Window, cx: &mut App) {
        state.update(cx, |state, cx| {
            state.focus(window, cx);
            state.on_action_increment(&Increment, window, cx);
        })
    }

    fn on_decrement(state: &Entity<InputState>, window: &mut Window, cx: &mut App) {
        state.update(cx, |state, cx| {
            state.focus(window, cx);
            state.on_action_decrement(&Decrement, window, cx);
        })
    }
}

impl DevreDisiBirakilabilir for NumberInput {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl InputState {
    fn on_action_increment(&mut self, _: &Increment, window: &mut Window, cx: &mut Context<Self>) {
        self.on_number_input_step(StepAction::Increment, window, cx);
    }

    fn on_action_decrement(&mut self, _: &Decrement, window: &mut Window, cx: &mut Context<Self>) {
        self.on_number_input_step(StepAction::Decrement, window, cx);
    }

    fn on_number_input_step(&mut self, action: StepAction, _: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        cx.emit(NumberInputEvent::Step(action));
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StepAction {
    Decrement,
    Increment,
}
pub enum NumberInputEvent {
    Step(StepAction),
}
impl EventEmitter<NumberInputEvent> for InputState {}

impl Focusable for NumberInput {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.state.focus_handle(cx)
    }
}

impl Boyutlandirilabilir for NumberInput {
    fn with_size(mut self, size: impl Into<BilesenBoyutu>) -> Self {
        self.size = size.into();
        self
    }
}

impl Styled for NumberInput {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NumberInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .id(("number-input", self.state.entity_id()))
            .key_context(CONTEXT)
            .on_action(window.listener_for(&self.state, InputState::on_action_increment))
            .on_action(window.listener_for(&self.state, InputState::on_action_decrement))
            .flex_1()
            .rounded(cx.theme().radius)
            .refine_style(&self.style)
            .when(self.disabled, |this| this.opacity(0.5))
            .child(
                Dugme::new("minus")
                    .outline()
                    .with_size(self.size)
                    .icon(SimgeAdi::Minus)
                    .compact()
                    .tab_stop(false)
                    .disabled(self.disabled)
                    .border_color(cx.theme().input)
                    .border_corners(Corners {
                        top_left: true,
                        top_right: false,
                        bottom_right: false,
                        bottom_left: true,
                    })
                    .border_edges(Edges {
                        top: self.appearance,
                        right: false,
                        bottom: self.appearance,
                        left: self.appearance,
                    })
                    .on_click({
                        let state = self.state.clone();
                        move |_, window, cx| {
                            Self::on_decrement(&state, window, cx);
                        }
                    }),
            )
            .child(
                Input::new(&self.state)
                    .appearance(self.appearance)
                    .with_size(self.size)
                    .disabled(self.disabled)
                    .gap_0()
                    .rounded_none()
                    .text_align(TextAlign::Center)
                    .when_some(self.prefix, |this, prefix| this.prefix(prefix))
                    .when_some(self.suffix, |this, suffix| this.suffix(suffix)),
            )
            .child(
                Dugme::new("plus")
                    .outline()
                    .with_size(self.size)
                    .icon(SimgeAdi::Plus)
                    .compact()
                    .tab_stop(false)
                    .disabled(self.disabled)
                    .border_color(cx.theme().input)
                    .border_corners(Corners {
                        top_left: false,
                        top_right: true,
                        bottom_right: true,
                        bottom_left: false,
                    })
                    .border_edges(Edges {
                        top: self.appearance,
                        right: self.appearance,
                        bottom: self.appearance,
                        left: false,
                    })
                    .on_click({
                        let state = self.state.clone();
                        move |_, window, cx| {
                            Self::on_increment(&state, window, cx);
                        }
                    }),
            )
    }
}
