use gpui::{
    Action, Anchor, App, AppContext, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, Half, InteractiveElement, IntoElement, KeyBinding, MouseButton, ParentElement as _,
    Render, Styled as _, Window, actions, div, px,
};
use kavis_ui::{
    EtkinTema, PencereUzantisi, StilUzantisi,
    button::{Dugme, DugmeVaryantlari as _},
    divider::Ayirici,
    h_flex,
    input::{Input, InputState},
    list::{Liste, ListeDurumu, ListeOgesi, ListeTemsilcisi},
    popover::AcilirKatman,
    v_flex,
};
use serde::Deserialize;

use crate::section;

#[derive(Action, Clone, PartialEq, Eq, Deserialize)]
#[action(namespace = popover_story, no_json)]
struct Info(usize);

actions!(popover_story, [Copy, Paste, Cut, SearchAll, ToggleCheck]);
const CONTEXT: &str = "popover-story";
pub fn init(cx: &mut App) {
    cx.bind_keys([
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-c", Copy, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", Paste, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-x", Cut, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-shift-f", SearchAll, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-shift-f", SearchAll, Some(CONTEXT)),
    ])
}

struct Form {
    parent: Entity<PopoverStory>,
    input1: Entity<InputState>,
}

impl Form {
    fn new(parent: Entity<PopoverStory>, window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self {
            parent,
            input1: cx.new(|cx| InputState::new(window, cx)),
        })
    }
}

impl Focusable for Form {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.input1.focus_handle(cx)
    }
}

struct DropdownListDelegate {
    parent: Entity<PopoverStory>,
}
impl ListeTemsilcisi for DropdownListDelegate {
    type Item = ListeOgesi;

    fn items_count(&self, _: usize, _: &App) -> usize {
        10
    }

    fn render_item(
        &mut self,
        ix: kavis_ui::IndexPath,
        _: &mut Window,
        _: &mut Context<ListeDurumu<Self>>,
    ) -> Option<Self::Item> {
        Some(ListeOgesi::new(ix).child(format!("Item {}", ix.row)))
    }

    fn set_selected_index(
        &mut self,
        _: Option<kavis_ui::IndexPath>,
        _: &mut Window,
        _: &mut Context<kavis_ui::list::ListeDurumu<Self>>,
    ) {
    }

    fn confirm(&mut self, _: bool, _: &mut Window, cx: &mut Context<ListeDurumu<Self>>) {
        self.parent.update(cx, |this, cx| {
            this.list_popover_open = false;
            cx.notify();
        })
    }

    fn cancel(&mut self, _: &mut Window, cx: &mut Context<ListeDurumu<Self>>) {
        self.parent.update(cx, |this, cx| {
            this.list_popover_open = false;
            cx.notify();
        })
    }
}

impl EventEmitter<DismissEvent> for Form {}

impl Render for Form {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let parent = self.parent.clone();
        v_flex()
            .gap_2()
            .p_3()
            .size_full()
            .child("Bu bir form kapsayıcısıdır.")
            .child("Açılır içeriği kapatmak için gönder düğmesine tıklayın.")
            .child(Input::new(&self.input1))
            .child(
                Dugme::new("submit")
                    .label("Gönder")
                    .primary()
                    .on_click(cx.listener(move |_, _, _, cx| {
                        parent.update(cx, |this, cx| {
                            this.form_popover_open = false;
                            cx.notify();
                        })
                    })),
            )
    }
}

pub struct PopoverStory {
    focus_handle: FocusHandle,
    form: Entity<Form>,
    list: Entity<ListeDurumu<DropdownListDelegate>>,
    form_popover_open: bool,
    list_popover_open: bool,
    checked: bool,
    message: String,
}

impl super::Story for PopoverStory {
    fn title() -> &'static str {
        "Açılır İçerik"
    }

    fn description() -> &'static str {
        "A popup displays content on top of the main page."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl PopoverStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let form = Form::new(cx.entity(), window, cx);
        let parent = cx.entity();
        let list = cx.new(|cx| {
            ListeDurumu::new(DropdownListDelegate { parent }, window, cx).searchable(true)
        });

        cx.focus_self(window);

        Self {
            form,
            list,
            checked: true,
            form_popover_open: false,
            list_popover_open: false,
            focus_handle: cx.focus_handle(),
            message: "".to_string(),
        }
    }

    fn on_copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        self.message = "Kopyala seçeneğine tıkladınız".to_string();
        cx.notify()
    }

    fn on_cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        self.message = "Kes seçeneğine tıkladınız".to_string();
        cx.notify()
    }

    fn on_paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        self.message = "Yapıştır seçeneğine tıkladınız".to_string();
        cx.notify()
    }

    fn on_search_all(&mut self, _: &SearchAll, _: &mut Window, cx: &mut Context<Self>) {
        self.message = "Tümünde ara seçeneğine tıkladınız".to_string();
        cx.notify()
    }

    fn on_action_info(&mut self, info: &Info, _: &mut Window, cx: &mut Context<Self>) {
        self.message = format!("Bilgi seçeneğine tıkladınız: {}", info.0);
        cx.notify()
    }

    fn on_action_toggle_check(&mut self, _: &ToggleCheck, _: &mut Window, cx: &mut Context<Self>) {
        self.checked = !self.checked;
        self.message = format!("Onay geçişine tıkladınız: {}", self.checked);
        cx.notify()
    }
}

impl Focusable for PopoverStory {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PopoverStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let form = self.form.clone();

