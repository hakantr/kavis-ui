use gpui::{prelude::FluentBuilder as _, *};
use kavis_ui::{
    EtkinTema as _, SimgeAdi, Sizable as _,
    button::{Dugme, DugmeVaryantlari as _},
    clipboard::Pano,
    h_flex,
    highlighter::Language,
    input::{Input, InputEvent, InputState, TabSize},
    resizable::{h_resizable, resizable_panel},
    text::markdown,
};
use kavis_ui_assets::Varliklar;
use kavis_ui_story::Open;

pub struct Example {
    input_state: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

const EXAMPLE: &str = include_str!("./fixtures/test.md");

impl Example {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor(Language::Markdown)
                .line_number(true)
                .tab_size(TabSize {
                    tab_size: 2,
                    ..Default::default()
                })
                .searchable(true)
                .placeholder("Enter your Markdown here...")
                .default_value(EXAMPLE)
        });

        // Focus the input on startup so that actions (e.g. Open) can bubble
        // up through this view's element tree and reach their handlers.
        let focus_handle = input_state.focus_handle(cx);
        window.defer(cx, move |window, cx| {
            focus_handle.focus(window, cx);
        });

        let _subscriptions = vec![cx.subscribe(&input_state, |_, _, _: &InputEvent, _| {})];

        Self {
            input_state,
            _subscriptions,
        }
    }

    fn on_action_open(&mut self, _: &Open, window: &mut Window, cx: &mut Context<Self>) {
        let path = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: true,
            multiple: false,
            prompt: Some("Secim a Markdown file".into()),
        });

        let input_state = self.input_state.clone();
        cx.spawn_in(window, async move |_, window| {
            let path = path.await.ok()?.ok()??.iter().next()?.clone();

            let content = std::fs::read_to_string(&path).ok()?;

            window
                .update(|window, cx| {
                    _ = input_state.update(cx, |this, cx| {
                        this.set_value(content, window, cx);
                    });
                })
                .ok();

            Some(())
        })
        .detach();
    }

    fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for Example {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("editor")
            .size_full()
            .on_action(cx.listener(Self::on_action_open))
            .child(
                h_resizable("container")
                    .child(
                        resizable_panel().child(
                            div()
                                .id("source")
                                .size_full()
                                .font_family(cx.theme().mono_font_family.clone())
                                .text_size(cx.theme().mono_font_size)
                                .child(
                                    Input::new(&self.input_state)
                                        .h_full()
                                        .p_0()
                                        .border_0()
                                        .focus_bordered(false),
                                ),
                        ),
                    )
                    .child(
                        resizable_panel().child(
                            markdown(self.input_state.read(cx).value().clone())
                                .code_block_actions(|code_block, _window, _cx| {
                                    let code = code_block.code();
                                    let lang = code_block.lang();

                                    h_flex()
                                        .gap_1()
                                        .child(Pano::new("copy").value(code.clone()))
                                        .when_some(lang, |this, lang| {
                                            // Only show run terminal button for certain languages
                                            if lang.as_ref() == "rust" || lang.as_ref() == "python"
                                            {
                                                this.child(
                                                    Dugme::new("run-terminal")
                                                        .icon(SimgeAdi::SquareTerminal)
                                                        .ghost()
                                                        .xsmall()
                                                        .on_click(move |_, _, _cx| {
                                                            println!(
                                                                "Running {} code: {}",
                                                                lang, code
                                                            );
                                                        }),
                                                )
                                            } else {
                                                this
                                            }
                                        })
                                })
                                .flex_none()
                                .p_5()
                                .scrollable(true)
                                .selectable(true),
                        ),
                    ),
            )
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(Varliklar);

    app.run(move |cx| {
        kavis_ui_story::init(cx);
        cx.activate(true);

        kavis_ui_story::create_new_window("Markdown Editor", Example::view, cx);
    });
}
