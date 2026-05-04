use gpui::*;
use kavis_ui::{
    BaslikCubugu, KokGorunum, PencereUzantisi as _, StilUzantisi as _, button::Dugme, h_flex,
    menu::BaglamMenusuUzantisi,
};
use kavis_ui_assets::Varliklar;

actions!(class_menu, [Open, Delete, Export, Info]);

pub struct HelloWorld;

impl HelloWorld {
    fn show_dialog(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.open_dialog(cx, move |dialog, _, _| {
            dialog.title("Test dialog").child("Hello from dialog!")
        });
    }

    fn show_drawer(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.open_sheet(cx, move |drawer, _, _| {
            drawer.title("Test Drawer").child("Hello from Drawer!")
        });
    }
}

impl Render for HelloWorld {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(gpui::white())
            .size_full()
            .child(BaslikCubugu::new().child("dialog & Drawer"))
            .child(
                div()
                    .p_8()
                    .v_flex()
                    .gap_2()
                    .size_full()
                    .child(
                        h_flex()
                            .gap_4()
                            .child(
                                Dugme::new("btn1")
                                    .outline()
                                    .label("Open dialog")
                                    .on_click(cx.listener(Self::show_dialog)),
                            )
                            .child(
                                Dugme::new("btn2")
                                    .outline()
                                    .label("Open Drawer")
                                    .on_click(cx.listener(Self::show_drawer)),
                            ),
                    )
                    .child(
                        div()
                            .id("second-area")
                            .v_flex()
                            .h_40()
                            .border_1()
                            .border_dashed()
                            .border_color(gpui::black())
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(gpui::yellow().opacity(0.2)))
                            .child("Hover test here.")
                            .child("Right click to show Context Menu")
                            .baglam_menusu({
                                move |this, _, _| {
                                    this.separator()
                                        .menu("Open", Box::new(Open))
                                        .menu("Delete", Box::new(Delete))
                                        .menu("Export", Box::new(Export))
                                        .menu("Info", Box::new(Info))
                                        .separator()
                                }
                            }),
                    ),
            )
            .children(KokGorunum::render_dialog_layer(window, cx))
            .children(KokGorunum::render_sheet_layer(window, cx))
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(Varliklar);

    app.run(move |cx| {
        kavis_ui::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    titlebar: Some(BaslikCubugu::title_bar_options()),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|_| HelloWorld);
                    // This first level on the window, should be a KokGorunum.
                    cx.new(|cx| KokGorunum::new(view, window, cx))
                },
            )
            .expect("Pencere açılamadı");
        })
        .detach();
    });
}
