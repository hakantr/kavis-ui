use kavis_ui::ham_gpui::*;
use kavis_ui::{
    BaslikCubugu, KokGorunum,
    button::{Dugme, DugmeVaryantlari},
    h_flex, v_flex,
};

pub struct Example;
impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(
                // Render custom title bar on top of KokGorunum view.
                BaslikCubugu::new().child(
                    h_flex()
                        .w_full()
                        .pr_2()
                        .justify_between()
                        .child("App with Custom title bar")
                        .child("Right Item"),
                ),
            )
            .child(
                div()
                    .id("window-body")
                    .p_5()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .child("Hello, World!")
                    .child(
                        Dugme::new("ok")
                            .primary()
                            .label("Let's Go!")
                            .on_click(|_, _, _| println!("Clicked!")),
                    ),
            )
    }
}

fn main() {
    let app = kavis_ui::platform::application().with_assets(kavis_ui_assets::Varliklar);

    app.run(move |cx| {
        kavis_ui::init(cx);

        cx.spawn(async move |cx| {
            let window_options = WindowOptions {
                // Setup GPUI to use custom title bar
                titlebar: Some(BaslikCubugu::title_bar_options()),
                ..Default::default()
            };

            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|_| Example);
                cx.new(|cx| KokGorunum::new(view, window, cx))
            })
            .expect("Pencere açılamadı");
        })
        .detach();
    });
}
