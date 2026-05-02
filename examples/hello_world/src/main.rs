use gpui::*;
use kavis_ui::{button::*, *};

pub struct Example;
impl Render for Example {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Dugme::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("Clicked!")),
            )
    }
}

fn main() {
    gpui_platform::application().run(move |cx| {
        // This must be called before using any Kavis UI features.
        kavis_ui::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| Example);
                // This first level on the window, should be a KokGorunum.
                cx.new(|cx| {
                    // You can refine the root view style by yourself.
                    KokGorunum::new(view, window, cx).bg(cx.theme().background)
                })
            })
            .expect("Pencere açılamadı");
        })
        .detach();
    });
}
