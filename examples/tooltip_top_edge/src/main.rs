use gpui::*;
use kavis_ui::{EtkinTema as _, KokGorunum, button::*};

struct TooltipTopEdgeExample;

impl Render for TooltipTopEdgeExample {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .bg(cx.theme().background)
            // Keep the trigger pinned to the top edge to exercise tooltip flipping.
            .child(
                div().absolute().top_0().left(px(24.)).child(
                    Dugme::new("top-edge-tooltip")
                        .primary()
                        .label("Hover for tooltip")
                        .tooltip("This tooltip should appear below the trigger near the top edge."),
                ),
            )
            .child(
                div()
                    .absolute()
                    .top(px(64.))
                    .left(px(24.))
                    .max_w(px(420.))
                    .text_color(cx.theme().muted_foreground)
                    .child(
                        "Hover the top button. The tooltip should flip below the trigger without changing the original visual gap.",
                    ),
            )
    }
}

fn main() {
    let app = gpui_platform::application();

    app.run(move |cx| {
        kavis_ui::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(520.), px(260.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|_| TooltipTopEdgeExample);
                cx.new(|cx| KokGorunum::new(view, window, cx).bg(cx.theme().background))
            })
            .expect("Pencere açılamadı");
        })
        .detach();
    });
}
