use gpui::*;
use kavis_ui::{
    input::{Input, InputEvent, InputState},
    *,
};
use kavis_ui_assets::Varliklar;

pub struct Example {
    input_state: Entity<InputState>,
    display_text: SharedString,

    /// Abonelikleri Example varlığıyla birlikte canlı tutmamız gerekir.
    ///
    /// Bu yüzden Example varlığı düşürülürse abonelikler de düşürülür.
    /// Bellek sızıntılarını önlemek için bu önemlidir.
    _subscriptions: Vec<Subscription>,
}

impl Example {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("Enter your name"));

        let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
            let input_state = input_state.clone();
            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    let value = input_state.read(cx).value();
                    this.display_text = format!("Hello, {}!", value).into();
                    cx.notify()
                }
                _ => {}
            }
        })];

        Self {
            input_state,
            display_text: SharedString::default(),
            _subscriptions,
        }
    }
}

impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p_5()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child(Input::new(&self.input_state))
            .child(self.display_text.clone())
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(Varliklar);

    app.run(move |cx| {
        // This must be called before using any Kavis UI features.
        kavis_ui::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(800.), px(600.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| Example::new(window, cx));
                // This first level on the window, should be a KokGorunum.
                cx.new(|cx| KokGorunum::new(view, window, cx))
            })
            .expect("Pencere açılamadı");
        })
        .detach();
    });
}