        v_flex()
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_copy))
            .on_action(cx.listener(Self::on_cut))
            .on_action(cx.listener(Self::on_paste))
            .on_action(cx.listener(Self::on_search_all))
            .on_action(cx.listener(Self::on_action_info))
            .on_action(cx.listener(Self::on_action_toggle_check))
            .size_full()
            .gap_6()
            .child(
                section("Basic AcilirKatman").child(
                    AcilirKatman::new("popover-0")
                        .max_w(px(600.))
                        .trigger(Dugme::new("btn").outline().label("Açılır İçerik"))
                        .gap_2()
                        .text_sm()
                        .w(px(400.))
                        .child("Merhaba, bu bir açılır içeriktir.")
                        .child(Ayirici::horizontal())
                        .child(
                            "You can put any content here, including text,\
                            buttons, forms, and more.",
                        ),
                ),
            )
            .child(
                section("AcilirKatman with Form").child(
                    AcilirKatman::new("popover-form")
                        .p_0()
                        .text_sm()
                        .trigger(Dugme::new("pop").outline().label("Açılır Form"))
                        .track_focus(&form.focus_handle(cx))
                        .open(self.form_popover_open)
                        .on_open_change(cx.listener(move |this, open, _, cx| {
                            println!("Açılır form açık durumu değişti: {}", open);
                            this.form_popover_open = *open;
                            cx.notify();
                        }))
                        .child(form.clone()),
                ),
            )
            .child(
                section("AcilirKatman with Liste").child(
                    AcilirKatman::new("popover-list")
                        .p_0()
                        .text_sm()
                        .open(self.list_popover_open)
                        .on_open_change(cx.listener(move |this, open, _, cx| {
                            this.list_popover_open = *open;
                            cx.notify();
                        }))
                        .trigger(Dugme::new("pop").outline().label("Açılır Liste"))
                        .track_focus(&self.list.focus_handle(cx))
                        .child(Liste::new(&self.list))
                        .w_64()
                        .h(px(200.)),
                ),
            )
            .child(
                section("Right click to open AcilirKatman").child(
                    AcilirKatman::new("popover-right-click")
                        .mouse_button(MouseButton::Right)
                        .trigger(Dugme::new("btn").outline().label("Sağ Tık Açılır İçerik"))
                        .max_w(px(600.))
                        .content(|_, _, cx| {
                            v_flex()
                                .gap_2()
                                .child("Merhaba, bu sağ altta açılan bir içeriktir.")
                                .child(Ayirici::horizontal())
                                .child(
                                    Dugme::new("info1")
                                        .primary()
                                        .label("Kapat")
                                        .w(px(80.))
                                        .on_click(cx.listener(|_, _, window, cx| {
                                            window.push_notification(
                                                "DismissEvent ile kapatmaya tıkladınız.",
                                                cx,
                                            );
                                            cx.emit(DismissEvent);
                                        })),
                                )
                        }),
                ),
            )
            .child(
                section("Styling AcilirKatman").child(
                    AcilirKatman::new("popover-1")
                        .trigger(
                            Dugme::new("btn")
                                .outline()
                                .label("Açılır İçeriği Stillendir"),
                        )
                        .appearance(false)
                        .py_1()
                        .px_2()
                        .bg(cx.theme().primary)
                        .text_color(cx.theme().primary_foreground)
                        .max_w(px(600.))
                        .rounded(cx.theme().radius.half())
                        .text_sm()
                        .shadow_2xl()
                        .child(
                            "Özel arka plan ve metin rengine sahip stillendirilmiş açılır içerik.",
                        ),
                ),
            )
            .child(
                section("Varsayılan Açık").child(
                    AcilirKatman::new("default-open-popover")
                        .default_open(true)
                        .trigger(
                            Dugme::new("default-open-btn")
                                .label("Varsayılan Açık")
                                .outline(),
                        )
                        .child(
                            "Bu açılır içerik ilk render edildiğinde varsayılan olarak açıktır.",
                        ),
                ),
            )
            .child(
                section("AcilirKatman Anchor")
                    .min_h(px(360.))
                    .v_flex()
                    .child(
                        div().absolute().top_0().left_0().w_full().h_10().child(
                            h_flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    AcilirKatman::new("anchor-top-left")
                                        .max_w(px(600.))
                                        .anchor(Anchor::TopLeft)
                                        .trigger(Dugme::new("btn").outline().label("ÜstSol"))
                                        .child("Bu üst solda açılan bir içeriktir."),
                                )
                                .child(
                                    AcilirKatman::new("anchor-top-center")
                                        .max_w(px(600.))
                                        .anchor(Anchor::TopCenter)
                                        .trigger(Dugme::new("btn").outline().label("ÜstOrta"))
                                        .child("Bu üst ortada açılan bir içeriktir."),
                                )
                                .child(
                                    AcilirKatman::new("anchor-top-right")
                                        .anchor(Anchor::TopRight)
                                        .trigger(Dugme::new("btn").outline().label("ÜstSağ"))
                                        .child("Bu üst sağda açılan bir içeriktir."),
                                ),
                        ),
                    )
                    .child(
                        div().absolute().bottom_0().left_0().w_full().h_10().child(
                            h_flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    AcilirKatman::new("anchor-bottom-left")
                                        .trigger(Dugme::new("btn").outline().label("AltSol"))
                                        .anchor(Anchor::BottomLeft)
                                        .child("Bu alt solda açılan bir içeriktir."),
                                )
                                .child(
                                    AcilirKatman::new("anchor-bottom-center")
                                        .trigger(Dugme::new("btn").outline().label("AltOrta"))
                                        .anchor(Anchor::BottomCenter)
                                        .child("Bu alt ortada açılan bir içeriktir."),
                                )
                                .child(
                                    AcilirKatman::new("anchor-bottom-right")
                                        .anchor(Anchor::BottomRight)
                                        .trigger(Dugme::new("btn").outline().label("AltSağ"))
                                        .child("Bu alt sağda açılan bir içeriktir."),
                                ),
                        ),
                    ),
            )
    }
}
