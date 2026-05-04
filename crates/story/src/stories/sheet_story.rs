use std::{sync::Arc, time::Duration};

use fake::Fake;
use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement as _, IntoElement,
    ParentElement, Render, SharedString, Styled, Task, WeakEntity, Window, div,
    prelude::FluentBuilder as _, px,
};

use kavis_ui::{
    EtkinTema as _, IndexPath, PencereUzantisi, Placement, Simge, SimgeAdi,
    button::{Dugme, DugmeVaryanti, DugmeVaryantlari as _},
    checkbox::OnayKutusu,
    date_picker::{TarihSecici, TarihSeciciDurumu},
    h_flex,
    input::{Girdi, GirdiDurumu},
    list::{Liste, ListeDurumu, ListeOgesi, ListeTemsilcisi},
    v_flex,
};

use crate::TestAction;
use crate::{Story, section};

pub struct ListItemDeletegate {
    story: WeakEntity<SheetStory>,
    confirmed_index: Option<usize>,
    selected_index: Option<usize>,
    items: Vec<Arc<String>>,
    matches: Vec<Arc<String>>,
}

impl ListeTemsilcisi for ListItemDeletegate {
    type Item = ListeOgesi;

    fn items_count(&self, _: usize, _: &App) -> usize {
        self.matches.len()
    }

    fn perform_search(
        &mut self,
        query: &str,
        _: &mut Window,
        cx: &mut Context<ListeDurumu<Self>>,
    ) -> Task<()> {
        let query = query.to_string();
        cx.spawn(async move |this, cx| {
            // Simulate a slow search.
            let sleep = (0.05..0.1).fake();
            cx.background_executor()
                .timer(Duration::from_secs_f64(sleep))
                .await;

            this.update(cx, |this, cx| {
                this.delegate_mut().matches = this
                    .delegate()
                    .items
                    .iter()
                    .filter(|item| item.to_lowercase().contains(&query.to_lowercase()))
                    .cloned()
                    .collect();
                cx.notify();
            })
            .ok();
        })
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _: &mut Window,
        _: &mut Context<ListeDurumu<Self>>,
    ) -> Option<Self::Item> {
        let confirmed = Some(ix.row) == self.confirmed_index;

        if let Some(item) = self.matches.get(ix.row) {
            let list_item = ListeOgesi::new(("item", ix.row))
                .check_icon(SimgeAdi::Check)
                .confirmed(confirmed)
                .child(
                    h_flex()
                        .items_center()
                        .justify_between()
                        .child(item.to_string()),
                )
                .suffix(|_, _| {
                    Dugme::new("like")
                        .tab_stop(false)
                        .icon(SimgeAdi::Heart)
                        .with_variant(DugmeVaryanti::Ghost)
                        .size(px(18.))
                        .on_click(move |_, window, cx| {
                            cx.stop_propagation();
                            window.prevent_default();

                            println!("Beğen düğmesine tıkladınız.");
                        })
                });
            Some(list_item)
        } else {
            None
        }
    }

    fn render_empty(
        &mut self,
        _: &mut Window,
        cx: &mut Context<ListeDurumu<Self>>,
    ) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(
                Simge::new(SimgeAdi::Inbox)
                    .size(px(50.))
                    .text_color(cx.theme().muted_foreground),
            )
            .child("Eşleşme bulunamadı")
            .items_center()
            .justify_center()
            .p_3()
            .bg(cx.theme().muted)
            .text_color(cx.theme().muted_foreground)
    }

    fn cancel(&mut self, window: &mut Window, cx: &mut Context<ListeDurumu<Self>>) {
        _ = self.story.update(cx, |this, cx| {
            this.close_sheet(window, cx);
        });
    }

    fn confirm(&mut self, _secondary: bool, _: &mut Window, cx: &mut Context<ListeDurumu<Self>>) {
        _ = self.story.update(cx, |this, _| {
            self.confirmed_index = self.selected_index;
            if let Some(ix) = self.confirmed_index {
                if let Some(item) = self.matches.get(ix) {
                    this.selected_value = Some(SharedString::from(item.to_string()));
                }
            }
        });
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _: &mut Window,
        cx: &mut Context<ListeDurumu<Self>>,
    ) {
        self.selected_index = ix.map(|ix| ix.row);

        if let Some(_) = ix {
            cx.notify();
        }
    }
}

