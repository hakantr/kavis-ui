use kavis_ui_assets::Varliklar;
use kavis_ui_story::{Gallery, create_new_window, init};

fn main() {
    let app = kavis_ui::platform::application().with_assets(Varliklar);

    // Parse `cargo run -- <story_name>`
    let name = std::env::args().nth(1);

    app.run(move |cx| {
        init(cx);
        cx.activate(true);

        create_new_window(
            "Kavis UI",
            move |window, cx| Gallery::view(name.as_deref(), window, cx),
            cx,
        );
    });
}
