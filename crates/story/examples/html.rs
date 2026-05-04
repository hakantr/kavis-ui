use kavis_ui::ham_gpui::*;
use kavis_ui::{
    EtkinTema as _,
    highlighter::Language,
    input::{Girdi, GirdiDurumu, SekmeBoyutu},
    resizable::h_resizable,
    text::html,
};
use kavis_ui_assets::Varliklar;

pub struct Example {
    input_state: Entity<GirdiDurumu>,
    _subscribe: Subscription,
}

const EXAMPLE: &str = include_str!("./fixtures/test.html");

impl Example {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| {
            GirdiDurumu::new(window, cx)
                .code_editor(Language::Html)
                .tab_size(SekmeBoyutu {
                    tab_size: 4,
                    hard_tabs: false,
                })
                .default_value(EXAMPLE)
                .placeholder("Enter your HTML here...")
        });

        let _subscribe = cx.subscribe(&input_state, |_, _, _: &kavis_ui::input::GirdiOlayi, cx| {
            cx.notify();
        });

        Self {
            input_state,
            _subscribe,
        }
    }

    fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for Example {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_resizable("container")
            .child(
                div()
                    .id("source")
                    .size_full()
                    .font_family(cx.theme().mono_font_family.clone())
                    .text_size(cx.theme().mono_font_size)
                    .child(
                        Girdi::new(&self.input_state)
                            .h_full()
                            .appearance(false)
                            .focus_bordered(false),
                    )
                    .into_any(),
            )
            .child(
                html(self.input_state.read(cx).value().clone())
                    .p_5()
                    .scrollable(true)
                    .selectable(true)
                    .into_any(),
            )
    }
}

fn main() {
    let app = kavis_ui::platform::application().with_assets(Varliklar);

    app.run(move |cx| {
        kavis_ui_story::init(cx);
        cx.activate(true);

        kavis_ui_story::create_new_window("HTML Render (native)", Example::view, cx);
    });
}