pub struct SheetStory {
    focus_handle: FocusHandle,
    placement: Option<Placement>,
    selected_value: Option<SharedString>,
    list: Entity<ListeDurumu<ListItemDeletegate>>,
    input1: Entity<GirdiDurumu>,
    input2: Entity<GirdiDurumu>,
    date: Entity<TarihSeciciDurumu>,
    overlay: bool,
    overlay_closable: bool,
}

impl Story for SheetStory {
    fn title() -> &'static str {
        "SayfaKatmani"
    }

    fn description() -> &'static str {
        "SayfaKatmani for open a popup in the edge of the window"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl SheetStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let items: Vec<Arc<String>> = [
            "Baguette (France)",
            "Baklava (Turkey)",
            "Beef Wellington (UK)",
            "Biryani (India)",
            "Borscht (Ukraine)",
            "Bratwurst (Germany)",
            "Bulgogi (Korea)",
            "Burrito (USA)",
            "Ceviche (Peru)",
            "Chicken Tikka Masala (India)",
            "Churrasco (Brazil)",
            "Couscous (North Africa)",
            "Croissant (France)",
            "Dim Sum (China)",
            "Empanada (Argentina)",
            "Fajitas (Mexico)",
            "Falafel (Middle East)",
            "Feijoada (Brazil)",
            "Fish and Chips (UK)",
            "Fondue (Switzerland)",
            "Goulash (Hungary)",
            "Haggis (Scotland)",
            "Kebab (Middle East)",
            "Kimchi (Korea)",
            "Lasagna (Italy)",
            "Maple Syrup Pancakes (Canada)",
            "Moussaka (Greece)",
            "Pad Thai (Thailand)",
            "Paella (Spain)",
            "Pancakes (USA)",
            "Pasta Carbonara (Italy)",
            "Pavlova (Australia)",
            "Peking Duck (China)",
            "Pho (Vietnam)",
            "Pierogi (Poland)",
            "Pizza (Italy)",
            "Poutine (Canada)",
            "Pretzel (Germany)",
            "Ramen (Japan)",
            "Rendang (Indonesia)",
            "Sashimi (Japan)",
            "Satay (Indonesia)",
            "Shepherd's Pie (Ireland)",
            "Sushi (Japan)",
            "Tacos (Mexico)",
            "Tandoori Chicken (India)",
            "Tortilla (Spain)",
            "Tzatziki (Greece)",
            "Wiener Schnitzel (Austria)",
        ]
        .iter()
        .map(|s| Arc::new(s.to_string()))
        .collect();

        let story = cx.entity().downgrade();
        let delegate = ListItemDeletegate {
            story,
            selected_index: None,
            confirmed_index: None,
            items: items.clone(),
            matches: items.clone(),
        };
        let list = cx.new(|cx| ListeDurumu::new(delegate, window, cx).searchable(true));
        let input1 = cx.new(|cx| GirdiDurumu::new(window, cx).placeholder("Adınız"));
        let input2 = cx.new(|cx| {
            GirdiDurumu::new(window, cx).placeholder("For test focus back on dialog close.")
        });
        let date = cx.new(|cx| TarihSeciciDurumu::new(window, cx));

        Self {
            focus_handle: cx.focus_handle(),
            placement: None,
            selected_value: None,
            list,
            input1,
            input2,
            date,
            overlay: true,
            overlay_closable: true,
        }
    }

    fn open_sheet_at(&mut self, placement: Placement, window: &mut Window, cx: &mut Context<Self>) {
        let list = self.list.clone();

        let drawer_h = match placement {
            Placement::Left | Placement::Right => px(400.),
            Placement::Top | Placement::Bottom => px(540.),
        };

        let overlay = self.overlay;
        let overlay_closable = self.overlay_closable;
        let input1 = self.input1.clone();
        let date = self.date.clone();
        window.open_sheet_at(placement, cx, move |this, _, cx| {
            this.overlay(overlay)
                .overlay_closable(overlay_closable)
                .size(drawer_h)
                .title("Kenar Paneli Başlığı")
                .child(
                    v_flex()
                        .size_full()
                        .gap_3()
                        .child(Girdi::new(&input1))
                        .child(TarihSecici::new(&date).placeholder("Doğum Tarihi"))
                        .child(
                            Dugme::new("send-notification")
                                .child("Test Bildirimi")
                                .on_click(|_, window, cx| {
                                    window.push_notification(
                                        "Merhaba, bu kenar panelinden gelen bir mesajdır.",
                                        cx,
                                    )
                                }),
                        )
                        .child(
                            Dugme::new("confirm-dialog-from-sheet")
                                .child("Onay İletişim Kutusunu Aç")
                                .on_click(|_, window, cx| {
                                    window.open_alert_dialog(cx, move |dialog, _, _| {
                                        dialog
                                            .child("Onay iletişim kutusu kenar panelinden açıldı.")
                                            .on_ok(|_, window, cx| {
                                                window.push_notification(
                                                    "Tamam düğmesine bastınız.",
                                                    cx,
                                                );
                                                true
                                            })
                                            .on_cancel(|_, window, cx| {
                                                window.push_notification(
                                                    "İptal düğmesine bastınız.",
                                                    cx,
                                                );
                                                true
                                            })
                                    });
                                }),
                        )
                        .child(
                            Liste::new(&list)
                                .border_1()
                                .border_color(cx.theme().border)
                                .rounded(cx.theme().radius),
                        ),
                )
                .footer(
                    h_flex()
                        .gap_6()
                        .items_center()
                        .child(Dugme::new("confirm").primary().label("Onayla").on_click(
                            |_, window, cx| {
                                window.close_sheet(cx);
                            },
                        ))
                        .child(
                            Dugme::new("cancel")
                                .label("İptal")
                                .on_click(|_, window, cx| {
                                    window.close_sheet(cx);
                                }),
                        ),
                )
        });
    }

    fn close_sheet(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.placement = None;
        cx.notify();
    }

    fn on_action_test_action(
        &mut self,
        _: &TestAction,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.push_notification("TestAction eylemine tıkladınız.", cx);
    }
}

