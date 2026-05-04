use std::rc::Rc;

use gpui::{
    AnyElement, App, AppContext as _, Entity, IntoElement, SharedString, StyleRefinement, Styled,
    Subscription, Window, prelude::FluentBuilder as _,
};

use crate::{
    Boyutlandirilabilir, EksenUzantisi, StilUzantisi,
    input::{InputEvent, InputState, NumberInput, NumberInputEvent, StepAction},
    setting::{
        HerhangiBirAyarAlani, RenderOptions,
        fields::{AyarAlaniCizimi, get_value, set_value},
    },
};

#[derive(Clone, Debug)]
pub struct NumberFieldOptions {
    /// Sayı girdisinin minimum değeri. Varsayılan `f64::MIN` değeridir.
    pub min: f64,
    /// Sayı girdisinin maksimum değeri. Varsayılan `f64::MAX` değeridir.
    pub max: f64,
    /// Sayı girdisinin adım değeri. Varsayılan `1.0` değeridir.
    pub step: f64,
}

impl Default for NumberFieldOptions {
    fn default() -> Self {
        Self {
            min: f64::MIN,
            max: f64::MAX,
            step: 1.0,
        }
    }
}

pub(crate) struct NumberField {
    options: NumberFieldOptions,
}

impl NumberField {
    pub(crate) fn new(options: Option<&NumberFieldOptions>) -> Self {
        Self {
            options: options.cloned().unwrap_or_default(),
        }
    }
}

struct State {
    input: Entity<InputState>,
    initial_value: f64,
    _subscriptions: Vec<Subscription>,
}

impl AyarAlaniCizimi for NumberField {
    fn render(
        &self,
        field: Rc<dyn HerhangiBirAyarAlani>,
        options: &RenderOptions,
        style: &StyleRefinement,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        let value = get_value::<f64>(&field, cx);
        let set_value = set_value::<f64>(&field, cx);
        let num_options = self.options.clone();

        let state = window
            .use_keyed_state(
                SharedString::from(format!(
                    "number-state-{}-{}-{}",
                    options.page_ix, options.group_ix, options.item_ix
                )),
                cx,
                |window, cx| {
                    let input =
                        cx.new(|cx| InputState::new(window, cx).default_value(value.to_string()));
                    let _subscriptions = vec![
                        cx.subscribe_in(&input, window, {
                            move |_, input, event: &NumberInputEvent, window, cx| match event {
                                NumberInputEvent::Step(action) => input.update(cx, |input, cx| {
                                    let value = input.value();
                                    if let Ok(value) = value.parse::<f64>() {
                                        let new_value = if *action == StepAction::Increment {
                                            value + num_options.step
                                        } else {
                                            value - num_options.step
                                        };
                                        input.set_value(
                                            SharedString::from(new_value.to_string()),
                                            window,
                                            cx,
                                        );
                                    }
                                }),
                            }
                        }),
                        cx.subscribe_in(&input, window, {
                            move |state: &mut State, input, event: &InputEvent, window, cx| {
                                match event {
                                    InputEvent::Change => {
                                        input.update(cx, |input, cx| {
                                            let value = input.value();
                                            if value == state.initial_value.to_string() {
                                                return;
                                            }

                                            if let Ok(value) = value.parse::<f64>() {
                                                let clamp_value =
                                                    value.clamp(num_options.min, num_options.max);

                                                set_value(clamp_value, cx);
                                                state.initial_value = clamp_value;
                                                if clamp_value != value {
                                                    input.set_value(
                                                        SharedString::from(clamp_value.to_string()),
                                                        window,
                                                        cx,
                                                    );
                                                }
                                            }
                                        });
                                    }
                                    _ => {}
                                }
                            }
                        }),
                    ];

                    State {
                        input,
                        initial_value: value,
                        _subscriptions,
                    }
                },
            )
            .read(cx);

        NumberInput::new(&state.input)
            .with_size(options.size)
            .map(|this| {
                if options.layout.yatay_mi() {
                    this.w_32()
                } else {
                    this.w_full()
                }
            })
            .refine_style(style)
            .into_any_element()
    }
}