impl Focusable for SheetStory {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SheetStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("sheet-story")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_action_test_action))
            .size_full()
            .child(
                v_flex()
                    .gap_6()
                    .child(
                        h_flex()
                            .id("state")
                            .items_center()
                            .gap_3()
                            .child(
                                OnayKutusu::new("overlay")
                                    .label("Kaplama")
                                    .checked(self.overlay)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.overlay = !view.overlay;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                OnayKutusu::new("closable")
                                    .label("Kaplama Kapatılabilir")
                                    .checked(self.overlay_closable)
                                    .on_click(cx.listener(|view, _, _, cx| {
                                        view.overlay_closable = !view.overlay_closable;
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        section("Normal SayfaKatmani")
                            .child(
                                Dugme::new("show-sheet-left")
                                    .outline()
                                    .label("Sol Kenar Paneli...")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_sheet_at(Placement::Left, window, cx)
                                    })),
                            )
                            .child(
                                Dugme::new("show-sheet-top")
                                    .outline()
                                    .label("Üst Kenar Paneli...")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_sheet_at(Placement::Top, window, cx)
                                    })),
                            )
                            .child(
                                Dugme::new("show-sheet-right")
                                    .outline()
                                    .label("Sağ Kenar Paneli...")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_sheet_at(Placement::Right, window, cx)
                                    })),
                            )
                            .child(
                                Dugme::new("show-sheet-bottom")
                                    .outline()
                                    .label("Alt Kenar Paneli...")
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.open_sheet_at(Placement::Bottom, window, cx)
                                    })),
                            ),
                    )
                    .child(
                        section("Kaydirilabilir SayfaKatmani").max_w_md().child(
                            Dugme::new("show-scrollable-sheet")
                                .outline()
                                .label("Kaydırılabilir Kenar Paneli...")
                                .on_click(cx.listener(|_, _, window, cx| {
                                    window.open_sheet_at(
                                        Placement::Right,
                                        cx,
                                        move |this, _, _| {
                                            this.title("Kaydırılabilir Kenar Paneli").child(
                                                "Bu kaydırılabilir bir kenar panelidir.\n"
                                                    .repeat(150),
                                            )
                                        },
                                    );
                                })),
                        ),
                    )
                    .child(
                        section("Focus back test")
                            .max_w_md()
                            .child(Girdi::new(&self.input2))
                            .child(
                                Dugme::new("test-action")
                                    .outline()
                                    .label("Test Eylemi")
                                    .flex_shrink_0()
                                    .on_click(|_, window, cx| {
                                        window.dispatch_action(Box::new(TestAction), cx);
                                    })
                                    .tooltip(
                                        "Bu düğme eylem göndermeyi test eder; \
                                        iletişim kutusu kapandığında bile\
                                        \neylemin işlenebildiğini doğrular.",
                                    ),
                            ),
                    )
                    .when_some(self.selected_value.clone(), |this, selected_value| {
                        this.child(
                            h_flex().gap_1().child("Seçtiniz:").child(
                                div()
                                    .child(selected_value.to_string())
                                    .text_color(gpui::red()),
                            ),
                        )
                    }),
            )
    }
}
